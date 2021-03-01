/*@@copyright@@*/

/*
 * Routines to manipulate the "line buffer".
 * The line buffer holds a line of output as it is being built
 * in preparation for output to the screen.
 */

#include "less.h"
#include "charset.h"
#include "position.h"

#if MSDOS_COMPILER==WIN32C
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#endif

#define MAX_PFX_WIDTH (MAX_LINENUM_WIDTH + MAX_STATUSCOL_WIDTH + 1)
static struct {
	char *buf;    /* Buffer which holds the current output line */
	int *attr;   /* Parallel to buf, to hold attributes */
	int print;    /* Index in buf of first printable char */
	int end;      /* Number of chars in buf */
	char pfx[MAX_PFX_WIDTH]; /* Holds status column and line number */
	int pfx_attr[MAX_PFX_WIDTH];
	int pfx_end;  /* Number of chars in pfx */
} linebuf;

static struct {
	char *buf;
	int size;
	int end;
} shifted_ansi;

public int size_linebuf = 0; /* Size of line buffer (and attr buffer) */
static struct ansi_state *line_ansi = NULL;
static int cshift;   /* Current left-shift of output line buffer */
public int hshift;   /* Desired left-shift of output line buffer */
public int tabstops[TABSTOP_MAX] = { 0 }; /* Custom tabstops */
public int ntabstops = 1;        /* Number of tabstops */
public int tabdefault = 8;       /* Default repeated tabstops */
public POSITION highest_hilite;  /* Pos of last hilite in file found so far */

static int end_column;  /* Printable length, accounting for backspaces, etc. */
static int right_curr;
static int right_column;
static int overstrike;  /* Next char should overstrike previous char */
static int last_overstrike = AT_NORMAL;
static int is_null_line;  /* There is no current line */
static LWCHAR pendc;
static POSITION pendpos;
static char *end_ansi_chars;
static char *mid_ansi_chars;

static int attr_swidth LESSPARAMS ((int a));
static int attr_ewidth LESSPARAMS ((int a));
static int do_append LESSPARAMS ((LWCHAR ch, char *rep, POSITION pos));

extern int sigs;
extern int bs_mode;
extern int linenums;
extern int ctldisp;
extern int twiddle;
extern int binattr;
extern int status_col;
extern int status_col_width;
extern int linenum_width;
extern int auto_wrap, ignaw;
extern int bo_s_width, bo_e_width;
extern int ul_s_width, ul_e_width;
extern int bl_s_width, bl_e_width;
extern int so_s_width, so_e_width;
extern int sc_width, sc_height;
extern int utf_mode;
extern POSITION start_attnpos;
extern POSITION end_attnpos;
extern char rscroll_char;
extern int rscroll_attr;
extern int use_color;

static char mbc_buf[MAX_UTF_CHAR_LEN];
static int mbc_buf_len = 0;
static int mbc_buf_index = 0;
static POSITION mbc_pos;

/* Configurable color map */
static char color_map[AT_NUM_COLORS][12] = {
	"Wm",  /* AT_COLOR_ATTN */
	"kR",  /* AT_COLOR_BIN */
	"kR",  /* AT_COLOR_CTRL */
	"kY",  /* AT_COLOR_ERROR */
	"c",   /* AT_COLOR_LINENUM */
	"Wb",  /* AT_COLOR_MARK */
	"kC",  /* AT_COLOR_PROMPT */
	"kc",  /* AT_COLOR_RSCROLL */
	"kG",  /* AT_COLOR_SEARCH */
	"",    /* AT_UNDERLINE */
	"",    /* AT_BOLD */
	"",    /* AT_BLINK */
	"",    /* AT_STANDOUT */
};

/* State while processing an ANSI escape sequence */
struct ansi_state {
	int hindex;   /* Index into hyperlink prefix */
	int hlink;    /* Processing hyperlink address? */
	int prev_esc; /* Prev char was ESC (to detect ESC-\ seq) */
};

/*
 * Initialize from environment variables.
 */
	public void
init_line(VOID_PARAM)
{
	end_ansi_chars = lgetenv("LESSANSIENDCHARS");
	if (isnullenv(end_ansi_chars))
		end_ansi_chars = "m";

	mid_ansi_chars = lgetenv("LESSANSIMIDCHARS");
	if (isnullenv(mid_ansi_chars))
		mid_ansi_chars = "0123456789:;[?!\"'#%()*+ ";

	linebuf.buf = (char *) ecalloc(LINEBUF_SIZE, sizeof(char));
	linebuf.attr = (int *) ecalloc(LINEBUF_SIZE, sizeof(int));
	size_linebuf = LINEBUF_SIZE;
	shifted_ansi.buf = NULL;
	shifted_ansi.size = 0;
}

/*
 * Expand the line buffer.
 */
	static int
