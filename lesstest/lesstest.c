#include <time.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include "lesstest.h"

extern TermInfo terminfo;

int verbose = 0;
int screen_width;
int screen_height;
int rstat_file;

static char* testfile = NULL;
static int less_in;
static int screen_out;
static pid_t screen_pid = -1;
///static int child_died = 0;
static int screen_ready = 0;
static char* testname = NULL;
static int less_quit = 0;

int usage(void) {
	fprintf(stderr, "usage: lt_gen\n");
	return 0;
}

void sleep_ms(int ms) {
	#define NS_PER_MS (1000*1000)
	struct timespec tm;
	tm.tv_sec = ms / 1000;
	tm.tv_nsec = (ms % 1000) * NS_PER_MS;
	if (nanosleep(&tm, NULL) < 0)
		fprintf(stderr, "sleep err %d\n", errno);
}

void send_char(char ch) {
	if (verbose) fprintf(stderr, "send %x\n", ch);
	write(less_in, &ch, 1);
}

void wait_less_ready(void) {
	if (rstat_file < 0) return;
	for (;;) {
		lseek(rstat_file, SEEK_SET, 0);
		char st;
		if (read(rstat_file, &st, 1) == 1) {
			if (st == 'R')
				break;
			if (st == 'Q') {
				less_quit = 1;
				fprintf(stderr, "less quit\n");
				break;
			}
		}
		sleep_ms(1);
	}
	sleep_ms(1);
}

int read_screen(char* buf, int buflen) {
	if (verbose) fprintf(stderr, "gen: read screen\n");
	wait_less_ready();
	if (less_quit)
		return 0;
	kill(screen_pid, LTSIG_SCREEN_DUMP);
	int rn = 0;
	for (; rn <= buflen; ++rn) {
		if (read(screen_out, &buf[rn], 1) != 1)
			break;
		if (buf[rn] == '\n')
			break;
	}
	return rn;
}

void read_and_display_screen(void) {
	char rbuf[8192];
	int rn = read_screen(rbuf, sizeof(rbuf));
	if (rn == 0) return;
	printf("%s", terminfo.clear_screen);
	display_screen(rbuf, rn, 1);
	log_screen(rbuf, rn);
}

int curr_screen_match(char const* img, int imglen) {
	char curr[8192];
	int currlen = read_screen(curr, sizeof(curr));
	if (currlen == imglen && memcmp(img, curr, imglen) == 0)
		return 1;
	fprintf(stderr, "MISMATCH: expect:\n");
	display_screen(img, imglen, 0);
	fprintf(stderr, "got:\n");
	display_screen(curr, currlen, 0);
	return 0;
}

// ------------------------------------------------------------------

char* const* less_envp(void) {
	static char lines[32];
	static char columns[32];
	static char* envp[] = {
		lines,
		columns,
		"LESS_TERMCAP_am=1",
		"LESS_TERMCAP_cd=\33S",
		"LESS_TERMCAP_ce=\33L",
		"LESS_TERMCAP_cl=\33A",
		"LESS_TERMCAP_cr=\33<",
		"LESS_TERMCAP_cm=\33%p2%d;%p1%dj",
		"LESS_TERMCAP_ho=\33h",
		"LESS_TERMCAP_ll=\33l",
		"LESS_TERMCAP_mb=\33b",
		"LESS_TERMCAP_md=\33d",
		"LESS_TERMCAP_md=\33e",
		"LESS_TERMCAP_se=\33t",
		"LESS_TERMCAP_so=\33s",
		"LESS_TERMCAP_sr=\33r",
		"LESS_TERMCAP_ue=\33v",
		"LESS_TERMCAP_uo=\33u",
		"LESS_TERMCAP_vb=\33g",
		NULL,
	};
	snprintf(lines, sizeof(lines), "LINES=%d", screen_height);
	snprintf(columns, sizeof(columns), "COLUMNS=%d", screen_width);
	return envp;
}

void child_handler(int signum) {
	int status;
	pid_t child = wait(&status);
	if (verbose) fprintf(stderr, "child %d died, status 0x%x\n", child, status);
	///child_died = 1;
}

void screen_ready_handler(int signum) {
	// (signum == LTSIG_SCREEN_READY)
	if (verbose) fprintf(stderr, "screen signals ready\n");
	screen_ready = 1;
}

