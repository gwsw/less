/*
 * Copyright (c) 1984,1985,1989,1994,1995  Mark Nudelman
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice in the documentation and/or other materials provided with 
 *    the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR 
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR 
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT 
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR 
 * BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, 
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE 
 * OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN 
 * IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */


/*
 * Routines which deal with the characteristics of the terminal.
 *
 * This file is specific to MS-DOS and uses Microsoft C graphics functions.
 */

#include "less.h"
#include "cmd.h"

#if MSDOS_COMPILER==MSOFTC
#include <graph.h>
#else
#if MSDOS_COMPILER==BORLANDC
#include <conio.h>
#endif
#endif
#include <time.h>

#if MSDOS_COMPILER==BORLANDC
static unsigned short *whitescreen;
#define _settextposition(y,x)	gotoxy(x,y)
#define _settextcolor(c)	textcolor(c)
#define _setbkcolor(c)		textbackground(c)
#define _clearscreen(m)		clrscr()
#define _outtext(s)		cputs(s)
#endif

static int init_done = 0;
static int flash_created = 0;
static int videopages;
static long msec_loops;

public int auto_wrap;		/* Terminal does \r\n when write past margin */
public int ignaw;		/* Terminal ignores \n immediately after wrap */
public int erase_char, kill_char; /* The user's erase and line-kill chars */
public int sc_width, sc_height;	/* Height & width of screen */
public int bo_s_width, bo_e_width;	/* Printing width of boldface seq */
public int ul_s_width, ul_e_width;	/* Printing width of underline seq */
public int so_s_width, so_e_width;	/* Printing width of standout seq */
public int bl_s_width, bl_e_width;	/* Printing width of blink seq */
public int can_goto_line;		/* Can move cursor to any line */

public int nm_fg_color = 7;			/* Color of normal text */
public int nm_bg_color = 0;
public int bo_fg_color = 15;		/* Color of bold text */
public int bo_bg_color = 0;
public int ul_fg_color = 9;			/* Color of underlined text */
public int ul_bg_color = 0;
public int so_fg_color = 0;			/* Color of standout text */
public int so_bg_color = 7;
public int bl_fg_color = 12;		/* Color of blinking text */
public int bl_bg_color = 0;

static int sy_fg_color;
static int sy_bg_color;

extern int quiet;		/* If VERY_QUIET, use visual bell for bell */
extern int know_dumb;		/* Don't complain about a dumb terminal */
extern int back_scroll;
extern int swindow;

/*
 * Change terminal to "raw mode", or restore to "normal" mode.
 * "Raw mode" means 
 *	1. An outstanding read will complete on receipt of a single keystroke.
 *	2. Input is not echoed.  
 *	3. On output, \n is mapped to \r\n.
 *	4. \t is NOT expanded into spaces.
 *	5. Signal-causing characters such as ctrl-C (interrupt),
 *	   etc. are NOT disabled.
 * It doesn't matter whether an input \n is mapped to \r, or vice versa.
 */
	public void
raw_mode(on)
	int on;
{
	static int curr_on = 0;

	if (on == curr_on)
		return;
	erase_char = CONTROL('h');
	kill_char = '\33'; /* ESC */
	curr_on = on;
}

/*
 * Get size of the output screen.
 */
	public void
scrsize(p_height, p_width)
	int *p_height;
	int *p_width;
{
	register int height;
	register int width;
	register char *s;

#if MSDOS_COMPILER==MSOFTC
	struct videoconfig w;

	_getvideoconfig(&w);
	height = w.numtextrows;
	width = w.numtextcols;
#else
#if MSDOS_COMPILER==BORLANDC
	struct text_info w;

	gettextinfo(&w);
	height = w.screenheight;
	width = w.screenwidth;
#endif
#endif
	if (height)
		*p_height = height;
	else if ((s = lgetenv("LINES")) != NULL && *s != '\0')
		*p_height = atoi(s);
	if (*p_height <= 0)
		*p_height = 24;
		
	if (width > 0)
		*p_width = width;
	else if ((s = lgetenv("COLUMNS")) != NULL)
		*p_width = atoi(s);
	if (*p_width <= 0)
  		*p_width = 80;
}

#if MSDOS_COMPILER==MSOFTC
/*
 * Figure out how many empty loops it takes to delay a millisecond.
 */
	static void