expand_linebuf(VOID_PARAM)
{
	/* Double the size of the line buffer. */
	int new_size = size_linebuf * 2;

	/* Just realloc to expand the buffer, if we can. */
#if HAVE_REALLOC
	char *new_buf = (char *) realloc(linebuf.buf, new_size);
	int *new_attr = (int *) realloc(linebuf.attr, new_size*sizeof(int));
#else
	char *new_buf = (char *) calloc(new_size, sizeof(char));
	int *new_attr = (int *) calloc(new_size, sizeof(int));
#endif
	if (new_buf == NULL || new_attr == NULL)
	{
		if (new_attr != NULL)
			free(new_attr);
		if (new_buf != NULL)
			free(new_buf);
		return 1;
	}
#if !HAVE_REALLOC
	/*
	 * We just calloc'd the buffers; copy the old contents.
	 */
	memcpy(new_buf, linebuf.buf, size_linebuf * sizeof(char));
	memcpy(new_attr, linebuf.attr, size_linebuf * sizeof(int));
	free(linebuf.attr);
	free(linebuf.buf);
#endif
	linebuf.buf = new_buf;
	linebuf.attr = new_attr;
	size_linebuf = new_size;
	return 0;
}

/*
 * Is a character ASCII?
 */
	public int
is_ascii_char(ch)
	LWCHAR ch;
{
	return (ch <= 0x7F);
}

/*
 */
	static void
inc_end_column(w)
	int w;
{
	if (end_column > right_column && w > 0)
	{
		right_column = end_column;
		right_curr = linebuf.end;
	}
	end_column += w;
}

/*
 * Rewind the line buffer.
 */
	public void
prewind(VOID_PARAM)
{
	linebuf.print = 6; /* big enough for longest UTF-8 sequence */
	linebuf.pfx_end = 0;
	for (linebuf.end = 0; linebuf.end < linebuf.print; linebuf.end++)
	{
		linebuf.buf[linebuf.end] = '\0';
		linebuf.attr[linebuf.end] = 0;
	}

	end_column = 0;
	right_curr = 0;
	right_column = 0;
	cshift = 0;
	overstrike = 0;
	last_overstrike = AT_NORMAL;
	mbc_buf_len = 0;
	is_null_line = 0;
	pendc = '\0';
	shifted_ansi.end = 0;
}

/*
 * Set a character in the line buffer.
 */
	static void
set_linebuf(n, ch, attr)
	int n;
	char ch;
	int attr;
{
	linebuf.buf[n] = ch;
	linebuf.attr[n] = attr;
}

/*
 * Append a character to the line buffer.
 */
	static void
add_linebuf(ch, attr, w)
	char ch;
	int attr;
	int w;
{
	set_linebuf(linebuf.end++, ch, attr);
	inc_end_column(w);
}

/*
 * Set a character in the line prefix buffer.
 */
	static void
set_pfx(n, ch, attr)
	int n;
	char ch;
	int attr;
{
	linebuf.pfx[n] = ch;
	linebuf.pfx_attr[n] = attr;
}

/*
 * Append a character to the line prefix buffer.
 */
	static void
add_pfx(ch, attr)
	char ch;
	int attr;
{
	set_pfx(linebuf.pfx_end++, ch, attr);
}

/*
 * Insert the status column and line number into the line buffer.
 */
	public void
plinestart(pos)
	POSITION pos;
{
	LINENUM linenum = 0;
	int i;

	if (linenums == OPT_ONPLUS)
	{
		/*
		 * Get the line number and put it in the current line.
		 * {{ Note: since find_linenum calls forw_raw_line,
		 *    it may seek in the input file, requiring the caller 
		 *    of plinestart to re-seek if necessary. }}
		 * {{ Since forw_raw_line modifies linebuf, we must
		 *    do this first, before storing anything in linebuf. }}
		 */
		linenum = find_linenum(pos);
	}

	/*
	 * Display a status column if the -J option is set.
	 */
	if (status_col)
	{
		int a = AT_NORMAL;
		char c = posmark(pos);
		if (c != 0)
			a |= AT_HILITE|AT_COLOR_MARK;
		else 
		{
			c = ' ';
			if (start_attnpos != NULL_POSITION &&
			    pos >= start_attnpos && pos <= end_attnpos)
				a |= AT_HILITE|AT_COLOR_ATTN;
		}
		add_pfx(c, a); /* column 0: status */
		while (linebuf.pfx_end < status_col_width)
			add_pfx(' ', AT_NORMAL);
	}

	/*
	 * Display the line number at the start of each line
	 * if the -N option is set.
	 */
	if (linenums == OPT_ONPLUS)
	{
		char buf[INT_STRLEN_BOUND(linenum) + 2];
		int len;

		linenumtoa(linenum, buf);
		len = (int) strlen(buf);
		for (i = 0; i < linenum_width - len; i++)
			add_pfx(' ', AT_NORMAL);
		for (i = 0; i < len; i++)
			add_pfx(buf[i], AT_NORMAL|AT_COLOR_LINENUM);
		add_pfx(' ', AT_NORMAL);
	}
	end_column = linebuf.pfx_end;
}

/*
 * Return the width of the line prefix (status column and line number).
 * {{ Actual line number can be wider than linenum_width. }}
 */
	public int
line_pfx_width(VOID_PARAM)
{
	int width = 0;
	if (status_col)
		width += status_col_width;
	if (linenums == OPT_ONPLUS)
		width += linenum_width + 1;
	return width;
}

/*
 * Add char to the shifted_ansi buffer.
 */
	static void
