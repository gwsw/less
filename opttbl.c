/*
 * Copyright (c) 1984,1985,1989,1994,1995,1996,1999  Mark Nudelman
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
 * The option table.
 */

#include "less.h"
#include "option.h"

/*
 * Variables controlled by command line options.
 */
public int quiet;		/* Should we suppress the audible bell? */
public int how_search;		/* Where should forward searches start? */
public int top_scroll;		/* Repaint screen from top?
				   (alternative is scroll from bottom) */
public int pr_type;		/* Type of prompt (short, medium, long) */
public int bs_mode;		/* How to process backspaces */
public int know_dumb;		/* Don't complain about dumb terminals */
public int quit_at_eof;		/* Quit after hitting end of file twice */
public int squeeze;		/* Squeeze multiple blank lines into one */
public int tabstop;		/* Tab settings */
public int back_scroll;		/* Repaint screen on backwards movement */
public int forw_scroll;		/* Repaint screen on forward movement */
public int caseless;		/* Do "caseless" searches */
public int linenums;		/* Use line numbers */
public int cbufs;		/* Current number of buffers */
public int autobuf;		/* Automatically allocate buffers as needed */
public int ctldisp;		/* Send control chars to screen untranslated */
public int force_open;		/* Open the file even if not regular file */
public int swindow;		/* Size of scrolling window */
public int jump_sline;		/* Screen line of "jump target" */
public int chopline;		/* Truncate displayed lines at screen width */
public int no_init;		/* Disable sending ti/te termcap strings */
public int twiddle;             /* Show tildes after EOF */
public int show_attn;		/* Hilite first unread line */
#if HILITE_SEARCH
public int hilite_search;	/* Highlight matched search patterns? */
#endif

/*
 * Table of all options and their semantics.
 */
static struct option option[] =
{
	{ 'a', "search-skip-screen",
		BOOL, OPT_OFF, &how_search, NULL,
		"Search includes displayed screen",
		"Search skips displayed screen",
		NULL
	},
	{ 'b', "buffers",
		NUMBER, 10, &cbufs, opt_b, 
		"Buffers: ",
		"%d buffers",
		NULL
	},
	{ 'B', "auto-buffers",
		BOOL, OPT_ON, &autobuf, NULL,
		"Don't automatically allocate buffers",
		"Automatically allocate buffers when needed",
		NULL
	},
	{ 'c', "clear-screen",
		TRIPLE, OPT_OFF, &top_scroll, NULL,
		"Repaint by scrolling from bottom of screen",
		"Repaint by clearing each line",
		"Repaint by painting from top of screen"
	},
	{ 'd', "dumb",
		BOOL|NO_TOGGLE, OPT_OFF, &know_dumb, NULL,
		"Assume intelligent terminal",
		"Assume dumb terminal",
		NULL
	},
#if MSDOS_COMPILER
	{ 'D', "color",
		STRING|REPAINT|NO_QUERY, 0, NULL, opt_D,
		"color desc: ", NULL, NULL
	},
#endif
	{ 'e', "quit-at-eof",
		TRIPLE, OPT_OFF, &quit_at_eof, NULL,
		"Don't quit at end-of-file",
		"Quit at end-of-file",
		"Quit immediately at end-of-file"
	},
	{ 'f', "force",
		BOOL, OPT_OFF, &force_open, NULL,
		"Open only regular files",
		"Open even non-regular files",
		NULL
	},
#if HILITE_SEARCH
	{ 'g', "hilite-search",
		TRIPLE|HL_REPAINT, OPT_ONPLUS, &hilite_search, NULL,
		"Don't highlight search matches",
		"Highlight matches for previous search only",
		"Highlight all matches for previous search pattern",
	},
#endif
	{ 'h', "scroll-back-limit",
		NUMBER, -1, &back_scroll, NULL,
		"Backwards scroll limit: ",
		"Backwards scroll limit is %d lines",
		NULL
	},
	{ 'i', "ignore-case",
		TRIPLE|HL_REPAINT, OPT_OFF, &caseless, opt_i,
		"Case is significant in searches",
		"Ignore case in searches",
		"Ignore case in searches and in patterns"
	},
	{ 'j', "jump-target",
		NUMBER, 1, &jump_sline, NULL,
		"Target line: ",
		"Position target at screen line %d",
		NULL
	},
#if USERFILE
	{ 'k', "lesskey-file",
		STRING|NO_TOGGLE|NO_QUERY, 0, NULL, opt_k,
		NULL, NULL, NULL
	},
#endif
	{ 'l', NULL,
		STRING|NO_TOGGLE|NO_QUERY, 0, NULL, opt_l,
		NULL, NULL, NULL
	},
	{ 'm', "long-prompt",
		TRIPLE, OPT_OFF, &pr_type, NULL,
		"Short prompt",
		"Medium prompt",
		"Long prompt"
	},
	{ 'n', "line-numbers",
		TRIPLE|REPAINT, OPT_ON, &linenums, NULL,
		"Don't use line numbers",
		"Use line numbers",
		"Constantly display line numbers"
	},
#if LOGFILE
	{ 'o', "log-file",
		STRING, 0, NULL, opt_o,
		"log file: ", NULL, NULL
	},
	{ 'O', "LOG-FILE",
		STRING, 0, NULL, opt__O,
		"Log file: ", NULL, NULL
	},
#endif
	{ 'p', "pattern",
		STRING|NO_TOGGLE|NO_QUERY, 0, NULL, opt_p,
		NULL, NULL, NULL
	},
	{ 'P', "prompt",
		STRING, 0, NULL, opt__P,
		"prompt: ", NULL, NULL
	},
	{ 'q', "quiet",
		TRIPLE, OPT_OFF, &quiet, NULL,
		"Ring the bell for errors AND at eof/bof",
		"Ring the bell for errors but not at eof/bof",
		"Never ring the bell"
	},
	{ 'r', "raw-control-chars",
		BOOL|REPAINT, OPT_ON, &ctldisp, NULL,
		"Display control characters directly",
		"Display control characters as ^X",
		NULL
	},
	{ 's', "squeeze-blank-lines",
		BOOL|REPAINT, OPT_OFF, &squeeze, NULL,
		"Display all blank lines",
		"Squeeze multiple blank lines",
		NULL
	},
	{ 'S', "chop-long-lines",
		BOOL|REPAINT, OPT_OFF, &chopline, NULL,
		"Fold long lines",
		"Chop long lines",
		NULL
	},
#if TAGS
	{ 't', "tag",
		STRING|NO_QUERY, 0, NULL, opt_t,
		"tag: ", NULL, NULL
	},
	{ 'T', "tag-file",
		STRING, 0, NULL, opt__T,
		"tags file: ", NULL, NULL
	},
#endif
	{ 'u', "underline-special",
		TRIPLE|REPAINT, OPT_OFF, &bs_mode, NULL,
		"Display underlined text in underline mode",
		"Backspaces cause overstrike",
		"Print backspace as ^H"
	},
	{ 'V', "version",
		NOVAR, 0, NULL, opt__V,
		NULL, NULL, NULL
	},
	{ 'w', "hilite-unread",
		TRIPLE|REPAINT, OPT_OFF, &show_attn, NULL,
		"Don't highlight first unread line",
		"Highlight first unread line after forward-screen",
		"Highlight first unread line after any forward movement",
	},
	{ 'x', "tabs",
		NUMBER|REPAINT, 8, &tabstop, NULL,
		"Tab stops: ",
		"Tab stops every %d spaces", 
		NULL
	},
	{ 'X', "noinit",
		BOOL|NO_TOGGLE, OPT_OFF, &no_init, NULL,
		"Send init/deinit strings to terminal",
		"Don't use init/deinit strings",
		NULL
	},
	{ 'y', "scroll-forw-limit",
		NUMBER, -1, &forw_scroll, NULL,
		"Forward scroll limit: ",
		"Forward scroll limit is %d lines",
		NULL
	},
	{ 'z', "window",
		NUMBER, -1, &swindow, NULL,
		"Scroll window size: ",
		"Scroll window size is %d lines",
		NULL
	},
	{ '"', "quotes",
		STRING, 0, NULL, opt_quote,
		"quotes: ", NULL, NULL
	},
	{ '~', "tilde",
		BOOL|REPAINT, OPT_ON, &twiddle, NULL,
		"Don't show tildes after end of file",
		"Show tildes after end of file",
		NULL
	},
	{ '?', "help",
		NOVAR, 0, NULL, opt_query,
		NULL, NULL, NULL
	},
	{ '\0', NULL, NOVAR, 0, NULL, NULL, NULL, NULL, NULL }
};