get_clock()
{
	clock_t start;
	
	/*
	 * Get synchronized at the start of a tick.
	 */
	start = clock();
	while (clock() == start)
		;
	/*
	 * Now count loops till the next tick.
	 */
	start = clock();
	msec_loops = 0;
	while (clock() == start)
		msec_loops++;
	/*
	 * Convert from (loops per clock) to (loops per millisecond).
	 */
	msec_loops *= CLOCKS_PER_SEC;
	msec_loops /= 1000;
}

/*
 * Delay for a specified number of milliseconds.
 */
	static void
dummy_func()
{
	static long delay_dummy = 0;
	delay_dummy++;
}

	static void
delay(msec)
	int msec;
{
	long i;
	
	while (msec-- > 0)
	{
		for (i = 0;  i < msec_loops;  i++)
		{
			/*
			 * Make it look like we're doing something here,
			 * so the optimizer doesn't remove the whole loop.
			 */
			dummy_func();
		}
	}
}
#endif

/*
 * Get terminal capabilities via termcap.
 */
	public void
get_term()
{
	scrsize(&sc_height, &sc_width);
	pos_init();
	auto_wrap = 1;
	ignaw = 0;
	so_e_width = so_s_width = 0;
	bo_s_width = bo_e_width = 0;
	ul_s_width = ul_e_width = 0;
	bl_s_width = bl_e_width = 0;
#if MSDOS_COMPILER==MSOFTC
	get_clock();
#endif
}


/*
 * Below are the functions which perform all the 
 * terminal-specific screen manipulation.
 */


/*
 * Initialize the screen to the correct color at startup.
 */
	static void
initcolor()
{
	_settextcolor(nm_fg_color);
	_setbkcolor(nm_bg_color);

#if 0
	/*
	 * This clears the screen at startup.  This is different from
	 * the behavior of other versions of less.  Disable it for now.
	 */
	int height;
	int width;
	char *blanks;
	int row;
	int col;
	
	/*
	 * Create a complete, blank screen using "normal" colors.
	 */
	_settextcolor(nm_fg_color);
	_setbkcolor(nm_bg_color);
	scrsize(&height, &width);
	blanks = (char *) ecalloc(width+1, sizeof(char));
	for (col = 0;  col < width;  col++)
		blanks[col] = ' ';
	blanks[width] = '\0';
	for (row = 0;  row < height;  row++)
		_outtext(blanks);
	free(blanks);
#endif
}

/*
 * Initialize terminal
 */
	public void
init()
{
	/* {{ What could we take no_init (-X) to mean? }} */
#if MSDOS_COMPILER==MSOFTC
	sy_bg_color = _getbkcolor();
	sy_fg_color = _gettextcolor();
#else
#if MSDOS_COMPILER==BORLANDC
	struct text_info w;

	gettextinfo(&w);
	sy_bg_color = (w.attribute >> 4) & 0x0F;
	sy_fg_color = (w.attribute >> 0) & 0x0F;
#endif
#endif
	initcolor();
	flush();
	init_done = 1;
}

/*
 * Create an alternate screen which is all white.
 * This screen is used to create a "flash" effect, by displaying it
 * briefly and then switching back to the normal screen.
 * {{ Yuck!  There must be a better way to get a visual bell. }}
 */
	static void
create_flash()
{
#if MSDOS_COMPILER==MSOFTC
	struct videoconfig w;
	char *blanks;
	int row, col;
	
	_getvideoconfig(&w);
	videopages = w.numvideopages;
	if (videopages < 2)
	{
		so_enter();
		so_exit();
	} else
	{
		_setactivepage(1);
		so_enter();
		blanks = (char *) ecalloc(w.numtextcols, sizeof(char));
		for (col = 0;  col < w.numtextcols;  col++)
			blanks[col] = ' ';
		for (row = w.numtextrows;  row > 0;  row--)
			_outmem(blanks, w.numtextcols);
		_setactivepage(0);
		_setvisualpage(0);
		free(blanks);
		so_exit();
	}
#else
#if MSDOS_COMPILER==BORLANDC
	register int n;

	whitescreen = (unsigned short *) 
		malloc(sc_width * sc_height * sizeof(short));
	if (whitescreen == NULL)
		return;
	for (n = 0;  n < sc_width * sc_height;  n++)
		whitescreen[n] = 0x7020;
#endif
#endif
	flash_created = 1;
}

/*
 * Deinitialize terminal
 */
	public void