add_ansi(ch)
	char ch;
{
	if (shifted_ansi.end == shifted_ansi.size)
	{
		/* Expand shifted_ansi buffer. */
		int size = (shifted_ansi.size == 0) ? 8 : shifted_ansi.size * 2;
		char *buf = (char *) ecalloc(size, sizeof(char));
		memcpy(buf, shifted_ansi.buf, shifted_ansi.size);
		if (shifted_ansi.buf != NULL) free(shifted_ansi.buf);
		shifted_ansi.buf = buf;
		shifted_ansi.size = size;
	}
	shifted_ansi.buf[shifted_ansi.end++] = ch;
}

/*
 * Shift line left so that the last char is just to the left
 * of the first visible column.
 */
	public void
pshift_all(VOID_PARAM)
{
	int i;
	for (i = linebuf.print;  i < linebuf.end;  i++)
		if (linebuf.attr[i] == AT_ANSI)
			add_ansi(linebuf.buf[i]);
	linebuf.end = linebuf.print;
	end_column = linebuf.pfx_end;
}

/*
 * Return the printing width of the start (enter) sequence
 * for a given character attribute.
 */
	static int
attr_swidth(a)
	int a;
{
	int w = 0;

	a = apply_at_specials(a);

	if (a & AT_UNDERLINE)
		w += ul_s_width;
	if (a & AT_BOLD)
		w += bo_s_width;
	if (a & AT_BLINK)
		w += bl_s_width;
	if (a & AT_STANDOUT)
		w += so_s_width;

	return w;
}

/*
 * Return the printing width of the end (exit) sequence
 * for a given character attribute.
 */
	static int
attr_ewidth(a)
	int a;
{
	int w = 0;

	a = apply_at_specials(a);

	if (a & AT_UNDERLINE)
		w += ul_e_width;
	if (a & AT_BOLD)
		w += bo_e_width;
	if (a & AT_BLINK)
		w += bl_e_width;
	if (a & AT_STANDOUT)
		w += so_e_width;

	return w;
}

/*
 * Return the printing width of a given character and attribute,
 * if the character were added after prev_ch.
 * Adding a character with a given attribute may cause an enter or exit
 * attribute sequence to be inserted, so this must be taken into account.
 */
	public int
pwidth(ch, a, prev_ch, prev_a)
	LWCHAR ch;
	int a;
	LWCHAR prev_ch;
	int prev_a;
{
	int w;

	if (ch == '\b')
	{
		/*
		 * Backspace moves backwards one or two positions.
		 */
		if (prev_a & (AT_ANSI|AT_BINARY))
			return strlen(prchar('\b'));
		return (utf_mode && is_wide_char(prev_ch)) ? -2 : -1;
	}

	if (!utf_mode || is_ascii_char(ch))
	{
		if (control_char((char)ch))
		{
			/*
			 * Control characters do unpredictable things,
			 * so we don't even try to guess; say it doesn't move.
			 * This can only happen if the -r flag is in effect.
			 */
			return (0);
		}
	} else
	{
		if (is_composing_char(ch) || is_combining_char(prev_ch, ch))
		{
			/*
			 * Composing and combining chars take up no space.
			 *
			 * Some terminals, upon failure to compose a
			 * composing character with the character(s) that
			 * precede(s) it will actually take up one end_column
			 * for the composing character; there isn't much
			 * we could do short of testing the (complex)
			 * composition process ourselves and printing
			 * a binary representation when it fails.
			 */
			return (0);
		}
	}

	/*
	 * Other characters take one or two columns,
	 * plus the width of any attribute enter/exit sequence.
	 */
	w = 1;
	if (is_wide_char(ch))
		w++;
	if (linebuf.end > 0 && !is_at_equiv(linebuf.attr[linebuf.end-1], a))
		w += attr_ewidth(linebuf.attr[linebuf.end-1]);
	if (apply_at_specials(a) != AT_NORMAL &&
	    (linebuf.end == 0 || !is_at_equiv(linebuf.attr[linebuf.end-1], a)))
		w += attr_swidth(a);
	return (w);
}

/*
 * Delete to the previous base character in the line buffer.
 */
	static int
backc(VOID_PARAM)
{
	LWCHAR ch;
	char *p;

	if (linebuf.end == 0)
		return (0);
	p = &linebuf.buf[linebuf.end];
	ch = step_char(&p, -1, linebuf.buf);
	/* Skip back to the next nonzero-width char. */
	while (p > linebuf.buf)
	{
		LWCHAR prev_ch;
		int width;
		linebuf.end = (int) (p - linebuf.buf);
		prev_ch = step_char(&p, -1, linebuf.buf);
		width = pwidth(ch, linebuf.attr[linebuf.end], prev_ch, linebuf.attr[linebuf.end-1]);
		end_column -= width;
		/* {{ right_column? }} */
		if (width > 0)
			break;
		ch = prev_ch;
	}
	return (1);
}

/*
 * Is a character the end of an ANSI escape sequence?
 */
	public int
is_ansi_end(ch)
	LWCHAR ch;
{
	if (!is_ascii_char(ch))
		return (0);
	return (strchr(end_ansi_chars, (char) ch) != NULL);
}

