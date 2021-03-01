#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <signal.h>
#include <errno.h>

#define ESC                '\33'

#define ATTR_BOLD          (1<<0)
#define ATTR_UNDERLINE     (1<<1)
#define ATTR_STANDOUT      (1<<2)
#define ATTR_BLINK         (1<<3)

#define LTSIG_SCREEN_READY  SIGUSR1
#define LTSIG_SCREEN_DUMP   SIGUSR2

typedef struct TestSetup {
	char* setup_name;
	char* textfile;
	char** argv;
	int argc;
	int width;
	int height;
} TestSetup;

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
} TermInfo;

int log_open(char const* logfile);
void log_close(void);
int log_header(void);
int log_test_header(char const* testname, int screen_width, int screen_height);
int log_test_footer(void);
int log_tty_char(char ch);
int log_screen(char const* img, int len);
int log_command(char* const* argv, int argc);
int log_textfile(char const* textfile);
int create_less_pipeline(char const* testname, char* const* less_argv, int less_argc, char* const* less_envp, int screen_width, int screen_height, int* p_less_in, int* p_screen_out, pid_t* p_screen_pid);
void print_strings(char const* title, char* const* strings);
void free_test_setup(TestSetup* setup);
TestSetup* read_test_setup(FILE* fd, char const* less);
int read_zline(FILE* fd, char* line, int line_len);
void raw_mode(int tty, int on);
int get_screen_size(void);
int setup_term(void);
void display_screen(char const* img, int imglen, int move_cursor);