deinit()
{
	if (!init_done)
		return;
	_setbkcolor(sy_bg_color);
	_settextcolor(sy_fg_color);
	putstr("\n");
	init_done = 0;
}

/*
 * Home cursor (move to upper left corner of screen).
 */
	public void
home()
{
	flush();
	_settextposition(1,1);
}

/*
 * Add a blank line (called with cursor at home).
 * Should scroll the display down.
 */
	public void
add_line()
{
	flush();
#if MSDOS_COMPILER==MSOFTC
	_scrolltextwindow(_GSCROLLDOWN);
	_settextposition(1,1);
#else
#if MSDOS_COMPILER==BORLANDC
	movetext(1,1, sc_width,sc_height-1, 1,2);
	gotoxy(1,1);
	clreol();
#endif
#endif
}

/*
 * Move cursor to lower left corner of screen.
 */
	public void
lower_left()
{
	flush();
	_settextposition(sc_height, 1);
}

/*
 * Goto a specific line on the screen.
 */
	public void
goto_line(slinenum)
	int slinenum;
{
	flush();
	_settextposition(slinenum, 1);
}


/*
 * Make a noise.
 */
	static void
beep()
{
	write(1, "\7", 1);
}

/*
 * Output the "visual bell", if there is one.
 */
	public void
vbell()
{
#if MSDOS_COMPILER==MSOFTC
	if (!flash_created)
		/*
		 * Create a "flash" on the second video page.
		 */
		create_flash();
	if (videopages < 2)
		/*
		 * There is no "second video page".
		 */
		return;
	_setvisualpage(1);
	/*
	 * Leave it displayed for 100 msec.
	 */
	delay(100);
	_setvisualpage(0);
#else
#if MSDOS_COMPILER==BORLANDC
	unsigned short *currscreen;

	/*
	 * Get a copy of the current screen.
	 * Display an all white screen.
	 * Then restore the old screen.
	 */
	if (!flash_created)
		create_flash();

	currscreen = (unsigned short *) 
		malloc(sc_width * sc_height * sizeof(short));
	if (currscreen == NULL || whitescreen == NULL)
	{
		beep();
		return;
	}
	gettext(1, 1, sc_width, sc_height, currscreen);
	puttext(1, 1, sc_width, sc_height, whitescreen);
	delay(100);
	puttext(1, 1, sc_width, sc_height, currscreen);
	free(currscreen);
#endif
#endif
}

/*
 * Ring the terminal bell.
 */
	public void
bell()
{
	if (quiet == VERY_QUIET)
		vbell();
	else
		beep();
}

/*
 * Clear the screen.
 */
	public void
clear()
{
	flush();
	_clearscreen(_GCLEARSCREEN);
}

/*
 * Clear from the cursor to the end of the cursor's line.
 * {{ This must not move the cursor. }}
 */
	public void
clear_eol()
{
#if MSDOS_COMPILER==MSOFTC
	short top, left;
	short bot, right;
	struct rccoord tpos;
	
	flush();
	/*
	 * Save current state.
	 */
	tpos = _gettextposition();
	_gettextwindow(&top, &left, &bot, &right);
	/*
	 * Set a temporary window to the current line,
	 * from the cursor's position to the right edge of the screen.
	 * Then clear that window.
	 */
	_settextwindow(tpos.row, tpos.col, tpos.row, sc_width);
	_clearscreen(_GWINDOW);
	/*
	 * Restore state.
	 */
	_settextwindow(top, left, bot, right);
	_settextposition(tpos.row, tpos.col);
#else
#if MSDOS_COMPILER==BORLANDC
	flush();
	clreol();
#endif
#endif
}

/*
 * Clear the bottom line of the display.
 * Leave the cursor at the beginning of the bottom line.
 */
	public void
clear_bot()
{
	lower_left();
	clear_eol();
}

/*
 * Begin "standout" (bold, underline, or whatever).
 */
	public void
so_enter()
{
	flush();
	_setbkcolor(so_bg_color);
	_settextcolor(so_fg_color);
}

/*
 * End "standout".
 */
	public void
so_exit()
{
	flush();
	_setbkcolor(nm_bg_color);
	_settextcolor(nm_fg_color);
}

/*
 * Begin "underline" (hopefully real underlining, 
 * otherwise whatever the terminal provides).
 */
	public void
ul_enter()
{
	flush();
	_setbkcolor(ul_bg_color);
	_settextcolor(ul_fg_color);
}

/*
 * End "underline".
 */
	public void