/*
 * Can a char appear in an ANSI escape sequence, before the end char?
 */
	public int
is_ansi_middle(ch)
	LWCHAR ch;
{
	if (!is_ascii_char(ch))
		return (0);
	if (is_ansi_end(ch))
		return (0);
	return (strchr(mid_ansi_chars, (char) ch) != NULL);
}

/*
 * Skip past an ANSI escape sequence.
 * pp is initially positioned just after the CSI_START char.
 */
	public void
skip_ansi(pansi, pp, limit)
	struct ansi_state *pansi;
	char **pp;
	constant char *limit;
{
	LWCHAR c;
	do {
		c = step_char(pp, +1, limit);
	} while (*pp < limit && ansi_step(pansi, c) != ANSI_MID);
	/* Note that we discard final char, for which is_ansi_end is true. */
}

/*
 * Determine if a character starts an ANSI escape sequence.
 * If so, return an ansi_state struct; otherwise return NULL.
 */
	public struct ansi_state *
ansi_start(ch)
	LWCHAR ch;
{
	struct ansi_state *pansi;

	if (!IS_CSI_START(ch))
		return NULL;
	pansi = ecalloc(1, sizeof(struct ansi_state));
	pansi->hindex = 0;
	pansi->hlink = 0;
	pansi->prev_esc = 0;
	return pansi;
}

/*
 * Determine whether the next char in an ANSI escape sequence
 * ends the sequence.
 */
	public int
ansi_step(pansi, ch)
	struct ansi_state *pansi;
	LWCHAR ch;
{
	if (pansi->hlink)
	{
		/* Hyperlink ends with \7 or ESC-backslash. */
		if (ch == '\7')
			return ANSI_END;
		if (pansi->prev_esc && ch == '\\')
			return ANSI_END;
		pansi->prev_esc = (ch == ESC);
		return ANSI_MID;
	}
	if (pansi->hindex >= 0)
	{
		static char hlink_prefix[] = ESCS "]8;";
		if (ch == hlink_prefix[pansi->hindex++] ||
		    (pansi->hindex == 0 && IS_CSI_START(ch)))
		{
			if (hlink_prefix[pansi->hindex] == '\0')
				pansi->hlink = 1; /* now processing hyperlink addr */
			return ANSI_MID;
		}
		pansi->hindex = -1; /* not a hyperlink */
	}
	/* Check for SGR sequences */
	if (is_ansi_middle(ch))
		return ANSI_MID;
	if (is_ansi_end(ch))
		return ANSI_END;
	return ANSI_ERR;
}

/*
 * Free an ansi_state structure.
 */
	public void
ansi_done(pansi)
	struct ansi_state *pansi;
{
	free(pansi);
}

/*
 * Append a character and attribute to the line buffer.
 */
#define STORE_CHAR(ch,a,rep,pos) \
	do { \
		if (store_char((ch),(a),(rep),(pos))) return (1); \
	} while (0)

	static int
store_char(ch, a, rep, pos)
	LWCHAR ch;
	int a;
	char *rep;
	POSITION pos;
{
	int w;
	int i;
	int replen;
	char cs;

	i = (a & (AT_UNDERLINE|AT_BOLD));
	if (i != AT_NORMAL)
		last_overstrike = i;

#if HILITE_SEARCH
	{
		int matches;
		int hl_attr = is_hilited_attr(pos, pos+1, 0, &matches);
		if (hl_attr)
		{
			/*
			 * This character should be highlighted.
			 * Override the attribute passed in.
			 */
			if (a != AT_ANSI)
			{
				if (highest_hilite != NULL_POSITION && pos > highest_hilite)
					highest_hilite = pos;
				a |= hl_attr;
			}
		}
	}
#endif

	if (a == AT_ANSI) {
		w = 0;
	} else {
		char *p = &linebuf.buf[linebuf.end];
		LWCHAR prev_ch = (linebuf.end > 0) ? step_char(&p, -1, linebuf.buf) : 0;
		int prev_a = (linebuf.end > 0) ? linebuf.attr[linebuf.end-1] : 0;
		w = pwidth(ch, a, prev_ch, prev_a);
	}

	if (ctldisp != OPT_ON && end_column - cshift + w + attr_ewidth(a) > sc_width)
		/*
		 * Won't fit on screen.
		 */
		return (1);

	if (rep == NULL)
	{
		cs = (char) ch;
		rep = &cs;
		replen = 1;
	} else
	{
		replen = utf_len(rep[0]);
	}
	if (linebuf.end + replen >= size_linebuf-6)
	{
		/*
		 * Won't fit in line buffer.
		 * Try to expand it.
		 */
		if (expand_linebuf())
			return (1);
	}

	if (cshift == hshift && shifted_ansi.end > 0)
	{
		/* Copy shifted ANSI sequences to beginning of line. */
		for (i = 0;  i < shifted_ansi.end;  i++)
			add_linebuf(shifted_ansi.buf[i], AT_ANSI, 0);
		shifted_ansi.end = 0;
	}
	/* Add the char to the buf, even if we will left-shift it next. */
	inc_end_column(w);
	for (i = 0;  i < replen;  i++)
		add_linebuf(*rep++, a, 0);

	if (cshift < hshift)
	{
		/* We haven't left-shifted enough yet. */
		if (a == AT_ANSI)
			add_ansi(ch); /* Save ANSI attributes */
		if (linebuf.end > linebuf.print)
		{
			/* Shift left enough to put last byte of this char at print-1. */
			memcpy(&linebuf.buf[0], &linebuf.buf[replen], linebuf.print);
			memcpy(&linebuf.attr[0], &linebuf.attr[replen], linebuf.print);
			linebuf.end -= replen;
			cshift += w;
			/*
			 * If the char we just left-shifted was double width,
			 * the 2 spaces we shifted may be too much.
			 * Represent the "half char" at start of line with a highlighted space.
			 */
			while (cshift > hshift)
			{
				add_linebuf(' ', rscroll_attr, 0);
				cshift--;
			}
		}
	}
	return (0);
}

