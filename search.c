/*
 * Copyright (c) 1984,1985,1989,1994  Mark Nudelman
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
 * Routines to search a file for a pattern.
 */

#include "less.h"
#include "position.h"

#if HAVE_POSIX_REGCOMP
#include <regex.h>
#endif
#if HAVE_RE_COMP
char *re_comp();
int re_exec();
#endif
#if HAVE_REGCMP
char *regcmp();
char *regex();
extern char *__loc1;
#endif
#if HAVE_V8_REGCOMP
#include "regexp.h"
#endif
#if NO_REGEX
static int match();
#endif

extern int sigs;
extern int how_search;
extern int caseless;
extern int linenums;
extern int sc_height;
extern int jump_sline;
extern int bs_mode;
#if HILITE_SEARCH
extern int hilite_search;
static int hide_hilite;
extern int screen_trashed;

struct hilite
{
	struct hilite *hl_next;
	POSITION hl_startpos;
	POSITION hl_endpos;
};
static struct hilite *hilite_list = NULL;
static struct hilite *hilite_tail = NULL;
#endif

/*
 * These are the static variables that represent the "remembered"
 * search pattern.  
 */
#if HAVE_POSIX_REGCOMP
static regex_t *regpattern = NULL;
#endif
#if HAVE_RE_COMP
int re_pattern = 0;
#endif
#if HAVE_REGCMP
static char *cpattern = NULL;
#endif
#if HAVE_V8_REGCOMP
static struct regexp *regpattern = NULL;
#endif
#if NO_REGEX
static char *last_pattern = NULL;
#endif

static int is_caseless;
static int is_ucase_pattern;

/*
 * Convert text.  Perform one or more of these transformations:
 */
#define	CVT_TO_LC	01	/* Convert upper-case to lower-case */
#define	CVT_BS		02	/* Do backspace processing */

	static void
cvt_text(odst, osrc, ops)
	char *odst;
	char *osrc;
	int ops;
{
	register char *dst;
	register char *src;

	for (src = osrc, dst = odst;  *src != '\0';  src++, dst++)
	{
		if ((ops & CVT_TO_LC) && isupper(*src))
			/* Convert uppercase to lowercase. */
			*dst = tolower(*src);
		else if ((ops & CVT_BS) && *src == '\b' && dst > odst)
			/* Delete BS and preceding char. */
			dst -= 2;
		else 
			/* Just copy. */
			*dst = *src;
	}
	*dst = '\0';
}

/*
 * Are there any uppercase letters in this string?
 */
	static int
is_ucase(s)
	char *s;
{
	register char *p;

	for (p = s;  *p != '\0';  p++)
		if (isupper(*p))
			return (1);
	return (0);
}

/*
 * Is there a previous (remembered) search pattern?
 */
	static int
prev_pattern()
{
#if HAVE_POSIX_REGCOMP
	return (regpattern != NULL);
#endif
#if HAVE_RE_COMP
	return (re_pattern != 0);
#endif
#if HAVE_REGCMP
	return (cpattern != NULL);
#endif
#if HAVE_V8_REGCOMP
	return (regpattern != NULL);
#endif
#if NO_REGEX
	return (last_pattern != NULL);
#endif
}

#if HILITE_SEARCH
/*
 * Repaint the hilites currently displayed on the screen.
 * Repaint each line which contains highlighted text.
 */
	public void
repaint_hilite()
{
	int slinenum;
	POSITION pos;
	POSITION epos;
	extern int can_goto_line;

	if (!can_goto_line)
	{
		repaint();
		return;
	}

	for (slinenum = TOP;  slinenum < TOP + sc_height-1;  slinenum++)
	{
		pos = position(slinenum);
		if (pos == NULL_POSITION)
			continue;
		epos = position(slinenum+1);
		/*
		 * If any character in the line is highlighted, 
		 * repaint the line.
		 */
		if (is_hilited(pos, epos, 1))
		{
			(void) forw_line(pos);
			goto_line(slinenum);
			put_line();
		}
	}
}
#endif

/*
 * Hide search string highlighting.
 */
	public void
undo_search()
{
	if (!prev_pattern())
	{
		error("No previous regular expression", NULL_PARG);
		return;
	}
#if HILITE_SEARCH
	hide_hilite = !hide_hilite;
	repaint_hilite();
#endif
}