ul_exit()
{
	flush();
	_setbkcolor(nm_bg_color);
	_settextcolor(nm_fg_color);
}

/*
 * Begin "bold"
 */
	public void
bo_enter()
{
	flush();
	_setbkcolor(bo_bg_color);
	_settextcolor(bo_fg_color);
}

/*
 * End "bold".
 */
	public void
bo_exit()
{
	flush();
	_setbkcolor(nm_bg_color);
	_settextcolor(nm_fg_color);
}

/*
 * Begin "blink"
 */
	public void
bl_enter()
{
	flush();
	_setbkcolor(bl_bg_color);
	_settextcolor(bl_fg_color);
}

/*
 * End "blink".
 */
	public void
bl_exit()
{
	flush();
	_setbkcolor(nm_bg_color);
	_settextcolor(nm_fg_color);
}

/*
 * Erase the character to the left of the cursor 
 * and move the cursor left.
 */
	public void
backspace()
{
#if MSDOS_COMPILER==MSOFTC
	struct rccoord tpos;
	
	/* 
	 * Erase the previous character by overstriking with a space.
	 */
	flush();
	tpos = _gettextposition();
	if (tpos.col <= 1)
		return;
	_settextposition(tpos.row, tpos.col-1);
	_outtext(" ");
	_settextposition(tpos.row, tpos.col-1);
#else
#if MSDOS_COMPILER==BORLANDC
	cputs("\b");
#endif
#endif
}

/*
 * Output a plain backspace, without erasing the previous char.
 */
	public void
putbs()
{
	int row, col;

	flush();
	{
#if MSDOS_COMPILER==MSOFTC
		struct rccoord tpos;
	
		tpos = _gettextposition();
		row = tpos.row;
		col = tpos.col;
#else
#if MSDOS_COMPILER==BORLANDC
		row = wherey();
		col = wherex();
#endif
#endif
	}
	if (col <= 1)
		return;
	_settextposition(row, col-1);
}

/*
 * Table of line editting characters, for editchar() in decode.c.
 */
char kecmdtable[] = {
	'\340','\115',0,	EC_RIGHT,	/* RIGHTARROW */
	'\340','\113',0,	EC_LEFT,	/* LEFTARROW */
	'\340','\163',0,	EC_W_LEFT,	/* CTRL-LEFTARROW */
	'\340','\164',0,	EC_W_RIGHT,	/* CTRL-RIGHTARROW */
	'\340','\122',0,	EC_INSERT,	/* INSERT */
	'\340','\123',0,	EC_DELETE,	/* DELETE */
	'\340','\223',0,	EC_W_DELETE,	/* CTRL-DELETE */
	'\177',0,		EC_W_BACKSPACE,	/* CTRL-BACKSPACE */
	'\340','\107',0,	EC_HOME,	/* HOME */
	'\340','\117',0,	EC_END,		/* END */
	'\340','\110',0,	EC_UP,		/* UPARROW */
	'\340','\120',0,	EC_DOWN,	/* DOWNARROW */
	'\t',0,			EC_F_COMPLETE,	/* TAB */
	'\17',0,		EC_B_COMPLETE,	/* BACKTAB (?) */
	'\340','\17',0,		EC_B_COMPLETE,	/* BACKTAB */
	'\14',0,		EC_EXPAND,	/* CTRL-L */
	0  /* Extra byte to terminate; subtracted from size, below */
};

int sz_kecmdtable = sizeof(kecmdtable) -1;


char kfcmdtable[] =
{
	/*
	 * PC function keys.
	 * Note that '\0' is converted to '\340' on input.
	 */
	'\340','\120',0,		A_F_LINE,		/* down arrow */
	'\340','\121',0,		A_F_SCREEN,		/* page down */
	'\340','\110',0,		A_B_LINE,		/* up arrow */
	'\340','\111',0,		A_B_SCREEN,		/* page up */
	'\340','\107',0,		A_GOLINE,		/* home */
	'\340','\117',0,		A_GOEND,		/* end */
	'\340','\073',0,		A_HELP,			/* F1 */
	'\340','\022',0,		A_EXAMINE,		/* Alt-E */
	0
};
int sz_kfcmdtable = sizeof(kfcmdtable) - 1;

	public void
get_editkeys()
{
	/*
	 * Register the two tables.
	 */
	add_fcmd_table(kfcmdtable, sz_kfcmdtable);
	add_ecmd_table(kecmdtable, sz_kecmdtable);
}