/*
 * Append a tab to the line buffer.
 * Store spaces to represent the tab.
 */
#define STORE_TAB(a,pos) \
	do { if (store_tab((a),(pos))) return (1); } while (0)

	static int
store_tab(attr, pos)
	int attr;
	POSITION pos;
{
	int to_tab = end_column - linebuf.pfx_end;

	if (ntabstops < 2 || to_tab >= tabstops[ntabstops-1])
		to_tab = tabdefault -
		     ((to_tab - tabstops[ntabstops-1]) % tabdefault);
	else
	{
		int i;
		for (i = ntabstops - 2;  i >= 0;  i--)
			if (to_tab >= tabstops[i])
				break;
		to_tab = tabstops[i+1] - to_tab;
	}

	do {
		STORE_CHAR(' ', attr, " ", pos);
	} while (--to_tab > 0);
	return 0;
}

#define STORE_PRCHAR(c, pos) \
	do { if (store_prchar((c), (pos))) return 1; } while (0)

	static int
store_prchar(c, pos)
	LWCHAR c;
	POSITION pos;
{
	char *s;

	/*
	 * Convert to printable representation.
	 */
	s = prchar(c);
	for ( ;  *s != 0;  s++)
		STORE_CHAR(*s, AT_BINARY|AT_COLOR_CTRL, NULL, pos);
	return 0;
}

	static int
flush_mbc_buf(pos)
	POSITION pos;
{
	int i;

	for (i = 0; i < mbc_buf_index; i++)
		if (store_prchar(mbc_buf[i], pos))
			return mbc_buf_index - i;
	return 0;
}

/*
 * Append a character to the line buffer.
 * Expand tabs into spaces, handle underlining, boldfacing, etc.
 * Returns 0 if ok, 1 if couldn't fit in buffer.
 */
	public int
pappend(c, pos)
	int c;
	POSITION pos;
{
	int r;

	if (pendc)
	{
		if (c == '\r' && pendc == '\r')
			return (0);
		if (do_append(pendc, NULL, pendpos))
			/*
			 * Oops.  We've probably lost the char which
			 * was in pendc, since caller won't back up.
			 */
			return (1);
		pendc = '\0';
	}

	if (c == '\r' && bs_mode == BS_SPECIAL)
	{
		if (mbc_buf_len > 0)  /* utf_mode must be on. */
		{
			/* Flush incomplete (truncated) sequence. */
			r = flush_mbc_buf(mbc_pos);
			mbc_buf_index = r + 1;
			mbc_buf_len = 0;
			if (r)
				return (mbc_buf_index);
		}

		/*
		 * Don't put the CR into the buffer until we see 
		 * the next char.  If the next char is a newline,
		 * discard the CR.
		 */
		pendc = c;
		pendpos = pos;
		return (0);
	}

	if (!utf_mode)
	{
		r = do_append(c, NULL, pos);
	} else
	{
		/* Perform strict validation in all possible cases. */
		if (mbc_buf_len == 0)
		{
		retry:
			mbc_buf_index = 1;
			*mbc_buf = c;
			if (IS_ASCII_OCTET(c))
				r = do_append(c, NULL, pos);
			else if (IS_UTF8_LEAD(c))
			{
				mbc_buf_len = utf_len(c);
				mbc_pos = pos;
				return (0);
			} else
				/* UTF8_INVALID or stray UTF8_TRAIL */
				r = flush_mbc_buf(pos);
		} else if (IS_UTF8_TRAIL(c))
		{
			mbc_buf[mbc_buf_index++] = c;
			if (mbc_buf_index < mbc_buf_len)
				return (0);
			if (is_utf8_well_formed(mbc_buf, mbc_buf_index))
				r = do_append(get_wchar(mbc_buf), mbc_buf, mbc_pos);
			else
				/* Complete, but not shortest form, sequence. */
				mbc_buf_index = r = flush_mbc_buf(mbc_pos);
			mbc_buf_len = 0;
		} else
		{
			/* Flush incomplete (truncated) sequence.  */
			r = flush_mbc_buf(mbc_pos);
			mbc_buf_index = r + 1;
			mbc_buf_len = 0;
			/* Handle new char.  */
			if (!r)
				goto retry;
		}
	}
	if (r)
	{
		/* How many chars should caller back up? */
		r = (!utf_mode) ? 1 : mbc_buf_index;
	}
	return (r);
}

	static int
