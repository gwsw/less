#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <fcntl.h>
#include <termios.h>
#include <termcap.h>
#include <time.h>
#include <errno.h>
#include <sys/ioctl.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include "lesstest.h"

typedef struct TestSetup {
	char* setup_name;
	char* textfile;
	char** argv;
	int argc;
	FILE* testfile;
} TestSetup;

int verbose = 0;
static char* testfile = NULL;
static int less_in;
static int screen_out;
static int screen_width;
static int screen_height;
static pid_t screen_pid = -1;
static FILE* logf = NULL;
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
static char* testdir = ".";

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

int log_header(void) {
	fprintf(logf, "!lesstest!\n");
	return 1;
}
int log_test_header(char const* testname) {
	fprintf(logf, "[ \"%s\"\n", testname);
	return 1;
}

int log_test_footer(void) {
	fprintf(logf, "]\n");
	return 1;
}

int log_tty_char(char ch) {
	fprintf(logf, "+%x\n", ch);
	return 1;
}

int log_screen(char const* img, int len) {
	fwrite("=", 1, 1, logf);
	fwrite(img, 1, len, logf);
	fwrite("\n", 1, 1, logf);
	return 1;
}

int log_command(char* const* argv, int argc) {
	fprintf(logf, "L");
	int a;
	for (a = 0; a < argc; ++a)
		fprintf(logf, " \"%s\"", argv[a]);
	fprintf(logf, "\n");
	return 1;
}

int log_textfile(char const* textfile) {
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
	char const* basename = rindex(textfile, '/');
	if (basename == NULL) basename = textfile; else ++basename;
	fprintf(logf, "F \"%s\" %ld\n", basename, (long) st.st_size);
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

void read_screen(void) {
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
	char* testname = NULL;
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
	if (optind+2 > argc)
		return usage();
	if (logfile != NULL) {
		logf = (strcmp(logfile, "-") == 0) ? stdout : fopen(logfile, "w");
		if (logf == NULL) {
			fprintf(stderr, "cannot create %s\n", logfile);
			return 0;
		}
		log_header();
	}
	if (!create_less_pipeline(testname, argv+optind, argc-optind, less_envp(), 
			screen_width, screen_height, &less_in, &screen_out, &screen_pid))
		return 0;
	return 1;
}

int run_interactive(void) {
	setup_term();
	int tty = 0;
	raw_mode(tty, 1);
//	while (!screen_ready)
		sleep(1);
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
		log_tty_char(ch);
		send_char(ch);
		sleep_ms(100); // FIXME
		read_screen();
	}
	raw_mode(tty, 0);
	if (logf != stdout)
		fclose(logf);
	return 1;
}

char* parse_qstring(const char* * s) {
	while (*(*s) == ' ') ++(*s);
	if (*(*s)++ != '"') return NULL;
	char const* start = *s;
	while (*(*s) != '"' && *(*s) != '\0') ++(*s);
	char* ret = strndup(start, (*s)-start);
	if (*(*s) == '"') ++(*s);
	return ret;
}

int parse_setup_name(TestSetup* setup, char const* line) {
	setup->setup_name = parse_qstring(&line);
	return 1;
}

int parse_command(TestSetup* setup, char const* line) {
	setup->argv = (char**) malloc(32*sizeof(char const*));
	setup->argc = 0;
	for (;;) {
		char const* arg = parse_qstring(&line);
		setup->argv[setup->argc++] = (char*) arg;
		if (arg == NULL) break;
	}
	return 1;
}

int parse_textfile(TestSetup* setup, char const* line, FILE* fd) {
	char const* filename = parse_qstring(&line);
	char const* fsize_str = parse_qstring(&line);
	int fsize = atoi(fsize_str);
	int len = strlen(testdir)+strlen(filename)+10;
	setup->textfile = malloc(len);
	snprintf(setup->textfile, len, "%s/%06d-%s", testdir, rand() % 1000000, filename);
	FILE* textfd = fopen(setup->textfile, "w");
	if (textfd == NULL) {
		fprintf(stderr, "cannot create %s\n", setup->textfile);
		return 0;
	}
	int nread = 0;
	while (nread < fsize) {
		char buf[4096];
		int chunk = fsize - nread;
		if (chunk > sizeof(buf)) chunk = sizeof(buf);
		size_t nread = fread(buf, 1, chunk, fd);
		fwrite(buf, 1, nread, textfd);
	}
	return 1;
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
	char line[10000];
	while (fgets(line, sizeof(line), fd) != NULL) {
		switch (line[0]) {
		case '\0':
		case '\n':
			break;
		case '!':
			break;
		case ']':
			return setup;
		case '[':
			if (!parse_setup_name(setup, line+1)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case 'L':
			if (!parse_command(setup, line+1)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case 'F':
			if (!parse_textfile(setup, line+1, fd)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		}
	}
	free(setup->argv[setup->argc-1]);
	setup->argv[setup->argc-1] = strdup(setup->textfile);
	return setup;
}

int run_test(TestSetup* setup) {
	fprintf(stderr, "run %s\n", setup->setup_name);
	return 0;
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
		ok = run_test(setup);
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
	int ok = (testfile == NULL) ? run_interactive() : run_testfile();
	return !ok;
}
