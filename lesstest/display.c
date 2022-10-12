#include <termcap.h>
#include "lesstest.h"

extern TermInfo terminfo;

static void display_attr(Attr attr) {
	static Attr prev_attr = 0;
	if (prev_attr & ATTR_STANDOUT)
		printf("%s", terminfo.exit_standout);
	if (prev_attr & ATTR_BLINK)
		printf("%s", terminfo.exit_blink);
	if (prev_attr & ATTR_BOLD)
		printf("%s", terminfo.exit_bold);
	if (prev_attr & ATTR_UNDERLINE)
		printf("%s", terminfo.exit_underline);
	if (attr & ATTR_UNDERLINE)
		printf("%s", terminfo.enter_underline);
	if (attr & ATTR_BOLD)
		printf("%s", terminfo.enter_bold);
	if (attr & ATTR_BLINK)
		printf("%s", terminfo.enter_blink);
	if (attr & ATTR_STANDOUT)
		printf("%s", terminfo.enter_standout);
	prev_attr = attr;
}

static void display_color(Color color) {
	if (color == NULL_COLOR)
		printf("\33[m");
	else
		printf("\33[%dm", color);
}

void display_screen(const byte* img, int imglen, int screen_width, int screen_height) {
	int x = 0;
	int y = 0;
	int cursor_x = 0;
	int cursor_y = 0;
	int literal = 0;
	Attr curr_attr = 0;
	Color curr_color = NULL_COLOR;
	while (imglen-- > 0) {
		wchar ch = load_wchar(&img);
		if (!literal) {
			switch (ch) {
			case '\\':
				literal = 1;
				continue;
			case LTS_CHAR_ATTR:
				curr_attr = *img++;
				display_attr(curr_attr);
				if (curr_color != NULL_COLOR)
					display_color(curr_color);
				continue;
			case LTS_CHAR_COLOR:
				curr_color = *img++;
				display_color(curr_color);
				if (curr_attr != 0)
					display_attr(curr_attr);
				continue;
			case LTS_CHAR_CURSOR:
				cursor_x = x;
				cursor_y = y;
				continue;
			}
		}
		literal = 0;
		if (ch != 0) {
			byte cbuf[UNICODE_MAX_BYTES];
			byte* cp = cbuf;
			store_wchar(&cp, ch);
			fwrite(cbuf, 1, cp-cbuf, stdout);
		}
		if (++x >= screen_width) {
			printf("\n");
			x = 0;
			if (++y >= screen_height)
				break;
		}
	}
	printf("%s", tgoto(terminfo.cursor_move, cursor_x, cursor_y));
	fflush(stdout);
}

void display_screen_debug(const byte* img, int imglen, int screen_width, int screen_height) {
	int x = 0;
	int y = 0;
	int literal = 0;
	while (imglen-- > 0) {
		wchar ch = load_wchar(&img);
		if (!literal) {
			switch (ch) {
			case '\\':
				literal = 1;
				continue;
			case LTS_CHAR_ATTR:
			case LTS_CHAR_COLOR:
				x -= 2; // don't count LTS_CHAR or following byte
				literal = 1;
				break;
			case LTS_CHAR_CURSOR:
				x -= 1; // don't count LTS_CHAR
				break;
			}
		}
		literal = 0;
		if (is_ascii(ch))
			fwrite(&ch, 1, 1, stdout);
		else
			printf("<%lx>", (unsigned long) ch);
		if (++x >= screen_width) {
			printf("\n");
			x = 0;
			if (++y >= screen_height)
				break;
		}
	}
	fflush(stdout);
}

void print_strings(const char* title, char* const* strings) {
	if (1) return; ///
	fprintf(stderr, "%s:\n", title);
	char* const* s;
	for (s = strings; *s != NULL; ++s) {
		fprintf(stderr, " ");
		const char* p;
		for (p = *s; *p != '\0'; ++p) {
			if (is_ascii(*p))
				fprintf(stderr, "%c", (char) *p);
			else
				fprintf(stderr, "\\x%04x", *p);
		}
		fprintf(stderr, "\n");
	}
	fprintf(stderr, "%s- end\n", title);
}
