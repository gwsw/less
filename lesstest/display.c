#include <termcap.h>
#include "lesstest.h"

extern TermInfo terminfo;

void display_attr(Attr attr) {
	static Attr prev_attr = 0;
	if (attr == prev_attr)
		return;
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

void display_color(Color fg_color, Color bg_color) {
printf("{%x/%x}", fg_color, bg_color);
}

void display_screen(const byte* img, int imglen, int screen_width, int screen_height, int move_cursor) {
	int x = 0;
	int y = 0;
	int cursor_x = 0;
	int cursor_y = 0;
	int literal = 0;
	while (imglen-- > 0) {
		wchar ch = load_wchar(&img);
		if (!literal) {
			if (ch == '\\') {
				literal = 1;
				continue;
			} else if (ch == '@') {
				Attr attr = *img++;
				display_attr(attr);
				continue;
			} else if (ch == '$') {
				Color fg_color = *img++;
				Color bg_color = *img++;
				display_color(fg_color, bg_color);
				continue;
			} else if (ch == '#') {
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
	if (move_cursor)
		printf("%s", tgoto(terminfo.cursor_move, cursor_x, cursor_y));
	fflush(stdout);
}

void print_strings(const char* title, char* const* strings) {
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
}
