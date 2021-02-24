#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <termios.h>
#include <termcap.h>
#include <time.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <sys/stat.h>
#include "lesstest.h"
#define RD 0
#define WR 1

static int less_in;
static int screen_out;
static int screen_width;
static int screen_height;
static pid_t screen_pid;
static FILE* logf = NULL;
static int verbose = 0;
static int run_less = 1;
static int child_died = 0;
static int screen_ready = 0;
static char backspace_key = '\b';
static char* enter_underline;
static char* exit_underline;
static char* enter_bold;
static char* exit_bold;
static char* enter_blink;
static char* exit_blink;
static char* enter_standout;
static char* exit_standout;
static char* clear_screen;
static char* cursor_move;

int usage() {
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
// ------------------------------------------------------------------

void display_attr(unsigned char attr) {
	static unsigned char prev_attr = 0;
	if (attr == prev_attr)
		return;
	if (prev_attr & ATTR_STANDOUT)
		printf("%s", exit_standout);
	if (prev_attr & ATTR_BLINK)
		printf("%s", exit_blink);
	if (prev_attr & ATTR_BOLD)
		printf("%s", exit_bold);
	if (prev_attr & ATTR_UNDERLINE)
		printf("%s", exit_underline);
	if (attr & ATTR_UNDERLINE)
		printf("%s", enter_underline);
	if (attr & ATTR_BOLD)
		printf("%s", enter_bold);
	if (attr & ATTR_BLINK)
		printf("%s", enter_blink);
	if (attr & ATTR_STANDOUT)
		printf("%s", enter_standout);
	prev_attr = attr;
}

void display_color(unsigned char fg_color, unsigned char bg_color) {
printf("{%x/%x}", fg_color, bg_color);
}

void display_screen(char const* img, int imglen) {
	printf("%s", clear_screen);
	int x = 0;
	int y = 0;
	int cursor_x = 0;
	int cursor_y = 0;
	while (imglen-- > 0) {
		char ch = *img++;
		if (ch == '\\') {
			ch = *img++;
		} else if (ch == '@') {
			char attr = *img++;
			display_attr(attr);
			continue;
		} else if (ch == '$') {
			char fg_color = *img++;
			char bg_color = *img++;
			display_color(fg_color, bg_color);
			continue;
		} else if (ch == '#') {
			cursor_x = x;
			cursor_y = y;
			continue;
		}
		//if (y == screen_height-1 && x == screen_width-3) break;
		printf("%c", ch);
		if (++x >= screen_width) {
			printf("\n");
			x = 0;
			if (++y >= screen_height)
				break;
		}
	}
	//FIXME
	printf("%s", tgoto(cursor_move, cursor_x, cursor_y));
}

void print_strings(char const* title, char* const* strings) {
	fprintf(stderr, "%s:\n", title);
	char* const* s;
	for (s = strings; *s != NULL; ++s) {
		fprintf(stderr, " ");
		char const* p;
		for (p = *s; *p != '\0'; ++p) {
			if (*p < 0x20 || *p >= 0x7f)
				fprintf(stderr, "\\x%02x", *p);
			else
				fprintf(stderr, "%c", *p);
		}
		fprintf(stderr, "\n");
	}
}

// ------------------------------------------------------------------

void log_char(char ch) {
	if (logf == NULL) return;
	fprintf(logf, "+%x\n", ch);
}

void log_screen(char const* img, int len) {
	if (logf == NULL) return;
	fwrite("=", 1, 1, logf);
	fwrite(img, 1, len, logf);
	fwrite("\n", 1, 1, logf);
}

int log_textfile(char const* textfile) {
	if (logf == NULL) return 0;
	struct stat st;
	if (stat(textfile, &st) < 0) {
		fprintf(stderr, "cannot stat %s\n", textfile);
		return 0;
	}
	FILE* fd = fopen(textfile, "r");
	if (fd == NULL) {
		fprintf(stderr, "cannot open %s\n", textfile);
		return 0;
	}
	fprintf(logf, "F %s %ld\n", textfile, (long) st.st_size);
	off_t nread = 0;
	while (nread < st.st_size) {
		char buf[4096];
		size_t n = fread(buf, 1, sizeof(buf), fd);
		if (n <= 0) {
			fprintf(stderr, "read only %ld/%ld from %s\n", (long) nread, (long) st.st_size, textfile);
			fclose(fd);
			return 0;
		}
		nread += n;
		fwrite(buf, 1, n, logf);
	}
	fclose(fd);
	return 1;
}

void send_char(char ch) {
	if (verbose) fprintf(stderr, "send %x\n", ch);
	write(less_in, &ch, 1);
}

void read_screen() {
	if (verbose) fprintf(stderr, "gen: read screen\n");
	kill(screen_pid, LTSIG_SCREEN_DUMP);
	char rbuf[8192];
	int rn = 0;
	for (; rn <= sizeof(rbuf); ++rn) {
		if (read(screen_out, &rbuf[rn], 1) != 1)
			break;
		if (rbuf[rn] == '\n')
			break;
	}
	display_screen(rbuf, rn);
	log_screen(rbuf, rn);
}

// ------------------------------------------------------------------

void raw_mode(int tty, int on)
{
	struct termios s;
	static struct termios save_term;
	if (!on) {
		s = save_term;
	} else {
		tcgetattr(tty, &s);
		save_term = s;
		s.c_lflag &= ~(ICANON | ECHO | ECHOE | ECHOK | ECHONL);
		s.c_oflag |= (TAB3 | OPOST | ONLCR);
		s.c_oflag &= ~(OCRNL | ONOCR | ONLRET);
		s.c_cc[VMIN] = 1;
		s.c_cc[VTIME] = 0;
	}
	tcsetattr(tty, TCSADRAIN, &s);
}

char* const* less_envp() {
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

int env_number(char const* s) {
	return (s == NULL) ? 0 : atoi(s);
}

int get_screen_size()
{
	screen_width = env_number("COLUMNS");
	screen_height = env_number("ROWS");
	if (screen_width == 0 || screen_height == 0) {
		struct winsize w;
		if (ioctl(2, TIOCGWINSZ, &w) == 0) {
			screen_height = w.ws_row;
			screen_width = w.ws_col;
		}
	}
	return (screen_width > 0 && screen_height > 0);
}

void dup_and_close(int fd0, int fd1, int close0, int close1) {
	if (close0 >= 0) close(close0);
	if (close1 >= 0) close(close1);
	if (fd0 >= 0) dup2(fd0, 0);
	if (fd1 >= 0) dup2(fd1, 1);
}

void child_handler(int signum) {
	if (verbose) fprintf(stderr, "child died\n");
	child_died = 1;
}

void screen_ready_handler(int signum) {
	// (signum == LTSIG_SCREEN_READY)
	if (verbose) fprintf(stderr, "screen signals ready\n");
	screen_ready = 1;
}

void setup_mode(char* enter_cap, char* exit_cap, char** enter_str, char** exit_str, char** spp) {
	*enter_str = tgetstr(enter_cap, spp);
	if (*enter_str == NULL) *enter_str = "";
	*exit_str = tgetstr(exit_cap, spp);
	if (*exit_str == NULL) *exit_str = tgetstr("me", spp);
	if (*exit_str == NULL) *exit_str = "";
}

int setup_term() {
	static char termbuf[4096];
	static char sbuf[4096];
	char* term = getenv("TERM");
	if (term == NULL) term = "dumb";
	if (tgetent(termbuf, term) <= 0) {
		fprintf(stderr, "cannot setup terminal %s\n", term);
		return 0;
	}
	char* sp = sbuf;
	setup_mode("so", "se", &enter_standout, &exit_standout, &sp);
	setup_mode("us", "ue", &enter_underline, &exit_underline, &sp);
	setup_mode("md", "me", &enter_bold, &exit_bold, &sp);
	setup_mode("mb", "me", &enter_blink, &exit_blink, &sp);
	cursor_move = tgetstr("cm", &sp);
	clear_screen = tgetstr("cl", &sp);
	char* bs = tgetstr("kb", &sp);
	if (bs != NULL && strlen(bs) == 1)
		backspace_key = *bs;
	return 1;
}

int setup(int argc, char* const* argv) {
	char* logfile = "-";
	char* lt_screen = "lt_screen";
	if (!get_screen_size()) {
		fprintf(stderr, "cannot get screen size\n");
		return 0;
	}
	int ch;
	while ((ch = getopt(argc, argv, "h:o:s:St:vw:")) != -1) {
		switch (ch) {
		case 'h':
			screen_height = atoi(optarg);
			break;
		case 'o':
			logfile = optarg;
			break;
		case 's':
			lt_screen = optarg;
			break;
		case 'v':
			verbose = 1;
			break;
		case 'w':
			screen_width = atoi(optarg);
			break;
		case 'S':
			run_less = 0;
			break;
		default:
			return usage();
		}
	}
	logf = (strcmp(logfile, "-") == 0) ? stdout : fopen(logfile, "w");
	if (logf == NULL) {
		fprintf(stderr, "cannot create %s\n", logfile);
		return 0;
	}
	fprintf(logf, "!lesstest!\n");


	int screen_in_pipe[2];
	if (pipe(screen_in_pipe) < 0)
		return 0;
	if (verbose) fprintf(stderr, "less out pipe %d,%d\n", screen_in_pipe[0], screen_in_pipe[1]);
	int less_in_pipe[2];
	if (run_less) {
		if (pipe(less_in_pipe) < 0)
			return 0;
		if (verbose) fprintf(stderr, "less in pipe %d,%d\n", less_in_pipe[RD], less_in_pipe[WR]);
		if (optind+2 > argc)
			return usage();
		char* less = argv[optind++];
		char* textfile = argv[argc-1];
		if (verbose) fprintf(stderr, "testing %s on %s\n", less, textfile);
		if (!log_textfile(textfile))
			return 0;
		char* const* lessenvp = less_envp();
		pid_t pid = fork();
		if (pid < 0)
			return 0;
		if (!pid) { // child: less
			char** less_argv = malloc(sizeof(char*) * argc-optind+3);
			int less_argc = 0;
			less_argv[less_argc++] = less;
			less_argv[less_argc++] = "--tty";
			less_argv[less_argc++] = "/dev/stdin";
			while (argv[optind] != NULL)
				less_argv[less_argc++] = argv[optind++];
			less_argv[less_argc] = NULL;
			if (verbose) fprintf(stderr, "less child: in %d, out %d, close %d,%d\n", less_in_pipe[RD], screen_in_pipe[WR], less_in_pipe[WR], screen_in_pipe[RD]);
			dup_and_close(less_in_pipe[RD], screen_in_pipe[WR],
			              less_in_pipe[WR], screen_in_pipe[RD]);
			if (verbose) print_strings("less argv", less_argv);
			if (verbose) print_strings("less envp", lessenvp);
			execve(less, less_argv, lessenvp);
			fprintf(stderr, "cannot exec %s\n", less);
			exit(1);
		}
		if (verbose) fprintf(stderr, "less child %ld\n", (long)pid);
		close(less_in_pipe[RD]);
		close(screen_in_pipe[WR]);
	}
	int screen_out_pipe[2];
	if (pipe(screen_out_pipe) < 0)
		return 0;
	if (verbose) fprintf(stderr, "screen out pipe %d,%d\n", screen_out_pipe[RD], screen_out_pipe[WR]);
	screen_pid = fork();
	pid_t gen_pid = getpid();
	if (!screen_pid) { // child: lt_screen
		if (verbose) fprintf(stderr, "screen child: in %d, out %d, close %d\n", screen_in_pipe[RD], screen_out_pipe[WR], screen_out_pipe[RD]);
		dup_and_close(screen_in_pipe[RD], screen_out_pipe[WR], screen_out_pipe[RD], -1);
		char sw[16];
		char sh[16];
		char gen_pid_str[32];
		snprintf(sw, sizeof(sw), "%d", screen_width);
		snprintf(sh, sizeof(sh), "%d", screen_height);
		snprintf(gen_pid_str, sizeof(gen_pid_str), "%lld", (long long) gen_pid);
		char* lts_argv[] = { lt_screen, "-w", sw, "-h", sh, /*"-r", gen_pid_str,*/ "-q", NULL, NULL };
		if (verbose) lts_argv[6] = "-v";
		char* const lts_envp[] = { NULL };
		if (verbose) print_strings("screen argv", lts_argv);
		execve(lt_screen, lts_argv, lts_envp);
		fprintf(stderr, "cannot exec %s\n", lt_screen);
		exit(1);
	}
	if (verbose) fprintf(stderr, "screen child %ld\n", (long)screen_pid);
	close(screen_out_pipe[WR]);
	close(screen_in_pipe[RD]);
	less_in = run_less ? less_in_pipe[WR] : screen_in_pipe[WR];
	screen_out = screen_out_pipe[RD];
	if (verbose) fprintf(stderr, "less in %d, screen out %d\n", less_in, screen_out);
	return 1;
}

int main(int argc, char* const* argv) {
	signal(SIGCHLD, child_handler);
	signal(LTSIG_SCREEN_READY, screen_ready_handler);
	if (!setup(argc, argv))
		return 1;
	setup_term();
	int tty = 0;
	raw_mode(tty, 1);
//	while (!screen_ready)
		sleep(1);
//fprintf(stderr, "sleep(2000)\n"); fflush(stderr);
//sleep_ms(5000);
//fprintf(stderr, "sleep done\n"); fflush(stderr);
	read_screen();
	for (;;) {
		if (child_died)
			break;
		char ch;
		int n = read(tty, &ch, 1);
		if (n <= 0)
			break;
		if (ch == backspace_key)
			ch = '\b';
		if (verbose) fprintf(stderr, "tty %c (%02x)\n", ch >= ' ' && ch < 0x7f ? ch : '.', ch);
		log_char(ch);
		send_char(ch);
		sleep_ms(100);
		read_screen();
	}
	raw_mode(tty, 0);
	if (logf != stdout)
		fclose(logf);
	return 0;
}