store_control_char(ch, rep, pos)
	LWCHAR ch;
	char *rep;
	POSITION pos;
{
	if (ctldisp == OPT_ON)
	{
		/* Output the character itself. */
		STORE_CHAR(ch, AT_NORMAL, rep, pos);
	} else 
	{
		/* Output a printable representation of the character. */
		STORE_PRCHAR((char) ch, pos);
	}
	return (0);
}

	static int
store_ansi(ch, rep, pos)
	LWCHAR ch;
	char *rep;
	POSITION pos;
{
	switch (ansi_step(line_ansi, ch))
	{
	case ANSI_MID:
		STORE_CHAR(ch, AT_ANSI, rep, pos);
		break;
	case ANSI_END:
		STORE_CHAR(ch, AT_ANSI, rep, pos);
		ansi_done(line_ansi);
		line_ansi = NULL;
		break;
	case ANSI_ERR: {
		/* Remove whole unrecognized sequence.  */
		char *p = &linebuf.buf[linebuf.end];
		LWCHAR bch;
		do {
			bch = step_char(&p, -1, linebuf.buf);
		} while (p > linebuf.buf && !IS_CSI_START(bch));
		linebuf.end = (int) (p - linebuf.buf);
		break; }
	}
	return (0);
} 

	static int
store_bs(ch, rep, pos)
	LWCHAR ch;
	char *rep;
	POSITION pos;
{
	if (bs_mode == BS_CONTROL)
		return store_control_char(ch, rep, pos);
	if (linebuf.end > 0 &&
		((linebuf.end <= linebuf.print && linebuf.buf[linebuf.end-1] == '\0') ||
	     (linebuf.end > 0 && linebuf.attr[linebuf.end - 1] & (AT_ANSI|AT_BINARY))))
		STORE_PRCHAR('\b', pos);
	else if (bs_mode == BS_NORMAL)
		STORE_CHAR(ch, AT_NORMAL, NULL, pos);
	else if (bs_mode == BS_SPECIAL)
		overstrike = backc();
	return 0;
}

	static int
do_append(ch, rep, pos)
	LWCHAR ch;
	char *rep;
	POSITION pos;
{
	int a = AT_NORMAL;

	if (ctldisp == OPT_ONPLUS && line_ansi == NULL)
		line_ansi = ansi_start(ch);

	if (line_ansi != NULL)
		return store_ansi(ch, rep, pos);

	if (ch == '\b')
		return store_bs(ch, rep, pos);

	if (overstrike > 0)
	{
		/*
		 * Overstrike the character at the current position
		 * in the line buffer.  This will cause either 
		 * underline (if a "_" is overstruck), 
		 * bold (if an identical character is overstruck),
		 * or just replacing the character in the buffer.
		 */
		LWCHAR prev_ch;
		overstrike = utf_mode ? -1 : 0;
		if (utf_mode)
		{
			/* To be correct, this must be a base character.  */
			prev_ch = get_wchar(&linebuf.buf[linebuf.end]);
		} else
		{
			prev_ch = (unsigned char) linebuf.buf[linebuf.end];
		}
		a = linebuf.attr[linebuf.end];
		if (ch == prev_ch)
		{
			/*
			 * Overstriking a char with itself means make it bold.
			 * But overstriking an underscore with itself is
			 * ambiguous.  It could mean make it bold, or
			 * it could mean make it underlined.
			 * Use the previous overstrike to resolve it.
			 */
			if (ch == '_')
			{
				if ((a & (AT_BOLD|AT_UNDERLINE)) != AT_NORMAL)
					a |= (AT_BOLD|AT_UNDERLINE);
				else if (last_overstrike != AT_NORMAL)
					a |= last_overstrike;
				else
					a |= AT_BOLD;
			} else
				a |= AT_BOLD;
		} else if (ch == '_')
		{
			a |= AT_UNDERLINE;
			ch = prev_ch;
			rep = &linebuf.buf[linebuf.end];
		} else if (prev_ch == '_')
		{
			a |= AT_UNDERLINE;
		}
		/* Else we replace prev_ch, but we keep its attributes.  */
	} else if (overstrike < 0)
	{
		if (   is_composing_char(ch)
		    || is_combining_char(get_wchar(&linebuf.buf[linebuf.end]), ch))
			/* Continuation of the same overstrike.  */
			a = last_overstrike;
		else
			overstrike = 0;
	}

	if (ch == '\t')
	{
		/*
		 * Expand a tab into spaces.
		 */
		switch (bs_mode)
		{
		case BS_CONTROL:
			return store_control_char(ch, rep, pos);
		case BS_NORMAL:
		case BS_SPECIAL:
			STORE_TAB(a, pos);
			break;
		}
		return (0);
	}
	if ((!utf_mode || is_ascii_char(ch)) && control_char((char)ch))
	{
		return store_control_char(ch, rep, pos);
	} else if (utf_mode && ctldisp != OPT_ON && is_ubin_char(ch))
	{
		char *s = prutfchar(ch);
		for ( ;  *s != 0;  s++)
			STORE_CHAR(*s, AT_BINARY, NULL, pos);
	} else
	{
		STORE_CHAR(ch, a, rep, pos);
	}
	return (0);
}

