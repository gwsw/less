#include <time.h>
#include <errno.h>
#include <setjmp.h>
#include <signal.h>
#include <sys/wait.h>
#include <termcap.h>
#include "lesstest.h"

extern int verbose;
extern int less_quit;
extern int details;
extern char* details_file;
extern int err_only;
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

static void set_signal_handlers(int set) {
	set_signal(SIGINT,  set ? SIG_IGN : SIG_DFL);
	set_signal(SIGQUIT, set ? SIG_IGN : SIG_DFL);
	set_signal(SIGKILL, set ? SIG_IGN : SIG_DFL);
	set_signal(SIGPIPE, set ? SIG_IGN : SIG_DFL);
	set_signal(SIGCHLD, set ? child_handler : SIG_DFL);
}

// Send a command char to a LessPipeline.
static void send_char(LessPipeline* pipeline, wchar ch) {
	if (verbose) fprintf(stderr, "lt.send %lx\n", ch);
	byte cbuf[UNICODE_MAX_BYTES];
	byte* cp = cbuf;
	store_wchar(&cp, ch);
	write(pipeline->less_in, cbuf, cp-cbuf);
}

// Read the screen image from the lt_screen in a LessPipeline.
static int read_screen(LessPipeline* pipeline, byte* buf, int buflen) {
	if (verbose) fprintf(stderr, "lt.gen: read screen\n");
	send_char(pipeline, LESS_DUMP_CHAR);
	int rn = 0;
	for (; rn <= buflen; ++rn) {
		byte ch;
		if (read(pipeline->screen_out, &ch, 1) != 1)
			break;
		if (ch == '\n')
			break;
		if (buf != NULL) buf[rn] = ch;
	}
	return rn;
}

// Read screen image from a LessPipeline and display it.
static void read_and_display_screen(LessPipeline* pipeline) {
	byte rbuf[MAX_SCREENBUF_SIZE];
	int rn = read_screen(pipeline, rbuf, sizeof(rbuf));
	if (rn == 0) return;
	printf("%s", terminfo.clear_screen);
	display_screen(rbuf, rn, pipeline->screen_width, pipeline->screen_height);
	log_screen(rbuf, rn);
}

// Is the screen image in a LessPipeline equal to a given buffer?
static int curr_screen_match(LessPipeline* pipeline, const byte* img, int imglen, char const* textfile, int cmd_num) {
	byte curr[MAX_SCREENBUF_SIZE];
	int currlen = read_screen(pipeline, curr, sizeof(curr));
	if (currlen == imglen && memcmp(img, curr, imglen) == 0)
		return 1;
	if (details) {
		fprintf(stderr, "lt: mismatch: expect:\n");
		display_screen_debug(img, imglen, pipeline->screen_width, pipeline->screen_height);
		fprintf(stderr, "lt: got:\n");
		display_screen_debug(curr, currlen, pipeline->screen_width, pipeline->screen_height);
	}
	if (details_file != NULL) {
		FILE* df = fopen(details_file, "w");
		if (df == NULL) {
			fprintf(stderr, "cannot create %s: %s\n", details_file, strerror(errno));
		} else {
			fprintf(df, "!lesstest-details!\n");
			fprintf(df, "F \"%s\"\n", textfile);
			fprintf(df, "N %d\n", cmd_num);
			fprintf(df, "=%.*s\n", imglen, img);
			fprintf(df, "<%.*s\n", currlen, curr);
			fclose(df);
		}
	}
	return 0;
}

// Run an interactive lesstest session to create an lt file.
// Read individual chars from stdin and send them to a LessPipeline.
// After each char, read the LessPipeline screen and display it 
// on the user's screen. 
// Also log the char and the screen image in the lt file.
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
	set_signal_handlers(1);
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
	printf("%s%s%s", terminfo.clear_screen, terminfo.exit_keypad, terminfo.deinit_term);
	raw_mode(ttyin, 0);
	destroy_less_pipeline(pipeline);
	set_signal_handlers(0);
	return 1;
}

