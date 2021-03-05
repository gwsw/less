#include "lt_types.h"

typedef struct wchar_range { wchar first, last; } wchar_range;

static wchar_range wide_chars[] = {
#include "../wide.uni"
};

static int is_in_table(wchar ch, wchar_range table[], int count) {
	if (ch < table[0].first)
		return 0;
	int lo = 0;
	int hi = count - 1;
	while (lo <= hi) {
		int mid = (lo + hi) / 2;
		if (ch > table[mid].last)
			lo = mid + 1;
		else if (ch < table[mid].first)
			hi = mid - 1;
		else
			return 1;
	}
	return 0;
}

int is_wide_char(wchar ch) {
	return is_in_table(ch, wide_chars, countof(wide_chars));
}