/*
 * Compile a pattern in preparation for a pattern match.
 */
	static int
compile_pattern(pattern)
	char *pattern;
{
#if HAVE_POSIX_REGCOMP
	regex_t *s = (regex_t *) ecalloc(1, sizeof(regex_t));
	if (regcomp(s, pattern, 0))
	{
		free(s);
		error("Invalid pattern", NULL_PARG);
		return (-1);
	}
	if (regpattern != NULL)
		regfree(regpattern);
	regpattern = s;
#endif
#if HAVE_RE_COMP
	PARG parg;
	if ((parg.p_string = re_comp(pattern)) != NULL)
	{
		error("%s", &parg);
		return (-1);
	}
	re_pattern = 1;
#endif
#if HAVE_REGCMP
	char *s;
	if ((s = regcmp(pattern, 0)) == NULL)
	{
		error("Invalid pattern", NULL_PARG);
		return (-1);
	}
	if (cpattern != NULL)
		free(cpattern);
	cpattern = s;
#endif
#if HAVE_V8_REGCOMP
	struct regexp *s;
	if ((s = regcomp(pattern)) == NULL)
	{
		/*
		 * regcomp has already printed error message via regerror().
		 */
		return (-1);
	}
	if (regpattern != NULL)
		free(regpattern);
	regpattern = s;
#endif
#if NO_REGEX
	static char lpbuf[100];
	strcpy(lpbuf, pattern);
	last_pattern = lpbuf;
#endif
	return (0);
}

/*
 * Perform a pattern match with the previously compiled pattern.
 * Set sp and ep to the start and end of the matched string.
 */
	static int
match_pattern(line, sp, ep)
	char *line;
	char **sp;
	char **ep;
{
	int matched;
#if HAVE_POSIX_REGCOMP
	regmatch_t rm;
	matched = !regexec(regpattern, line, 1, &rm, 0);
	if (!matched)
		return (0);
	*sp = line + rm.rm_so;
	*ep = line + rm.rm_eo;
#endif
#if HAVE_RE_COMP
	matched = (re_exec(line) == 1);
	/*
	 * re_exec doesn't seem to provide a way to get the matched string.
	 */
	*sp = *ep = NULL;
#endif
#if HAVE_REGCMP
	*ep = regex(cpattern, line);
	matched = (*ep != NULL);
	if (!matched)
		return (0);
	*sp = __loc1;
#endif
#if HAVE_V8_REGCOMP
	matched = regexec(regpattern, line);
	if (!matched)
		return (0);
	*sp = regpattern->startp[0];
	*ep = regpattern->endp[0];
#endif
#if NO_REGEX
	matched = match(last_pattern, line, sp, ep);
#endif
	return (matched);
}

#if HILITE_SEARCH
/*
 * Clear the hilite list.
 */
	public void
clr_hilite()
{
	struct hilite *hl;
	struct hilite *nexthl;

	for (hl = hilite_list;  hl != NULL;  hl = nexthl)
	{
		nexthl = hl->hl_next;
		free((void*)hl);
	}
	hilite_list = hilite_tail = NULL;
}

/*
 * Add a new hilite to the hilite list.
 */
	public void
add_hilite(startpos, endpos)
	POSITION startpos;
	POSITION endpos;
{
	struct hilite *hl;

	hl = (struct hilite *) ecalloc(1, sizeof(struct hilite));
	hl->hl_next = NULL;
	hl->hl_startpos = startpos;
	hl->hl_endpos = endpos;
	if (hilite_tail == NULL)
	{
		hilite_list = hilite_tail = hl;
	} else
	{
		hilite_tail->hl_next = hl;
		hilite_tail = hl;
	}
}

/*
 * Should any characters in a specified range be highlighted?
 * If nohide is nonzero, don't consider hide_hilite.
 */
	public int
is_hilited(pos, epos, nohide)
	POSITION pos;
	POSITION epos;
	int nohide;
{
	struct hilite *hl;

	if (hilite_search == 0)
		/*
		 * Not doing highlighting.
		 */
		return (0);

	if (!nohide && hide_hilite)
		/*
		 * Highlighting is hidden.
		 */
		return (0);

	/*
	 * Look at each highlight and see if any part of it falls in the range.
	 */
	for (hl = hilite_list;  hl != NULL;  hl = hl->hl_next)
	{
		if (hl->hl_endpos > pos &&
		    (epos == NULL_POSITION || epos > hl->hl_startpos))
			return (1);
	}
	return (0);
}

