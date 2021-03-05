#include <time.h>
#include <errno.h>
//#include <sys/ioctl.h>
#include "lesstest.h"

extern int verbose;
extern int screen_width;
extern int screen_height;
extern char* testname;
extern int less_quit;
extern int details;
extern TermInfo terminfo;

int rstat_file = -1;

static int less_in = -1;
static int screen_out = -1;
static pid_t screen_pid = 0;

void sleep_ms(int ms) {
	#define NS_PER_MS (1000*1000)
	struct timespec tm;
	tm.tv_sec = ms / 1000;
	tm.tv_nsec = (ms % 1000) * NS_PER_MS;
	if (nanosleep(&tm, NULL) < 0)
		fprintf(stderr, "sleep error: %s\n", strerror(errno));
}

void send_char(wchar ch) {
	if (verbose) fprintf(stderr, "send %lx\n", ch);
	byte cbuf[UNICODE_MAX_BYTES];
	byte* cp = cbuf;
	store_wchar(&cp, ch);
	write(less_in, cbuf, cp-cbuf);
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
	sleep_ms(10);
}

int read_screen(byte* buf, int buflen) {
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
	byte rbuf[MAX_SCREENBUF_SIZE];
	int rn = read_screen(rbuf, sizeof(rbuf));
	if (rn == 0) return;
	printf("%s", terminfo.clear_screen);
	display_screen(rbuf, rn, 1);
	log_screen(rbuf, rn);
}

int curr_screen_match(const byte* img, int imglen) {
	byte curr[MAX_SCREENBUF_SIZE];
	int currlen = read_screen(curr, sizeof(curr));
	if (currlen == imglen && memcmp(img, curr, imglen) == 0)
		return 1;
	if (details) {
		fprintf(stderr, "MISMATCH: expect:\n");
		display_screen(img, imglen, 0);
		fprintf(stderr, "got:\n");
		display_screen(curr, currlen, 0);
	}
	return 0;
}

char* const* less_envp(const char* lesscharset) {
	static char lines[32];
	static char columns[32];
	static char charset[100];
	static char key_right[32];
	static char key_left[32];
	static char key_up[32];
	static char key_down[32];
	static char key_home[32];
	static char key_end[32];
	static char* envp[] = {
		lines,
		columns,
		charset,
		key_right,
		key_left,
		key_up,
		key_down,
		key_home,
		key_end,
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
	snprintf(charset, sizeof(charset), "LESSCHARSET=%s", lesscharset);
	snprintf(key_right, sizeof(key_right), "LESS_TERMCAP_kr=%s", terminfo.key_right ? terminfo.key_right : "");
	snprintf(key_left, sizeof(key_left), "LESS_TERMCAP_kl=%s", terminfo.key_left ? terminfo.key_left : "");
	snprintf(key_up, sizeof(key_up), "LESS_TERMCAP_ku=%s", terminfo.key_up ? terminfo.key_up : "");
	snprintf(key_down, sizeof(key_down), "LESS_TERMCAP_kd=%s", terminfo.key_down ? terminfo.key_down : "");
	snprintf(key_home, sizeof(key_home), "LESS_TERMCAP_kh=%s", terminfo.key_home ? terminfo.key_home : "");
	snprintf(key_end, sizeof(key_end), "LESS_TERMCAP_@7=%s", terminfo.key_end ? terminfo.key_end : "");
	return envp;
}

const char* get_envp(char* const* envp, const char* name) {
	int i;
	for (i = 0; envp[i] != NULL; ++i) {
		const char* eq = strchr(envp[i], '=');
		if (eq == NULL) continue;
		if (strncmp(name, envp[i], eq-envp[i]) == 0)
			return eq+1;
	}
	return NULL;
}

int run_interactive(char* const* argv, int argc) {
	const char* textfile = argv[argc-1];
	const char* slash = strrchr(textfile, '/');
	const char* temp = NULL;
	if (slash != NULL) {
		temp = slash+1;
		if (link(textfile, temp) < 0) {
			fprintf(stderr, "cannot link %s to %s: %s\n", textfile, temp, strerror(errno));
			return 0;
		}
	}
	const char* charset = getenv("LESSCHARSET");
	if (charset == NULL) charset = "";
	if (!create_less_pipeline(testname, argv, argc, 
			less_envp(charset), screen_width, screen_height, 1,
			&less_in, &screen_out, &screen_pid)) {
		if (temp != NULL) unlink(temp);
		return 0;
	}
	less_quit = 0;
	int tty = 0;
	raw_mode(tty, 1);
	read_and_display_screen();
	while (!less_quit) {
		wchar ch = read_wchar(tty);
		if (ch == terminfo.backspace_key)
			ch = '\b';
		if (verbose) fprintf(stderr, "tty %c (%lx)\n", pr_ascii(ch), ch);
		log_tty_char(ch);
		send_char(ch);
		read_and_display_screen();
	}
	raw_mode(tty, 0);
	if (temp != NULL) unlink(temp);
	return 1;
}

int run_test(TestSetup* setup, FILE* testfd) {
	fprintf(stderr, "RUN  %s\n", setup->setup_name);
	screen_width = setup->width;
	screen_height = setup->height;
	if (!create_less_pipeline(setup->setup_name, setup->argv, setup->argc, 
			less_envp(setup->charset), setup->width, setup->height, 0,
			&less_in, &screen_out, &screen_pid))
		return 0;
	less_quit = 0;
	wchar last_char = 0;
	int ok = 1;
	int cmds = 0;
	while (!less_quit) {
		char line[10000];
		int line_len = read_zline(testfd, line, sizeof(line));
		if (line_len < 0)
			break;
		if (line_len < 1)
			continue;
		switch (line[0]) {
		case '+':
			last_char = (wchar) strtol(line+1, NULL, 16);
			send_char(last_char);
			++cmds;
			break;
		case '=': 
			if (!curr_screen_match((byte*)line+1, line_len-1)) {
				ok = 0;
				fprintf(stderr, "FAIL %s on cmd #%d (%c %lx)\n",
					setup->setup_name, cmds, pr_ascii(last_char), last_char);
			}
			break;
		case '\n':
		case '!':
			break;
		default:
			fprintf(stderr, "unrecognized char at start of \"%s\"\n", line);
			return 0;
		}
	}
	fprintf(stderr, "END  %s (%d commands) %s\n", setup->setup_name, cmds, ok ? "OK" : "FAILED");
	return ok;
}

int run_testfile(const char* testfile, const char* less) {
	FILE* testfd = fopen(testfile, "r");
	if (testfd == NULL) {
		fprintf(stderr, "cannot open %s\n", testfile);
		return 0;
	}
	int tests = 0;
	int fails = 0;
	for (;;) {
		TestSetup* setup = read_test_setup(testfd, less);
		if (setup == NULL)
			break;
		++tests;
		int ok = run_test(setup, testfd);
		free_test_setup(setup);
		if (!ok) ++fails;
	}
	fprintf(stderr, "DONE %d test%s: %d ok", tests, tests>1?"s":"", tests-fails);
	if (fails > 0)
		fprintf(stderr, ", %d failed", fails);
	fprintf(stderr, "\n");
	fclose(testfd);
	return (fails == 0);
}
