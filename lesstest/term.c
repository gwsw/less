#include <string.h>
#include <fcntl.h>
#include <termios.h>
#include <termcap.h>
#include <sys/ioctl.h>
#include "lesstest.h"

TermInfo terminfo;

void raw_mode(int tty, int on) {
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

int env_number(const char* s) {
	return (s == NULL) ? 0 : atoi(s);
}

#if 0
int get_screen_size(int* screen_width, int* screen_height) {
	*screen_width = env_number("COLUMNS");
	*screen_height = env_number("ROWS");
	if (*screen_width == 0 || *screen_height == 0) {
		struct winsize w;
		if (ioctl(2, TIOCGWINSZ, &w) == 0) {
			*screen_height = w.ws_row;
			*screen_width = w.ws_col;
		}
	}
	return (*screen_width > 0 && *screen_height > 0);
}
#endif

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
	setup_mode("so", "se", &terminfo.enter_standout, &terminfo.exit_standout, &sp);
	setup_mode("us", "ue", &terminfo.enter_underline, &terminfo.exit_underline, &sp);
	setup_mode("md", "me", &terminfo.enter_bold, &terminfo.exit_bold, &sp);
	setup_mode("mb", "me", &terminfo.enter_blink, &terminfo.exit_blink, &sp);
	terminfo.cursor_move = tgetstr("cm", &sp);
	if (terminfo.cursor_move == NULL) terminfo.cursor_move = "";
	terminfo.clear_screen = tgetstr("cl", &sp);
	if (terminfo.clear_screen == NULL) terminfo.clear_screen = "";
	char* bs = tgetstr("kb", &sp);
	terminfo.backspace_key = (bs != NULL && strlen(bs) == 1) ? *bs : '\b';
	terminfo.init_term = tgetstr("ti", &sp);
	terminfo.deinit_term = tgetstr("te", &sp);
	terminfo.enter_keypad = tgetstr("ks", &sp);
	terminfo.exit_keypad = tgetstr("ke", &sp);
	terminfo.key_right = tgetstr("kr", &sp);
	terminfo.key_left = tgetstr("kl", &sp);
	terminfo.key_up = tgetstr("ku", &sp);
	terminfo.key_down = tgetstr("kd", &sp);
	terminfo.key_home = tgetstr("kh", &sp);
	terminfo.key_end = tgetstr("@7", &sp);
	return 1;
}
