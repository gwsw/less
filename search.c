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
static POSITION hl_startpos[MAX_HILITE];
static POSITION hl_endpos[MAX_HILITE];
static int num_hilite;
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

	for (slinenum = TOP;  slinenum < TOP + sc_height-1;  slinenum++)
	{
		pos = position(slinenum);
		if (pos == NULL_POSITION)
			continue;
		epos = position(slinenum+1);
		if (is_hilited(pos, epos, 1))
		{
			(void) forw_line(pos);
			goto_line(slinenum);
			put_line();
		}
	}
}

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
 * Clear hilites.
 */
	public void
clr_hilite()
{
	num_hilite = 0;
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
	int i;

	if (!hilite_search)
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
	for (i = 0;  i < num_hilite;  i++)
	{
		if (hl_endpos[i] > pos &&
		    (epos == NULL_POSITION || epos > hl_startpos[i]))
			return (1);
	}
	return (0);
}

/*
 * Adjust hl_startpos & hl_endpos to account for backspace processing.
 */
	static void
adj_hlpos(linepos)
	POSITION linepos;
{
	char *line;
	int i;
	int checkstart;
	POSITION opos;
	POSITION npos;

	/*
	 * Get the line again.  Look at each character.
	 */
	(void) forw_raw_line(linepos, &line);
	opos = npos = linepos;
	i = 0;
	checkstart = 1;
	for (;;)
	{
		/*
		 * See if we need to adjust the current hl_startpos or 
		 * hl_endpos.  After adjusting startpos[i], move to endpos[i].
		 * After adjusting endpos[i], move to startpos[i+1].
		 * The hl_ lists must be sorted: 
		 * startpos[0] < endpos[0] <= startpos[1] < endpos[1] <= etc.
		 */
		if (checkstart && hl_startpos[i] == opos)
		{
			hl_startpos[i] = npos;
			checkstart = 0;
			continue; /* {{ not really necessary }} */
		} else if (!checkstart && hl_endpos[i] == opos)
		{
			hl_endpos[i] = npos;
			checkstart = 1;
			if (++i >= num_hilite)
				break;
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
 * Highlight all the strings in a physical line which match the current pattern.
 */
	static void
hilite_line(linepos, line, sp, ep)
	POSITION linepos;
	char *line;
	char *sp;
	char *ep;
{
	char *searchp;

	/*
	 * sp and ep delimit the first match in the line.
	 * Mark the corresponding file positions, then
	 * look for further matches and mark them.
	 * {{ This technique, of calling match_pattern on subsequent
	 *    substrings of the line, may mark more than is correct
	 *    if, for example, the pattern starts with "^". }}
	 */
	num_hilite = 0;
	searchp = line;
	do {
		if (ep > sp)
		{
			hl_startpos[num_hilite] = linepos + (sp - line);
			hl_endpos[num_hilite] = linepos + (ep - line);
			num_hilite++;
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
	} while (num_hilite < MAX_HILITE && match_pattern(searchp, &sp, &ep));

	if (bs_mode == BS_SPECIAL) 
	{
		/*
		 * If there were backspaces in the original line, they
		 * were removed, and hl_startpos/hl_endpos are not correct.
		 * {{ This is very ugly. }}
		 */
		adj_hlpos(linepos);
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
	clr_hilite();
#endif
}

/*
 * Figure out where to start a search.
 */
	static POSITION
search_pos(goforw)
	int goforw;
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
		if (goforw)
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
		if (goforw)
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
		if (goforw)
			pos = forw_raw_line(pos, (char **)NULL);
	}
	return (pos);
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
	POSITION pos, linepos, oldpos;
	register int goforw;
	register int want_match;
	char *line;
	char *sp, *ep;
	int linenum;
	int line_match;

	/*
	 * Extract flags and type of search.
	 */
	goforw = (SRCH_DIR(search_type) == SRCH_FORW);
	want_match = !(search_type & SRCH_NOMATCH);

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
	pos = search_pos(goforw);
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
		hide_hilite = 1;
		repaint_hilite();
		hide_hilite = 0;
	}
#endif

	linenum = find_linenum(pos);
	oldpos = pos;
	for (;;)
	{
		/*
		 * Get lines until we find a matching one or 
		 * until we hit end-of-file (or beginning-of-file 
		 * if we're going backwards).
		 */
		if (ABORT_SIGS())
			/*
			 * A signal aborts the search.
			 */
			return (-1);

		if (goforw)
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
			 * We hit EOF/BOF without a match.
			 */
#if HILITE_SEARCH
			if (hilite_search)
				repaint_hilite();
#endif
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
		 * We are successful if want_match and line_match are
		 * both true (want a match and got it),
		 * or both false (want a non-match and got it).
		 */
		line_match = match_pattern(line, &sp, &ep);
		if (((want_match && line_match) || (!want_match && !line_match)) &&
		      --n <= 0)
		{
			/*
			 * Found the line.
			 */
#if HILITE_SEARCH
			if (hilite_search)
			{
				/*
				 * Remove any old hilites.
				 * Put hilites on the new matching strings.
				 */
				num_hilite = 0;
				if (line_match && sp != NULL && ep != NULL)
					hilite_line(linepos, line, sp, ep);
			}
#endif
			break;
		}
	}

#if HILITE_SEARCH
	if (hilite_search)
		repaint_hilite();
#endif

	jump_loc(linepos, jump_sline);
	return (0);
}

#if 0 /*UNRELIABLE_HILITE_SEARCH*/
/*
 * Search for all occurrences of the previous pattern
 * within a line.  Mark all matches found.
 * "Marking" a match is done by copying "marker" into the chars whose
 * positions in the "found" string are equal to the positions of
 * the matching string in the "line" string.
 */
	public void
hlsearch(line, found, marker)
	char *line;
	char *found;
	int marker;
{
	char *buf;
	register char *p;

	if (hide_hilite)
		return;
	if (!prev_pattern())
		return;

	/*
	 * If the last search was caseless, convert 
	 * uppercase from the input line to lowercase
	 * in a temporary buffer.
	 */
	buf = line;
	if (is_caseless)
	{
		/*
		 * {{ Yikes, a calloc for every line displayed (with -i)!
		 *    Is this worth optimizing? }}
		 */
		buf = save(line);
		cvt_text(buf, line, CVT_TO_LC);
	}

	{
		/*
		 * Now depending on what type of pattern matching function
		 * we have, define the macro NEXT_MATCH(p).
		 * This will look for the first match in "p";
		 * if it finds one it sets sp and ep to the start and 
		 * end of the matching string and returns 1.
		 * If there is no match, it returns 0.
		 */

#if HAVE_POSIX_REGCOMP
regmatch_t rm;
#define	NEXT_MATCH(p)	(!regexec(regpattern,p,1,&rm,0))
#define	sp		(p+rm.rm_so)
#define	ep		(p+rm.rm_eo)
#endif

#if HAVE_RE_COMP
@@@ Sorry, re_comp does not support HILITE_SEARCH.
@@@ Either undefine HILITE_SEARCH, or use the regexp.c that is 
@@@ distributed with "less" (and set HAVE_V8_REGCOMP).
#endif

#if HAVE_REGCMP
#define	sp		__loc1
char *ep;
#define	NEXT_MATCH(p)	(ep = regex(cpattern, p))
#endif

#if HAVE_V8_REGCOMP
#define	sp		(regpattern->startp[0])
#define	ep		(regpattern->endp[0])
#define	NEXT_MATCH(p)	(regexec(regpattern, p))
#endif

#if NO_REGEX
char *sp, *ep;
#define	NEXT_MATCH(p)	(match(last_pattern,p,&sp,&ep))
#endif

		for (p = buf;  NEXT_MATCH(p);  )
		{
			register char *fsp = &found[sp-buf];
			register char *fep = &found[ep-buf];

			while (fsp < fep)
				*fsp++ = marker;
			
			/*
			 * Move past current match to see if there
			 * are any more matches.  (But don't loop 
			 * forever if we matched zero characters.)
			 */
			if (ep > p)
				p = ep;
			else if (*p != '\0')
				p++;
			else
				break;
		}
	}

	if (buf != line)
		free(buf);
}
#endif /* HILITE_SEARCH */

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

