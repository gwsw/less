#include <time.h>
#include <errno.h>
#include <setjmp.h>
#include "lesstest.h"

extern int verbose;
extern int less_quit;
extern int details;
extern TermInfo terminfo;
extern int run_catching;
extern jmp_buf run_catch;

void sleep_ms(int ms) {
	#define NS_PER_MS (1000*1000)
	struct timespec tm;
	tm.tv_sec = ms / 1000;
	tm.tv_nsec = (ms % 1000) * NS_PER_MS;
	if (nanosleep(&tm, NULL) < 0)
		fprintf(stderr, "sleep error: %s\n", strerror(errno));
}

void send_char(LessPipeline* pipeline, wchar ch) {
	if (verbose) fprintf(stderr, "send %lx\n", ch);
	byte cbuf[UNICODE_MAX_BYTES];
	byte* cp = cbuf;
	store_wchar(&cp, ch);
	write(pipeline->less_in, cbuf, cp-cbuf);
}

	
void wait_less_ready(LessPipeline* pipeline) {
	if (pipeline->rstat_file < 0) return;
	for (;;) {
		lseek(pipeline->rstat_file, SEEK_SET, 0);
		char st;
		if (read(pipeline->rstat_file, &st, 1) == 1) {
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
	sleep_ms(25); // why is this needed? rstat should prevent need for this
}

int read_screen(LessPipeline* pipeline, byte* buf, int buflen) {
	if (verbose) fprintf(stderr, "gen: read screen\n");
	wait_less_ready(pipeline);
	if (less_quit)
		return 0;
	kill(pipeline->screen_pid, LTSIG_SCREEN_DUMP);
	int rn = 0;
	for (; rn <= buflen; ++rn) {
		if (read(pipeline->screen_out, &buf[rn], 1) != 1)
			break;
		if (buf[rn] == '\n')
			break;
	}
	return rn;
}

void read_and_display_screen(LessPipeline* pipeline) {
	byte rbuf[MAX_SCREENBUF_SIZE];
	int rn = read_screen(pipeline, rbuf, sizeof(rbuf));
	if (rn == 0) return;
	printf("%s", terminfo.clear_screen);
	display_screen(rbuf, rn, pipeline->screen_width, pipeline->screen_height, 1);
	log_screen(rbuf, rn);
}

int curr_screen_match(LessPipeline* pipeline, const byte* img, int imglen) {
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
	char* const* envp = less_envp(prog_envp, LT_ENV_PREFIX);
	LessPipeline* pipeline = create_less_pipeline(argv, argc, envp);
	if (pipeline == NULL)
		return 0;
	const char* textfile = (pipeline->tempfile != NULL) ? pipeline->tempfile : argv[argc-1];
	if (!log_test_header(argv, argc, textfile)) {
		destroy_less_pipeline(pipeline);
		return 0;
	}
	less_quit = 0;
	int ttyin = 0; // stdin
	raw_mode(ttyin, 1);
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
	raw_mode(ttyin, 0);
	destroy_less_pipeline(pipeline);
	return 1;
}

int run_test(TestSetup* setup, FILE* testfd) {
	const char* setup_name = setup->argv[setup->argc-1];
	fprintf(stderr, "RUN  %s\n", setup_name);
	LessPipeline* pipeline = create_less_pipeline(setup->argv, setup->argc, 
			less_envp(setup->env.env_list, ""));
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
		run_catching = 1;
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
	}
	run_catching = 0;
	destroy_less_pipeline(pipeline);
	fprintf(stderr, "%s %s (%d commands)\n", ok ? "OK  " : "FAIL", setup_name, cmds);
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
#if 0
	fprintf(stderr, "DONE %d test%s", tests, tests==1?"":"s");
	if (tests > fails)  fprintf(stderr, ", %d ok",  tests-fails);
	if (fails > 0)      fprintf(stderr, ", %d failed", fails);
	fprintf(stderr, "\n");
#endif
	fclose(testfd);
	return (fails == 0);
}
