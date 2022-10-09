#include <time.h>
#include <errno.h>
#include <setjmp.h>
#include <errno.h>
#include <signal.h>
#include <sys/wait.h>
#include "lesstest.h"

extern int verbose;
extern int less_quit;
extern int details;
extern TermInfo terminfo;

static pid_t less_pid;
static jmp_buf run_catch;

static void set_signal(int signum, void (*handler)(int)) {
	struct sigaction sa;
	sa.sa_handler = handler;
	sa.sa_flags = 0;
	sigemptyset(&sa.sa_mask);
	sigaction(signum, &sa, NULL);
}

static void child_handler(int signum) {
	int status;
	pid_t child = wait(&status);
	if (verbose) fprintf(stderr, "child %d died, status 0x%x\n", child, status);
	if (child == less_pid) {
		if (verbose) fprintf(stderr, "less died\n");
		less_quit = 1;
	}
}

static void set_intr_handler(int set) {
	set_signal(SIGINT,  set ? SIG_IGN : SIG_DFL);
	set_signal(SIGQUIT, set ? SIG_IGN : SIG_DFL);
	set_signal(SIGKILL, set ? SIG_IGN : SIG_DFL);
	set_signal(SIGPIPE, set ? SIG_IGN : SIG_DFL);
	set_signal(SIGCHLD, set ? child_handler : SIG_DFL);
}

static void send_char(LessPipeline* pipeline, wchar ch) {
	if (verbose) fprintf(stderr, "send %lx\n", ch);
	byte cbuf[UNICODE_MAX_BYTES];
	byte* cp = cbuf;
	store_wchar(&cp, ch);
	write(pipeline->less_in, cbuf, cp-cbuf);
}

static int read_screen(LessPipeline* pipeline, byte* buf, int buflen) {
	if (verbose) fprintf(stderr, "gen: read screen\n");
	send_char(pipeline, LESS_DUMP_CHAR);
	int rn = 0;
	for (; rn <= buflen; ++rn) {
		if (read(pipeline->screen_out, &buf[rn], 1) != 1)
			break;
		if (buf[rn] == '\n')
			break;
	}
	return rn;
}

static void read_and_display_screen(LessPipeline* pipeline) {
	byte rbuf[MAX_SCREENBUF_SIZE];
	int rn = read_screen(pipeline, rbuf, sizeof(rbuf));
	if (rn == 0) return;
	printf("%s", terminfo.clear_screen);
	display_screen(rbuf, rn, pipeline->screen_width, pipeline->screen_height, 1);
	log_screen(rbuf, rn);
}

static int curr_screen_match(LessPipeline* pipeline, const byte* img, int imglen) {
	byte curr[MAX_SCREENBUF_SIZE];
	int currlen = read_screen(pipeline, curr, sizeof(curr));
	if (currlen == imglen && memcmp(img, curr, imglen) == 0)
		return 1;
	if (details) {
		fprintf(stderr, "MISMATCH: expect:\n");
		display_screen(img, imglen, pipeline->screen_width, pipeline->screen_height, 0);
		fprintf(stderr, "got:\n");
		display_screen(curr, currlen, pipeline->screen_width, pipeline->screen_height, 0);
	}
	return 0;
}

int run_interactive(char* const* argv, int argc, char* const* prog_envp) {
	setup_term();
	char* const* envp = less_envp(prog_envp, 1);
	LessPipeline* pipeline = create_less_pipeline(argv, argc, envp);
	if (pipeline == NULL)
		return 0;
	less_pid = pipeline->less_pid;
	const char* textfile = (pipeline->tempfile != NULL) ? pipeline->tempfile : argv[argc-1];
	if (!log_test_header(argv, argc, textfile)) {
		destroy_less_pipeline(pipeline);
		return 0;
	}
	set_intr_handler(1);
	less_quit = 0;
	int ttyin = 0; // stdin
	raw_mode(ttyin, 1);
	printf("%s%s", terminfo.init_term, terminfo.enter_keypad);
	read_and_display_screen(pipeline);
	while (!less_quit) {
		wchar ch = read_wchar(ttyin);
		if (ch == terminfo.backspace_key)
			ch = '\b';
		if (verbose) fprintf(stderr, "tty %c (%lx)\n", pr_ascii(ch), ch);
		log_tty_char(ch);
		send_char(pipeline, ch);
		read_and_display_screen(pipeline);
	}
	log_test_footer();
	printf("%s%s", terminfo.exit_keypad, terminfo.deinit_term);
	raw_mode(ttyin, 0);
	destroy_less_pipeline(pipeline);
	set_intr_handler(0);
	return 1;
}

int run_test(TestSetup* setup, FILE* testfd) {
	const char* setup_name = setup->argv[setup->argc-1];
	fprintf(stderr, "RUN  %s\n", setup_name);
	LessPipeline* pipeline = create_less_pipeline(setup->argv, setup->argc, 
			less_envp(setup->env.env_list, 0));
	if (pipeline == NULL)
		return 0;
	less_quit = 0;
	wchar last_char = 0;
	int ok = 1;
	int cmds = 0;
	if (setjmp(run_catch)) {
		fprintf(stderr, "\nINTR test interrupted\n");
		ok = 0;
	} else {
		set_intr_handler(1);
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
				send_char(pipeline, last_char);
				++cmds;
				break;
			case '=': 
				if (!curr_screen_match(pipeline, (byte*)line+1, line_len-1)) {
					ok = 0;
					fprintf(stderr, "FAIL %s on cmd #%d (%c %lx)\n",
						setup_name, cmds, pr_ascii(last_char), last_char);
				}
				break;
			case 'Q':
				less_quit = 1;
				break;
			case '\n':
			case '!':
				break;
			default:
				fprintf(stderr, "unrecognized char at start of \"%s\"\n", line);
				return 0;
			}
		}
		set_intr_handler(0);
	}
	destroy_less_pipeline(pipeline);
	fprintf(stderr, "%s %s (%d commands)\n", ok ? "OK  " : "FAIL", setup_name, cmds);
	return ok;
}

// Should run in empty directory.
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
#if 0
	fprintf(stderr, "DONE %d test%s", tests, tests==1?"":"s");
	if (tests > fails)  fprintf(stderr, ", %d ok",  tests-fails);
	if (fails > 0)      fprintf(stderr, ", %d failed", fails);
	fprintf(stderr, "\n");
#endif
	fclose(testfd);
	return (fails == 0);
}