int setup(int argc, char* const* argv) {
	char* logfile = NULL;
	if (!get_screen_size()) {
		fprintf(stderr, "cannot get screen size\n");
		return 0;
	}
	int ch;
	while ((ch = getopt(argc, argv, "h:n:o:t:vw:")) != -1) {
		switch (ch) {
		case 'h':
			screen_height = atoi(optarg);
			break;
		case 'n':
			testname = optarg;
			break;
		case 'o':
			logfile = optarg;
			break;
		case 't':
			testfile = optarg;
			break;
		case 'v':
			verbose = 1;
			break;
		case 'w':
			screen_width = atoi(optarg);
			break;
		default:
			return usage();
		}
	}
	if (logfile != NULL) {
		log_open(logfile);
		log_header();
	}
	return 1;
}

int run_interactive(char* const* argv, int argc) {
	if (create_less_pipeline(testname, argv, argc, less_envp(), 
			screen_width, screen_height, &less_in, &screen_out, &screen_pid))
		return 0;
	less_quit = 0;
	int tty = 0;
	raw_mode(tty, 1);
	read_and_display_screen();
	while (!less_quit) {
		/// if (child_died) break;
		char ch;
		int n = read(tty, &ch, 1);
		if (n <= 0)
			break;
		if (ch == terminfo.backspace_key)
			ch = '\b';
		if (verbose) fprintf(stderr, "tty %c (%02x)\n", ch >= ' ' && ch < 0x7f ? ch : '.', ch);
		log_tty_char(ch);
		send_char(ch);
		read_and_display_screen();
	}
	raw_mode(tty, 0);
	log_close();
	return 1;
}

int run_test(TestSetup* setup, FILE* fd) {
	fprintf(stderr, "RUN %s\n", setup->setup_name);
	screen_width = setup->width;
	screen_height = setup->height;
	if (!create_less_pipeline(setup->setup_name, setup->argv, setup->argc, less_envp(), 
			setup->width, setup->height, &less_in, &screen_out, &screen_pid))
		return 0;
	less_quit = 0;
	char last_char = 0;
	int ok = 1;
	while (!less_quit) {
		char line[10000];
		int line_len = read_zline(fd, line, sizeof(line));
		if (line_len < 0)
			break;
		if (line_len < 1)
			continue;
		switch (line[0]) {
		case '+':
			last_char = (char) strtol(line+1, NULL, 16);
			send_char(last_char);
			break;
		case '=': 
			if (curr_screen_match(line+1, line_len-1)) {
				fprintf(stderr, ".");
			} else {
				ok = 0;
				fprintf(stderr, "FAIL %s on ", setup->setup_name);
				if (last_char >= ' ' && last_char < 0x7f)
					fprintf(stderr, "%c", last_char);
				else
					fprintf(stderr, "0x%02x", last_char);
			}
			fflush(stderr);
			break;
		case '\n':
		case '!':
			break;
		default:
			fprintf(stderr, "unrecognized char at start of \"%s\"\n", line);
			return 0;
		}
	}
	fprintf(stderr, "\nEND %s\n", setup->setup_name);
	return ok;
}

int run_testfile(void) {
	FILE* fd = fopen(testfile, "r");
	if (fd == NULL) {
		fprintf(stderr, "cannot open %s\n", testfile);
		return 0;
	}
	int ok = 1;
	int fails = 0;
	int tests = 0;
	for (;;) {
		TestSetup* setup = read_test_setup(fd);
		if (setup == NULL)
			break;
		++tests;
		ok = run_test(setup, fd);
		free_test_setup(setup);
		if (!ok) ++fails;
	}
	fprintf(stderr, "%d tests, %d failed\n", tests, fails);
	fclose(fd);
	return ok;
}

int main(int argc, char* const* argv) {
	signal(SIGCHLD, child_handler);
	signal(LTSIG_SCREEN_READY, screen_ready_handler);
	if (!setup(argc, argv))
		return 1;
	setup_term();
	int ok = 0;
	if (testfile != NULL) {
		ok = run_testfile();
	} else {
		if (optind+2 > argc)
			return usage();
		ok = run_interactive(argv+optind, argc-optind);
	}
	return !ok;
}