/*
 * Adjust hl_startpos & hl_endpos to account for backspace processing.
 */
	static void
adj_hilite(linepos)
	POSITION linepos;
{
	char *line;
	struct hilite *hl;
	int checkstart;
	POSITION opos;
	POSITION npos;

	/*
	 * Get the raw line again.  Look at each character.
	 */
	(void) forw_raw_line(linepos, &line);
	opos = npos = linepos;
	for (hl = hilite_list;  hl != NULL;  hl = hl->hl_next)
		if (hl->hl_startpos >= linepos)
			break;
	checkstart = 1;
	while (hl != NULL)
	{
		/*
		 * See if we need to adjust the current hl_startpos or 
		 * hl_endpos.  After adjusting startpos[i], move to endpos[i].
		 * After adjusting endpos[i], move to startpos[i+1].
		 * The hl_ lists must be sorted: 
		 * startpos[0] < endpos[0] <= startpos[1] < endpos[1] <= etc.
		 */
		if (checkstart && hl->hl_startpos == opos)
		{
			hl->hl_startpos = npos;
			checkstart = 0;
			continue; /* {{ not really necessary }} */
		} else if (!checkstart && hl->hl_endpos == opos)
		{
			hl->hl_endpos = npos;
			checkstart = 1;
			hl = hl->hl_next;
			continue; /* {{ necessary }} */
		}
		if (*line == '\0')
			break;
		opos++;
		npos++;
		line++;
		if (line[0] == '\b' && line[1] != '\0')
		{
			npos += 2;
			line += 2;
		}
	}
}

/*
 * Make a hilite for each string in a physical line which matches 
 * the current pattern.
 */
	static void
hilite_line(linepos, line, sp, ep)
	POSITION linepos;
	char *line;
	char *sp;
	char *ep;
{
	char *searchp;

	if (sp == NULL || ep == NULL)
		return;
	/*
	 * sp and ep delimit the first match in the line.
	 * Mark the corresponding file positions, then
	 * look for further matches and mark them.
	 * {{ This technique, of calling match_pattern on subsequent
	 *    substrings of the line, may mark more than is correct
	 *    if, for example, the pattern starts with "^". }}
	 */
	searchp = line;
	do {
		if (ep > sp)
		{
			/*
			 * Assume that each char position in the "line"
			 * buffer corresponds to one char position in the file.
			 * This is not quite true; we need to adjust later.
			 */
			add_hilite(linepos + (sp-line), linepos + (ep-line));
		}
		/*
		 * If we matched more than zero characters,
		 * move to the first char after the string we matched.
		 * If we matched zero, just move to the next char.
		 */
		if (ep > searchp)
			searchp = ep;
		else if (*searchp != '\0')
			searchp++;
		else /* end of line */
			break;
	} while (match_pattern(searchp, &sp, &ep));

	if (bs_mode == BS_SPECIAL) 
	{
		/*
		 * If there were backspaces in the original line, they
		 * were removed, and hl_startpos/hl_endpos are not correct.
		 * {{ This is very ugly. }}
		 */
		adj_hilite(linepos);
	}
}
#endif

/*
 * Change the caseless-ness of searches.  
 * Updates the internal search state to reflect a change in the -i flag.
 */
	public void
chg_caseless()
{
	if (!is_ucase_pattern)
		is_caseless = caseless;
#ifdef HILITE_SEARCH
	chg_hilite();
#endif
}

#ifdef HILITE_SEARCH
	public void
chg_hilite()
{
	hide_hilite = 1;
	repaint_hilite();
	clr_hilite();
	hide_hilite = 0;
}
#endif

/*
 * Figure out where to start a search.
 */
	static POSITION