/*
 *
 */
	public int
pflushmbc(VOID_PARAM)
{
	int r = 0;

	if (mbc_buf_len > 0)
	{
		/* Flush incomplete (truncated) sequence.  */
		r = flush_mbc_buf(mbc_pos);
		mbc_buf_len = 0;
	}
	return r;
}

/*
 * Switch to normal attribute at end of line.
 */
	static void
add_attr_normal(VOID_PARAM)
{
	char *p = "\033[m";

	if (ctldisp != OPT_ONPLUS || !is_ansi_end('m'))
		return;
	for ( ;  *p != '\0';  p++)
		add_linebuf(*p, AT_ANSI, 0);
}

/*
 * Terminate the line in the line buffer.
 */
	public void
pdone(endline, chopped, forw)
	int endline;
	int chopped;
	int forw;
{
	(void) pflushmbc();

	if (pendc && (pendc != '\r' || !endline))
		/*
		 * If we had a pending character, put it in the buffer.
		 * But discard a pending CR if we are at end of line
		 * (that is, discard the CR in a CR/LF sequence).
		 */
		(void) do_append(pendc, NULL, pendpos);

	if (chopped && rscroll_char)
	{
		/*
		 * Display the right scrolling char.
		 * If we've already filled the rightmost screen char 
		 * (in the buffer), overwrite it.
		 */
		if (end_column >= sc_width + cshift)
		{
			/* We've already written in the rightmost char. */
			end_column = right_column;
			linebuf.end = right_curr;
		}
		add_attr_normal();
		while (end_column < sc_width-1 + cshift) 
		{
			/*
			 * Space to last (rightmost) char on screen.
			 * This may be necessary if the char we overwrote
			 * was double-width.
			 */
			add_linebuf(' ', rscroll_attr, 1);
		}
		/* Print rscroll char. It must be single-width. */
		add_linebuf(rscroll_char, rscroll_attr, 1);
	} else
	{
		add_attr_normal();
	}

	/*
	 * Add a newline if necessary,
	 * and append a '\0' to the end of the line.
	 * We output a newline if we're not at the right edge of the screen,
	 * or if the terminal doesn't auto wrap,
	 * or if this is really the end of the line AND the terminal ignores
	 * a newline at the right edge.
	 * (In the last case we don't want to output a newline if the terminal 
	 * doesn't ignore it since that would produce an extra blank line.
	 * But we do want to output a newline if the terminal ignores it in case
	 * the next line is blank.  In that case the single newline output for
	 * that blank line would be ignored!)
	 */
	if (end_column < sc_width + cshift || !auto_wrap || (endline && ignaw) || ctldisp == OPT_ON)
	{
		add_linebuf('\n', AT_NORMAL, 0);
	} 
	else if (ignaw && end_column >= sc_width + cshift && forw)
	{
		/*
		 * Terminals with "ignaw" don't wrap until they *really* need
		 * to, i.e. when the character *after* the last one to fit on a
		 * line is output. But they are too hard to deal with when they
		 * get in the state where a full screen width of characters
		 * have been output but the cursor is sitting on the right edge
		 * instead of at the start of the next line.
		 * So we nudge them into wrapping by outputting a space 
		 * character plus a backspace.  But do this only if moving 
		 * forward; if we're moving backward and drawing this line at
		 * the top of the screen, the space would overwrite the first
		 * char on the next line.  We don't need to do this "nudge" 
		 * at the top of the screen anyway.
		 */
		add_linebuf(' ', AT_NORMAL, 1);
		add_linebuf('\b', AT_NORMAL, -1);
	}
	set_linebuf(linebuf.end, '\0', AT_NORMAL);
}

/*
 *
 */
	public void
set_status_col(c, attr)
	int c;
	int attr;
{
	set_pfx(0, c, attr);
}

/*
 * Get a character from the current line.
 * Return the character as the function return value,
 * and the character attribute in *ap.
 */
	public int
gline(i, ap)
	int i;
	int *ap;
{
	if (is_null_line)
	{
		/*
		 * If there is no current line, we pretend the line is
		 * either "~" or "", depending on the "twiddle" flag.
		 */
		if (twiddle)
		{
			if (i == 0)
			{
				*ap = AT_BOLD;
				return '~';
			}
			--i;
		}
		/* Make sure we're back to AT_NORMAL before the '\n'.  */
		*ap = AT_NORMAL;
		return i ? '\0' : '\n';
	}

	if (i < linebuf.pfx_end)
	{
		*ap = linebuf.pfx_attr[i];
		return linebuf.pfx[i];
	}
	i += linebuf.print - linebuf.pfx_end;
	*ap = linebuf.attr[i];
	return (linebuf.buf[i] & 0xFF);
}

/*
 * Indicate that there is no current line.
 */
	public void
null_line(VOID_PARAM)
{
	is_null_line = 1;
	cshift = 0;
}

/*
 * Analogous to forw_line(), but deals with "raw lines":
 * lines which are not split for screen width.
 * {{ This is supposed to be more efficient than forw_line(). }}
 */
	public POSITION
