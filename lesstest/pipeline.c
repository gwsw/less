#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include "lesstest.h"

#define RD 0
#define WR 1

extern int verbose;
extern int rstat_file;
static char* rstat_file_name = "less.rstat";
static char* lt_screen = "./lt_screen"; // FIXME

static void dup_and_close(int dup0, int dup1, int close0, int close1) {
	if (close0 >= 0) close(close0);
	if (close1 >= 0) close(close1);
	if (dup0 >= 0) dup2(dup0, 0);
	if (dup1 >= 0) dup2(dup1, 1);
}

int create_less_pipeline(char const* testname, char* const* less_argv, int less_argc, char* const* less_envp, int screen_width, int screen_height, int* p_less_in, int* p_screen_out, pid_t* p_screen_pid) {
	int run_less = 1;
	int screen_in_pipe[2];
	if (pipe(screen_in_pipe) < 0)
		return 0;
	unlink(rstat_file_name);
	rstat_file = open(rstat_file_name, O_CREAT|O_RDONLY, 0664);
	if (rstat_file < 0) {
		fprintf(stderr, "cannot create %s: errno %d\n", rstat_file_name, errno);
		return 0;
	}
	if (verbose) fprintf(stderr, "less out pipe %d,%d\n", screen_in_pipe[0], screen_in_pipe[1]);
	int less_in_pipe[2];
	if (run_less) { 
		if (pipe(less_in_pipe) < 0)
			return 0;
		if (verbose) fprintf(stderr, "less in pipe %d,%d\n", less_in_pipe[RD], less_in_pipe[WR]);
		char* less = less_argv[0];
		char* textfile = less_argv[less_argc-1];
		if (testname == NULL)
			testname = textfile;
		if (verbose) fprintf(stderr, "test '%s': testing %s on %s\n", testname, less, textfile);
		if (!log_test_header(testname, screen_width, screen_height))
			return 0;
		if (!log_command(less_argv, less_argc))
			return 0;
		if (!log_textfile(textfile))
			return 0;
		if (!log_test_footer())
			return 0;
		pid_t less_pid = fork();
		if (less_pid < 0)
			return 0;
		if (!less_pid) { // child: less
			if (verbose) fprintf(stderr, "less child: in %d, out %d, close %d,%d\n", less_in_pipe[RD], screen_in_pipe[WR], less_in_pipe[WR], screen_in_pipe[RD]);
			dup_and_close(less_in_pipe[RD], screen_in_pipe[WR],
			              less_in_pipe[WR], screen_in_pipe[RD]);
			if (verbose) { print_strings("less argv", less_argv); print_strings("less envp", less_envp); }
			execve(less, less_argv, less_envp);
			fprintf(stderr, "cannot exec %s: errno %d\n", less, errno);
			exit(1);
		}
		if (verbose) fprintf(stderr, "less child %ld\n", (long) less_pid);
		close(less_in_pipe[RD]);
		close(screen_in_pipe[WR]);
	}
	int screen_out_pipe[2];
	if (pipe(screen_out_pipe) < 0) {
		// FIXME cleanup
		return 0;
	}
	if (verbose) fprintf(stderr, "screen out pipe %d,%d\n", screen_out_pipe[RD], screen_out_pipe[WR]);
	*p_screen_pid = fork();
	//pid_t gen_pid = getpid();
	if (!*p_screen_pid) { // child: lt_screen
		if (verbose) fprintf(stderr, "screen child: in %d, out %d, close %d\n", screen_in_pipe[RD], screen_out_pipe[WR], screen_out_pipe[RD]);
		dup_and_close(screen_in_pipe[RD], screen_out_pipe[WR], screen_out_pipe[RD], -1);
		//char gen_pid_str[32];
		char* screen_argv[32];
		int screen_argc = 0;
		char sw[16];
		char sh[16];
		screen_argv[screen_argc++] = lt_screen;
		if (screen_width >= 0) {
			snprintf(sw, sizeof(sw), "%d", screen_width);
			screen_argv[screen_argc++] = "-w";
			screen_argv[screen_argc++] = sw;
		}
		if (screen_height >= 0) {
			snprintf(sh, sizeof(sh), "%d", screen_height);
			screen_argv[screen_argc++] = "-h";
			screen_argv[screen_argc++] = sh;
		}
		if (1)
			screen_argv[screen_argc++] = "-q";
		if (verbose)
			screen_argv[screen_argc++] = "-v";
		screen_argv[screen_argc] = NULL;
		if (verbose) print_strings("screen argv", screen_argv);
		char* const screen_envp[] = { NULL };
		execve(lt_screen, screen_argv, screen_envp);
		fprintf(stderr, "cannot exec %s: errno %d\n", lt_screen, errno);
		exit(1);
	}
	if (verbose) fprintf(stderr, "screen child %ld\n", (long)*p_screen_pid);
	close(screen_out_pipe[WR]);
	close(screen_in_pipe[RD]);
	*p_less_in = run_less ? less_in_pipe[WR] : screen_in_pipe[WR];
	*p_screen_out = screen_out_pipe[RD];
	if (verbose) fprintf(stderr, "less in %d, screen out %d\n", *p_less_in, *p_screen_out);
	return 1;
}