// Run a test of less, as directed by an open lt file.
// Read a logged char and screen image from the lt file.
// Send the char to a LessPipeline, then read the LessPipeline screen image
// and compare it to the screen image from the lt file.
// Report an error if they differ.
static int run_test(TestSetup* setup, FILE* testfd) {
	const char* setup_name = setup->argv[setup->argc-1];
	//fprintf(stderr, "RUN  %s\n", setup_name);
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
		set_signal_handlers(1);
		(void) read_screen(pipeline, NULL, MAX_SCREENBUF_SIZE); // wait until less is running
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
				if (!curr_screen_match(pipeline, (byte*)line+1, line_len-1, setup->textfile, cmds)) {
					ok = 0;
					less_quit = 1;
					fprintf(stderr, "DIFF %s on cmd #%d (%c %lx)\n",
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
		set_signal_handlers(0);
	}
	destroy_less_pipeline(pipeline);
	if (!ok)
		printf("FAIL: %s (%d steps)\n", setup_name, cmds);
	else if (!err_only)
		printf("PASS: %s (%d steps)\n", setup_name, cmds);
	return ok;
}

// Run a test of less, as directed by a named lt file.
// Should be run in an empty temp directory;
// it creates its own files in the current directory.
int run_testfile(const char* ltfile, const char* less) {
	FILE* testfd = fopen(ltfile, "r");
	if (testfd == NULL) {
		fprintf(stderr, "cannot open %s\n", ltfile);
		return 0;
	}
	int fails = 0;
	// This for loop is to handle multiple tests in one file.
	for (;;) {
		TestSetup* setup = read_test_setup(testfd, less);
		if (setup == NULL)
			break;
		int ok = run_test(setup, testfd);
		free_test_setup(setup);
		if (!ok) ++fails;
	}
	fclose(testfd);
	return (fails == 0);
}

static void free_states(TestState* states, int num_states) {
	int s;
	for (s = 0; s < num_states; ++s) {
		free(states[s].screen);
	}
	free(states);
}

static int read_states(FILE* testfd, TestState** p_states) {
	TestState* states = NULL;
	int num_states = 0;
	int max_states = 0;
	int quit = 0;
	while (!quit) {
		char line[10000];
		int line_len = read_zline(testfd, line, sizeof(line));
		if (line_len < 0)
			break;
		if (line_len < 1)
			continue;
		switch (line[0]) {
		case '=': 
			if (num_states >= max_states) {
				TestState* new_states;
				max_states = (max_states+1) * 2;
				new_states = malloc(max_states * sizeof(TestState));
				if (states != NULL) {
					memcpy(new_states, states, num_states * sizeof(TestState));
					free(states);
				}
				states = new_states;
			}
			states[num_states].screen_len = line_len-1;
			states[num_states].screen = (byte*) malloc(states[num_states].screen_len);
			states[num_states].ch = 0;
			memcpy(states[num_states].screen, line+1, states[num_states].screen_len);
			break;
		case '+':
			states[num_states].ch = (wchar) strtol(line+1, NULL, 16);
			++num_states;
			break;
		case 'Q':
			quit = 1;
			break;
		case '\n':
		case '!':
			break;
		default:
			fprintf(stderr, "unrecognized char at start of \"%s\"\n", line);
			free_states(states, num_states);
			return -1;
		}
	}
	*p_states = states;
	return num_states;
}

int explore_testfile(const char* ltfile) {
	TestDetails* td = NULL;
	if (details_file != NULL) {
		FILE* df = fopen(details_file, "r");
		if (df == NULL) {
			fprintf(stderr, "cannot open %s\n", details_file);
			return 0;
		}
		td = read_test_details(df);
		fclose(df);
		if (td == NULL)
			return 0;
	}
	FILE* testfd = fopen(ltfile, "r");
	if (testfd == NULL) {
		fprintf(stderr, "cannot open %s\n", ltfile);
		free_test_details(td);
		return 0;
	}
	int ok = 0;
	TestSetup* setup = read_test_setup(testfd, NULL);
	if (setup != NULL) {
		TestState* states;
		int num_states = read_states(testfd, &states);
		const char* w = get_envp(setup->env.env_list, "COLUMNS");
		const char* h = get_envp(setup->env.env_list, "LINES");
		int screen_width;
		int screen_height;
		int ttyin = 0; // stdin
		if (w == NULL || (screen_width = atoi(w)) <= 0 ||
		    h == NULL || (screen_height = atoi(h)) <= 0) {
			fprintf(stderr, "no screen geometry found in %s\n", ltfile);
		} else {
			int curr_state = 0;
			int num = -1;
			int disp_td = 0;
			setup_term();
			raw_mode(ttyin, 1);
			printf("%s%s", terminfo.init_term, terminfo.enter_keypad);
			while (!less_quit) {
				printf("%s%s%s[%d/%d]%s", terminfo.clear_screen,
					tgoto(terminfo.cursor_move, 1, screen_height+1),
					terminfo.enter_bold, curr_state+1, num_states, terminfo.exit_bold);
				if (td != NULL && curr_state == td->cmd_num)
					printf(" %s", disp_td ? "FAILED(bad)" : "FAILED(good)");
				if (states[curr_state].ch) {
					wchar ch = states[curr_state].ch;
					printf("  Next key: ");
					switch (ch) {
					case ESC:  printf("ESC"); break;
					case '\b': printf("\\b"); break;
					case '\n': printf("\\n"); break;
					case '\r': printf("\\r"); break;
					case '\t': printf("\\t"); break;
					default:
						if (ch > ' ' && ch < 0x7f)
							printf("%c", (char) ch);
						else
							printf("0x%lx", (long) ch);
						break;
					}
				}
				if (num >= 0) printf("  Number:%d", num);
				printf("%s", tgoto(terminfo.cursor_move, 0, 0));
				if (disp_td) {
					display_screen(td->img_actual, td->len_actual, screen_width, screen_height);
				} else {
					display_screen(states[curr_state].screen, states[curr_state].screen_len, screen_width, screen_height);
				}
				wchar ch = read_wchar(ttyin);
				if (ch == 'q') break;
				if (ch >= '0' && ch <= '9') {
					if (num < 0) num = 0;
					num = (10 * num) + (ch - '0');
					continue;
				}
				switch (ch) {
				case 'l':
					if (num <= 0) num = 1;
					curr_state += num;
					if (curr_state >= num_states) curr_state = num_states-1;
					disp_td = 0;
					break;
				case 'h':
					if (num <= 0) num = 1;
					curr_state -= num;
					if (curr_state < 0) curr_state = 0;
					disp_td = 0;
					break;
				case 'g': case 'G':
					if (num < 0 && ch == 'G') num = num_states;
					if (num <= 0) num = 1;
					curr_state = num-1;
					if (curr_state >= num_states) curr_state = num_states-1;
					if (curr_state < 0) curr_state = 0;
					disp_td = 0;
					break;
				case 'L': case 'H':
					if (td != NULL)
						curr_state = td->cmd_num;
					break;
				case 'j': case 'k':
					if (td != NULL && td->cmd_num == curr_state)
						disp_td = !disp_td;
					break;
				case '?':
					printf("%s%s", tgoto(terminfo.cursor_move, 0, screen_height+1), terminfo.clear_eos);
					printf("Commands in -x mode:\n");
					printf("  [N]l   Go to (N-th) next state.\n");
					printf("  [N]h   Go to (N-th) previous state.\n");
					printf("  [N]g   Go to first (or N-th) state.\n");
					printf("  [N]G   Go to last (or N-th) state.\n");
					printf("     L   Go to failed state.\n");
					printf("     j   Toggle between good and failed screens.\n");
					printf("Press any key to continue.\n");
					(void) read_wchar(ttyin);
					break;
				default:
					break;
				}
				num = -1;
			}
			printf("%s%s%s", terminfo.clear_screen, terminfo.exit_keypad, terminfo.deinit_term);
			raw_mode(ttyin, 0);
			free_test_setup(setup);
		}
		free_states(states, num_states);
	}
	fclose(testfd);
	free_test_details(td);
	return ok;
}