forw_raw_line(curr_pos, linep, line_lenp)
	POSITION curr_pos;
	char **linep;
	int *line_lenp;
{
	int n;
	int c;
	POSITION new_pos;

	if (curr_pos == NULL_POSITION || ch_seek(curr_pos) ||
		(c = ch_forw_get()) == EOI)
		return (NULL_POSITION);

	n = 0;
	for (;;)
	{
		if (c == '\n' || c == EOI || ABORT_SIGS())
		{
			new_pos = ch_tell();
			break;
		}
		if (n >= size_linebuf-1)
		{
			if (expand_linebuf())
			{
				/*
				 * Overflowed the input buffer.
				 * Pretend the line ended here.
				 */
				new_pos = ch_tell() - 1;
				break;
			}
		}
		linebuf.buf[n++] = c;
		c = ch_forw_get();
	}
	linebuf.buf[n] = '\0';
	if (linep != NULL)
		*linep = linebuf.buf;
	if (line_lenp != NULL)
		*line_lenp = n;
	return (new_pos);
}

/*
 * Analogous to back_line(), but deals with "raw lines".
 * {{ This is supposed to be more efficient than back_line(). }}
 */
	public POSITION
back_raw_line(curr_pos, linep, line_lenp)
	POSITION curr_pos;
	char **linep;
	int *line_lenp;
{
	int n;
	int c;
	POSITION new_pos;

	if (curr_pos == NULL_POSITION || curr_pos <= ch_zero() ||
		ch_seek(curr_pos-1))
		return (NULL_POSITION);

	n = size_linebuf;
	linebuf.buf[--n] = '\0';
	for (;;)
	{
		c = ch_back_get();
		if (c == '\n' || ABORT_SIGS())
		{
			/*
			 * This is the newline ending the previous line.
			 * We have hit the beginning of the line.
			 */
			new_pos = ch_tell() + 1;
			break;
		}
		if (c == EOI)
		{
			/*
			 * We have hit the beginning of the file.
			 * This must be the first line in the file.
			 * This must, of course, be the beginning of the line.
			 */
			new_pos = ch_zero();
			break;
		}
		if (n <= 0)
		{
			int old_size_linebuf = size_linebuf;
			char *fm;
			char *to;
			if (expand_linebuf())
			{
				/*
				 * Overflowed the input buffer.
				 * Pretend the line ended here.
				 */
				new_pos = ch_tell() + 1;
				break;
			}
			/*
			 * Shift the data to the end of the new linebuf.
			 */
			for (fm = linebuf.buf + old_size_linebuf - 1,
			      to = linebuf.buf + size_linebuf - 1;
			     fm >= linebuf.buf;  fm--, to--)
				*to = *fm;
			n = size_linebuf - old_size_linebuf;
		}
		linebuf.buf[--n] = c;
	}
	if (linep != NULL)
		*linep = &linebuf.buf[n];
	if (line_lenp != NULL)
		*line_lenp = size_linebuf - 1 - n;
	return (new_pos);
}

/*
 * Find the shift necessary to show the end of the longest displayed line.
 */
	public int
rrshift(VOID_PARAM)
{
	POSITION pos;
	int save_width;
	int line;
	int longest = 0;

	save_width = sc_width;
	sc_width = INT_MAX;
	pos = position(TOP);
	for (line = 0; line < sc_height && pos != NULL_POSITION; line++)
	{
		pos = forw_line(pos);
		if (end_column > longest)
			longest = end_column;
	}
	sc_width = save_width;
	if (longest < sc_width)
		return 0;
	return longest - sc_width;
}

/*
 * Get the color_map index associated with a given attribute.
 */
	static int
color_index(attr)
	int attr;
{
	if (use_color)
	{
		switch (attr & AT_COLOR)
		{
		case AT_COLOR_ATTN:    return 0;
		case AT_COLOR_BIN:     return 1;
		case AT_COLOR_CTRL:    return 2;
		case AT_COLOR_ERROR:   return 3;
		case AT_COLOR_LINENUM: return 4;
		case AT_COLOR_MARK:    return 5;
		case AT_COLOR_PROMPT:  return 6;
		case AT_COLOR_RSCROLL: return 7;
		case AT_COLOR_SEARCH:  return 8;
		}
	}
	if (attr & AT_UNDERLINE)
		return 9;
	if (attr & AT_BOLD)
		return 10;
	if (attr & AT_BLINK)
		return 11;
	if (attr & AT_STANDOUT)
		return 12;
	return -1;
}

/*
 * Set the color string to use for a given attribute.
 */
	public int
set_color_map(attr, colorstr)
	int attr;
	char *colorstr;
{
	int cx = color_index(attr);
	if (cx < 0)
		return -1;
	if (strlen(colorstr)+1 > sizeof(color_map[cx]))
		return -1;
	if (*colorstr != '\0' && parse_color(colorstr, NULL, NULL) == CT_NULL)
		return -1;
	strcpy(color_map[cx], colorstr);
	return 0;
}

/*
 * Get the color string to use for a given attribute.
 */
	public char *
get_color_map(attr)
	int attr;
{
	int cx = color_index(attr);
	if (cx < 0)
		return NULL;
	return color_map[cx];
}
