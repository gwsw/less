#include <fcntl.h>
#include <termios.h>
#include <termcap.h>
#include <time.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include "lesstest.h"

int verbose = 0;
static char* testfile = NULL;
static int less_in;
static int screen_out;
static int screen_width;
static int screen_height;
static pid_t screen_pid = -1;
static int run_less = 1;
///static int child_died = 0;
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
static char* testdir = ".";
static char* testname = NULL;

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

void send_char(char ch) {
	if (verbose) fprintf(stderr, "send %x\n", ch);
	write(less_in, &ch, 1);
}

int read_screen(char* buf, int buflen) {
	if (verbose) fprintf(stderr, "gen: read screen\n");
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
	printf("%s", clear_screen);
	display_screen(rbuf, rn);
	log_screen(rbuf, rn);
}

int curr_screen_match(char const* img, int imglen) {
	char curr[8192];
	int currlen = read_screen(curr, sizeof(curr));
	if (currlen == imglen && memcmp(img, curr, imglen) == 0)
		return 1;
	fprintf(stderr, "MISMATCH: expect:\n");
	display_screen(img, imglen);
	fprintf(stderr, "got:\n");
	display_screen(curr, currlen);
	return 0;
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

int env_number(char const* s) {
	return (s == NULL) ? 0 : atoi(s);
}

int get_screen_size(void)
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

void setup_mode(char* enter_cap, char* exit_cap, char** enter_str, char** exit_str, char** spp) {
	*enter_str = tgetstr(enter_cap, spp);
	if (*enter_str == NULL) *enter_str = "";
	*exit_str = tgetstr(exit_cap, spp);
	if (*exit_str == NULL) *exit_str = tgetstr("me", spp);
	if (*exit_str == NULL) *exit_str = "";
}

int setup_term(void) {
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
	if (cursor_move == NULL) cursor_move = "";
	clear_screen = tgetstr("cl", &sp);
	if (clear_screen == NULL) clear_screen = "";
	char* bs = tgetstr("kb", &sp);
	if (bs != NULL && strlen(bs) == 1)
		backspace_key = *bs;
	return 1;
}

int setup(int argc, char* const* argv) {
	char* logfile = NULL;
	if (!get_screen_size()) {
		fprintf(stderr, "cannot get screen size\n");
		return 0;
	}
	int ch;
	while ((ch = getopt(argc, argv, "d:h:n:o:St:vw:")) != -1) {
		switch (ch) {
		case 'd':
			testdir = optarg;
			break;
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
		case 'S':
			run_less = 0;
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
	if (!create_less_pipeline(testname, argv, argc, less_envp(), testdir,
			screen_width, screen_height, &less_in, &screen_out, &screen_pid))
		return 0;
	setup_term();
	int tty = 0;
	raw_mode(tty, 1);
//	while (!screen_ready)
//		sleep(1);
	read_and_display_screen();
	for (;;) {
		/// if (child_died) break;
		char ch;
		int n = read(tty, &ch, 1);
		if (n <= 0)
			break;
		if (ch == backspace_key)
			ch = '\b';
		if (verbose) fprintf(stderr, "tty %c (%02x)\n", ch >= ' ' && ch < 0x7f ? ch : '.', ch);
		log_tty_char(ch);
		send_char(ch);
		sleep_ms(100); // FIXME
		read_and_display_screen();
	}
	raw_mode(tty, 0);
	log_close();
	return 1;
}

int read_zline(FILE* fd, char* line, int line_len) {
	int nread = 0;
	while (nread < line_len) {
		int ch = fgetc(fd);
		if (ch == EOF) return -1;
		if (ch == '\n') break;
		line[nread++] = (char) ch;
	}
	return nread;
}

void free_test_setup(TestSetup* setup) {
	unlink(setup->textfile);
	free(setup->setup_name);
	free(setup->textfile);
	int i;
	for (i = 0; i < setup->argc; ++i)
		free(setup->argv[i]);
	free((void*)setup->argv);
	free(setup);
}

TestSetup* read_test_setup(FILE* fd) {
	TestSetup* setup = (TestSetup*) malloc(sizeof(TestSetup));
	setup->setup_name = NULL;
	setup->textfile = NULL;
	setup->argv = NULL;
	setup->argc = 0;
	setup->width = setup->height = 0;
	int hdr_complete = 0;
	while (!hdr_complete) {
		char line[10000];
		int line_len = read_zline(fd, line, sizeof(line));
		if (line_len < 0)
			break;
		if (line_len < 1)
			continue;
		switch (line[0]) {
		case ']':
			hdr_complete = 1;
			break;
		case '[':
			if (!parse_setup_name(setup, line+1, line_len-1)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case 'L':
			if (!parse_command(setup, line+1, line_len-1)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case 'F':
			if (!parse_textfile(setup, line+1, line_len-1, testdir, fd)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case '!':
			break;
		default:
			break;
		}
	}
	if (setup->textfile == NULL || setup->argv == NULL || setup->width < 1 || setup->height < 1) {
		free_test_setup(setup);
		return NULL;
	}
	//free(setup->argv[setup->argc-1]);
	//setup->argv[setup->argc-1] = strdup(setup->textfile);
	if (verbose) { fprintf(stderr, "setup: [%s] textfile %s, %dx%d\n", setup->setup_name, setup->textfile, setup->width, setup->height); print_strings("argv:", setup->argv); }
	return setup;
}

int run_test(TestSetup* setup, FILE* fd) {
	fprintf(stderr, "run %s\n", setup->setup_name);
	screen_width = setup->width;
	screen_height = setup->height;
	if (!create_less_pipeline(setup->setup_name, setup->argv, setup->argc, less_envp(), testdir,
			setup->width, setup->height, &less_in, &screen_out, &screen_pid))
		return 0;
	for (;;) {
		char line[10000];
		int line_len = read_zline(fd, line, sizeof(line));
		if (line_len < 0)
			break;
		if (line_len < 1)
			continue;
		switch (line[0]) {
		case '+':
			send_char((char) strtol(line+1, NULL, 16));
sleep_ms(500);
			break;
		case '=': 
			if (curr_screen_match(line+1, line_len-1)) {
				fprintf(stderr, "OK   %s\n", setup->setup_name);
			} else {
				fprintf(stderr, "FAIL %s\n", setup->setup_name);
			}
			break;
		case '\n':
		case '!':
			break;
		default:
			return 1;
		}
	}
	return 1;
}

int run_testfile(void) {
	FILE* fd = fopen(testfile, "r");
	if (fd == NULL) {
		fprintf(stderr, "cannot open %s\n", testfile);
		return 0;
	}
	int ok = 1;
	for (;;) {
		TestSetup* setup = read_test_setup(fd);
		if (setup == NULL)
			break;
		ok = run_test(setup, fd);
		free_test_setup(setup);
		if (!ok) break;
	}
	fclose(fd);
	return ok;
}

int main(int argc, char* const* argv) {
	signal(SIGCHLD, child_handler);
	signal(LTSIG_SCREEN_READY, screen_ready_handler);
	if (!setup(argc, argv))
		return 1;
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
