#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include "lt_types.h"
#include "wchar.h"

typedef struct TestSetup {
	char* setup_name;
	char* textfile;
	char** argv;
	int argc;
	int width;
	int height;
	char* charset;
} TestSetup;

typedef struct LessPipeline {
	int less_in;
	int screen_out;
	int rstat_file;
	pid_t screen_pid;
	char* tempfile;
	int less_in_pipe[2];
	int screen_in_pipe[2];
	int screen_out_pipe[2];
} LessPipeline;

typedef struct TermInfo {
	char backspace_key;
	char* enter_underline;
	char* exit_underline;
	char* enter_bold;
	char* exit_bold;
	char* enter_blink;
	char* exit_blink;
	char* enter_standout;
	char* exit_standout;
	char* clear_screen;
	char* cursor_move;
	char* key_right;
	char* key_left;
	char* key_up;
	char* key_down;
	char* key_home;
	char* key_end;
	char* enter_keypad;
	char* exit_keypad;
	char* init_term;
	char* deinit_term;
} TermInfo;

int log_open(char const* logfile);
void log_close(void);
int log_file_header(void);
int log_test_header(const char* testname, int screen_width, int screen_height, const char* charset, char* const* argv, int argc, const char* textfile);
int log_tty_char(wchar ch);
int log_screen(byte const* img, int len);
LessPipeline* create_less_pipeline(const char* testname, char* const* argv, int argc, char* const* envp, int screen_width, int screen_height, int do_log);
void destroy_less_pipeline(LessPipeline* pipeline);
void print_strings(const char* title, char* const* strings);
void free_test_setup(TestSetup* setup);
TestSetup* read_test_setup(FILE* fd, char const* less);
int read_zline(FILE* fd, char* line, int line_len);
void raw_mode(int tty, int on);
int get_screen_size(void);
int setup_term(void);
void display_screen(const byte* img, int imglen, int move_cursor);
const char* get_envp(char* const* envp, const char* name);
int run_interactive(char* const* argv, int argc);
int run_testfile(const char* testfile, const char* less);