/*
 * Initialize each option to its default value.
 */
	public void
init_option()
{
	register struct option *o;

	for (o = option;  o->oletter != '\0';  o++)
	{
		/*
		 * Set each variable to its default.
		 */
		if (o->ovar != NULL)
			*(o->ovar) = o->odefault;
	}
}

/*
 * Find an option in the option table, given its option letter.
 */
	public struct option *
findopt(c)
	int c;
{
	register struct option *o;

	for (o = option;  o->oletter != '\0';  o++)
	{
		if (o->oletter == c)
			return (o);
		if ((o->otype & TRIPLE) && toupper(o->oletter) == c)
			return (o);
	}
	return (NULL);
}

/*
 * Find an option in the option table, given its option name.
 */
	public struct option *
findopt_name(p_optname, p_err)
	char **p_optname;
	int *p_err;
{
	char *optname = *p_optname;
	register struct option *o;
	register int len;
	int uppercase;
	struct option *maxo = NULL;
	int maxlen = 0;
	int ambig = 0;
	int exact = 0;

	for (o = option;  o->oletter != '\0';  o++)
	{
		if (o->oname == NULL)
			continue;
		/* 
		 * Try normal match first (uppercase == 0),
		 * then, then if it's a TRIPLE option,
		 * try uppercase match (uppercase == 1).
		 */
		for (uppercase = 0;  uppercase <= 1;  uppercase++)
		{
			len = sprefix(optname, o->oname, uppercase);
			if (!exact && len == maxlen)
				ambig = 1;
			else if (len > maxlen)
			{
				maxo = o;
				maxlen = len;
				ambig = 0;
				exact = (len == strlen(o->oname));
			}

			if (!(o->otype & TRIPLE))
				break;
		}
	}
	if (ambig)
	{
		if (p_err != NULL)
			*p_err = OPT_AMBIG;
		return (NULL);
	}
	*p_optname = optname + maxlen;
	return (maxo);
}