search_pos(search_type)
	int search_type;
{
	POSITION pos;
	int linenum;

	if (empty_screen())
	{
		/*
		 * Start at the beginning (or end) of the file.
		 * (The empty_screen() case is mainly for 
		 * command line initiated searches;
		 * for example, "+/xyz" on the command line.)
		 */
		if (search_type & SRCH_FORW)
		{
			return (ch_zero());
		} else
		{
			pos = ch_length();
			if (pos == NULL_POSITION)
				return (ch_zero());
			return (pos);
		}
	}
	if (how_search)
	{
		/*
		 * Search does not include current screen.
		 */
		if (search_type & SRCH_FORW)
			linenum = BOTTOM_PLUS_ONE;
		else
			linenum = TOP;
		pos = position(linenum);
	} else
	{
		/*
		 * Search includes current screen.
		 * It starts at the jump target (if searching backwards),
		 * or at the jump target plus one (if forwards).
		 */
		linenum = adjsline(jump_sline);
		pos = position(linenum);
		if (search_type & SRCH_FORW)
			pos = forw_raw_line(pos, (char **)NULL);
	}
	return (pos);
}

/*
 * Search a subset of the file, specified by start/end position.
 */
	static int
search_range(pos, endpos, search_type, n, plinepos)
	POSITION pos;
	POSITION endpos;
	int search_type;
	int n;
	POSITION *plinepos;
{
	char *line;
	int linenum;
	char *sp, *ep;
	int line_match;
	POSITION linepos, oldpos;

	linenum = find_linenum(pos);
	oldpos = pos;
	for (;;)
	{
		/*
		 * Get lines until we find a matching one or until
		 * we hit end-of-file (or beginning-of-file if we're 
		 * going backwards), or until we hit the end position.
		 */
		if (ABORT_SIGS())
		{
			/*
			 * A signal aborts the search.
			 */
			return (-1);
		}

		if (endpos != NULL_POSITION && pos >= endpos)
		{
			/*
			 * Reached end position without a match.
			 */
			return (n);
		}

		if (search_type & SRCH_FORW)
		{
			/*
			 * Read the next line, and save the 
			 * starting position of that line in linepos.
			 */
			linepos = pos;
			pos = forw_raw_line(pos, &line);
			if (linenum != 0)
				linenum++;
		} else
		{
			/*
			 * Read the previous line and save the
			 * starting position of that line in linepos.
			 */
			pos = back_raw_line(pos, &line);
			linepos = pos;
			if (linenum != 0)
				linenum--;
		}

		if (pos == NULL_POSITION)
		{
			/*
			 * Reached EOF/BOF without a match.
			 */
			return (n);
		}

		/*
		 * If we're using line numbers, we might as well
		 * remember the information we have now (the position
		 * and line number of the current line).
		 * Don't do it for every line because it slows down
		 * the search.  Remember the line number only if
		 * we're "far" from the last place we remembered it.
		 */
		if (linenums && abs((int)(pos - oldpos)) > 1024)
		{
			add_lnum(linenum, pos);
			oldpos = pos;
		}

		/*
		 * If it's a caseless search, convert the line to lowercase.
		 * If we're doing backspace processing, delete backspaces.
		 */
		if (is_caseless || bs_mode == BS_SPECIAL)
		{
			int ops = 0;
			if (is_caseless) 
				ops |= CVT_TO_LC;
			if (bs_mode == BS_SPECIAL)
				ops |= CVT_BS;
			cvt_text(line, line, ops);
		}

		/*
		 * Test the next line to see if we have a match.
		 * We are successful if we either want a match and got one,
		 * or if we want a non-match and got one.
		 */
		line_match = match_pattern(line, &sp, &ep);
		line_match = (!(search_type & SRCH_NOMATCH) && line_match) ||
				((search_type & SRCH_NOMATCH) && !line_match);
		if (!line_match)
			continue;
		/*
		 * Got a match.
		 */
		if (search_type & SRCH_FIND_ALL)
		{
#if HILITE_SEARCH
			/*
			 * We are supposed to find all matches in the range.
			 * Just add the matches in this line to the 
			 * hilite list and keep searching.
			 */
			if (line_match)
				hilite_line(linepos, line, sp, ep);
#endif
		} else if (--n <= 0)
		{
			/*
			 * Found the one match we're looking for.
			 * Return it.
			 */
#if HILITE_SEARCH
			if (hilite_search == 1)
			{
				/*
				 * Clear the hilite list and add only
				 * the matches in this one line.
				 */
				clr_hilite();
				if (line_match)
					hilite_line(linepos, line, sp, ep);
			}
#endif
			if (plinepos != NULL)
				*plinepos = linepos;
			return (0);
		}
	}
}

/*
 * Search for the n-th occurrence of a specified pattern, 
 * either forward or backward.
 * Return the number of matches not yet found in this file
 * (that is, n minus the number of matches found).
 * Return -1 if the search should be aborted.
 * Caller may continue the search in another file 
 * if less than n matches are found in this file.
 */
	public int
search(search_type, pattern, n)
	int search_type;
	char *pattern;
	int n;
{
	POSITION pos;
	char *line;

	if (pattern == NULL || *pattern == '\0')
	{
		/*
		 * A null pattern means use the previously compiled pattern.
		 */
		if (!prev_pattern())
		{
			error("No previous regular expression", NULL_PARG);
			return (-1);
		}
	} else
	{
		/*
		 * Ignore case if -i is set AND 
		 * the pattern is all lowercase.
		 */
		is_ucase_pattern = is_ucase(pattern);
		if (is_ucase_pattern)
			is_caseless = 0;
		else
			is_caseless = caseless;

		/*
		 * Compile the pattern.
		 */
		if (compile_pattern(pattern) < 0)
			return (-1);
	}

	/*
	 * Figure out where to start the search.
	 */
	pos = search_pos(search_type);
	if (pos == NULL_POSITION)
	{
		/*
		 * Can't find anyplace to start searching from.
		 */
		if (search_type & SRCH_PAST_EOF)
			return (n);
		error("Nothing to search", NULL_PARG);
		return (-1);
	}

#if HILITE_SEARCH
	if (hilite_search)
	{
		/*
		 * Hide current hilites on the screen, because they may change.
		 */
		hide_hilite = 1;
		repaint_hilite();
		hide_hilite = 0;
	}
#endif

	n = search_range(pos, NULL_POSITION, search_type, n, &pos);
	if (n != 0)
	{
		/*
		 * Search was unsuccessful.
		 */
#if HILITE_SEARCH
		repaint_hilite();
#endif
		return (n);
	}

	/*
	 * Go to the matching line.
	 */
	jump_loc(pos, jump_sline);
#if HILITE_SEARCH
	if (hilite_search == 1)
	{
		/*
		 * Display the hilites in the matching line.
		 */
		repaint_hilite();
	}
#endif
	return (0);
}

#if HILITE_SEARCH
/*
 * Highlight all strings currently displayed which match the current pattern.
 */
	public void
screen_hilite()
{
	struct scrpos scrpos;

	if (!prev_pattern())
		return;
	/*
	 * Get the range of file positions displayed on the screen.
	 */
	get_scrpos(&scrpos);
	if (scrpos.pos == NULL_POSITION)
		return;
	/*
	 * Clear the hilite list.
	 * Search the subset of the file which is currently displayed
	 * and add all matches to the hilite list.
	 * Then display the hilites.
	 */
	clr_hilite();
	(void) search_range(scrpos.pos, position(BOTTOM_PLUS_ONE), 
		SRCH_FORW|SRCH_FIND_ALL, 0, NULL);
	repaint_hilite();
}
#endif

#if NO_REGEX
/*
 * We have no pattern matching function from the library.
 * We use this function to do simple pattern matching.
 * It supports no metacharacters like *, etc.
 */
	static int
match(pattern, buf, pfound, pend)
	char *pattern, *buf;
	char **pfound, **pend;
{
	register char *pp, *lp;

	for ( ;  *buf != '\0';  buf++)
	{
		for (pp = pattern, lp = buf;  *pp == *lp;  pp++, lp++)
			if (*pp == '\0' || *lp == '\0')
				break;
		if (*pp == '\0')
		{
			if (pfound != NULL)
				*pfound = buf;
			if (pend != NULL)
				*pend = lp;
			return (1);
		}
	}
	return (0);
}
#endif

#if HAVE_V8_REGCOMP
/*
 * This function is called by the V8 regcomp to report 
 * errors in regular expressions.
 */
	void 
regerror(s) 
	char *s; 
{
	error(s, NULL_PARG);
}
#endif

#if !HAVE_STRCHR
/*
 * strchr is used by regexp.c.
 */
	char *
strchr(s, c)
	char *s;
	int c;
{
	for ( ;  *s != '\0';  s++)
		if (*s == c)
			return (s);
	if (c == '\0')
		return (s);
	return (NULL);
}
#endif

