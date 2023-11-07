/*
 * Copyright (C) 1984-2023  Mark Nudelman
 *
 * You may distribute under the terms of either the GNU General Public
 * License or the Less License, as specified in the README file.
 *
 * For more information, see the README file.
 */


#include "less.h"

#define WHITESP(c)      ((c)==' ' || (c)=='\t')

#if TAGS

public char ztags[] = "tags";
public char *tags = ztags;

static int total;
static int curseq;

extern int linenums;
extern int sigs;
extern int ctldisp;

enum tag_result {
	TAG_FOUND,
	TAG_NOFILE,
	TAG_NOTAG,
	TAG_NOTYPE,
	TAG_INTR
};

/*
 * Tag type
 */
enum {
	T_CTAGS,        /* 'tags': standard and extended format (ctags) */
	T_CTAGS_X,      /* stdin: cross reference format (ctags) */
	T_GTAGS,        /* 'GTAGS': function definition (global) */
	T_GRTAGS,       /* 'GRTAGS': function reference (global) */
	T_GSYMS,        /* 'GSYMS': other symbols (global) */
	T_GPATH         /* 'GPATH': path name (global) */
};

static enum tag_result findctag(char *tag);
static enum tag_result findgtag(char *tag, int type);
static char *nextgtag(void);
static char *prevgtag(void);
static POSITION ctagsearch(void);
static POSITION gtagsearch(void);
static int getentry(char *buf, char **tag, char **file, char **line);

/*
 * The list of tags generated by the last findgtag() call.
 *
 * Use either pattern or line number.
 * findgtag() always uses line number, so pattern is always NULL.
 * findctag() uses either pattern (in which case line number is 0),
 * or line number (in which case pattern is NULL).
 */
struct taglist {
	struct tag *tl_first;
	struct tag *tl_last;
};
struct tag {
	struct tag *next, *prev; /* List links */
	char *tag_file;         /* Source file containing the tag */
	LINENUM tag_linenum;    /* Appropriate line number in source file */
	char *tag_pattern;      /* Pattern used to find the tag */
	char tag_endline;       /* True if the pattern includes '$' */
};
#define TAG_END  ((struct tag *) &taglist)
static struct taglist taglist = { TAG_END, TAG_END };
static struct tag *curtag;

#define TAG_INS(tp) \
	(tp)->next = TAG_END; \
	(tp)->prev = taglist.tl_last; \
	taglist.tl_last->next = (tp); \
	taglist.tl_last = (tp);

#define TAG_RM(tp) \
	(tp)->next->prev = (tp)->prev; \
	(tp)->prev->next = (tp)->next;

/*
 * Delete tag structures.
 */
public void cleantags(void)
{
	struct tag *tp;

	/*
	 * Delete any existing tag list.
	 * {{ Ideally, we wouldn't do this until after we know that we
	 *    can load some other tag information. }}
	 */
	while ((tp = taglist.tl_first) != TAG_END)
	{
		TAG_RM(tp);
		free(tp->tag_file);
		free(tp->tag_pattern);
		free(tp);
	}
	curtag = NULL;
	total = curseq = 0;
}

/*
 * Create a new tag entry.
 */
static struct tag * maketagent(char *name, char *file, LINENUM linenum, char *pattern, int endline)
{
	struct tag *tp;

	tp = (struct tag *) ecalloc(sizeof(struct tag), 1);
	tp->tag_file = (char *) ecalloc(strlen(file) + 1, sizeof(char));
	strcpy(tp->tag_file, file);
	tp->tag_linenum = linenum;
	tp->tag_endline = endline;
	if (pattern == NULL)
		tp->tag_pattern = NULL;
	else
	{
		tp->tag_pattern = (char *) ecalloc(strlen(pattern) + 1, sizeof(char));
		strcpy(tp->tag_pattern, pattern);
	}
	return (tp);
}

/*
 * Get tag mode.
 */
public int gettagtype(void)
{
	int f;

	if (strcmp(tags, "GTAGS") == 0)
		return T_GTAGS;
	if (strcmp(tags, "GRTAGS") == 0)
		return T_GRTAGS;
	if (strcmp(tags, "GSYMS") == 0)
		return T_GSYMS;
	if (strcmp(tags, "GPATH") == 0)
		return T_GPATH;
	if (strcmp(tags, "-") == 0)
		return T_CTAGS_X;
	f = open(tags, OPEN_READ);
	if (f >= 0)
	{
		close(f);
		return T_CTAGS;
	}
	return T_GTAGS;
}

/*
 * Find tags in tag file.
 * Find a tag in the "tags" file.
 * Sets "tag_file" to the name of the file containing the tag,
 * and "tagpattern" to the search pattern which should be used
 * to find the tag.
 */
public void findtag(char *tag)
{
	int type = gettagtype();
	enum tag_result result;

	if (type == T_CTAGS)
		result = findctag(tag);
	else
		result = findgtag(tag, type);
	switch (result)
	{
	case TAG_FOUND:
	case TAG_INTR:
		break;
	case TAG_NOFILE:
		error("No tags file", NULL_PARG);
		break;
	case TAG_NOTAG:
		error("No such tag in tags file", NULL_PARG);
		break;
	case TAG_NOTYPE:
		error("unknown tag type", NULL_PARG);
		break;
	}
}

/*
 * Search for a tag.
 */
public POSITION tagsearch(void)
{
	if (curtag == NULL)
		return (NULL_POSITION);  /* No gtags loaded! */
	if (curtag->tag_linenum != 0)
		return gtagsearch();
	else
		return ctagsearch();
}

/*
 * Go to the next tag.
 */
public char * nexttag(int n)
{
	char *tagfile = (char *) NULL;

	while (n-- > 0)
		tagfile = nextgtag();
	return tagfile;
}

/*
 * Go to the previous tag.
 */
public char * prevtag(int n)
{
	char *tagfile = (char *) NULL;

	while (n-- > 0)
		tagfile = prevgtag();
	return tagfile;
}

/*
 * Return the total number of tags.
 */
public int ntags(void)
{
	return total;
}

/*
 * Return the sequence number of current tag.
 */
public int curr_tag(void)
{
	return curseq;
}

/*****************************************************************************
 * ctags
 */

/*
 * Find tags in the "tags" file.
 * Sets curtag to the first tag entry.
 */
static enum tag_result findctag(char *tag)
{
	char *p;
	char *q;
	FILE *f;
	int taglen;
	LINENUM taglinenum;
	char *tagfile;
	char *tagpattern;
	int tagendline;
	int search_char;
	int err;
	char tline[TAGLINE_SIZE];
	struct tag *tp;

	p = shell_unquote(tags);
	f = fopen(p, "r");
	free(p);
	if (f == NULL)
		return TAG_NOFILE;

	cleantags();
	total = 0;
	taglen = (int) strlen(tag);

	/*
	 * Search the tags file for the desired tag.
	 */
	while (fgets(tline, sizeof(tline), f) != NULL)
	{
		if (tline[0] == '!')
			/* Skip header of extended format. */
			continue;
		if (strncmp(tag, tline, taglen) != 0 || !WHITESP(tline[taglen]))
			continue;

		/*
		 * Found it.
		 * The line contains the tag, the filename and the
		 * location in the file, separated by white space.
		 * The location is either a decimal line number, 
		 * or a search pattern surrounded by a pair of delimiters.
		 * Parse the line and extract these parts.
		 */
		tagpattern = NULL;

		/*
		 * Skip over the whitespace after the tag name.
		 */
		p = skipsp(tline+taglen);
		if (*p == '\0')
			/* File name is missing! */
			continue;

		/*
		 * Save the file name.
		 * Skip over the whitespace after the file name.
		 */
		tagfile = p;
		while (!WHITESP(*p) && *p != '\0')
			p++;
		*p++ = '\0';
		p = skipsp(p);
		if (*p == '\0')
			/* Pattern is missing! */
			continue;

		/*
		 * First see if it is a line number. 
		 */
		tagendline = 0;
		taglinenum = getnum(&p, 0, &err);
		if (err)
		{
			/*
			 * No, it must be a pattern.
			 * Delete the initial "^" (if present) and 
			 * the final "$" from the pattern.
			 * Delete any backslash in the pattern.
			 */
			taglinenum = 0;
			search_char = *p++;
			if (*p == '^')
				p++;
			tagpattern = q = p;
			while (*p != search_char && *p != '\0')
			{
				if (*p == '\\')
					p++;
				if (q != p)
				{
					*q++ = *p++;
				} else
				{
					q++;
					p++;
				}
			}
			tagendline = (q[-1] == '$');
			if (tagendline)
				q--;
			*q = '\0';
		}
		tp = maketagent(tag, tagfile, taglinenum, tagpattern, tagendline);
		TAG_INS(tp);
		total++;
	}
	fclose(f);
	if (total == 0)
		return TAG_NOTAG;
	curtag = taglist.tl_first;
	curseq = 1;
	return TAG_FOUND;
}

/*
 * Edit current tagged file.
 */
public int edit_tagfile(void)
{
	if (curtag == NULL)
		return (1);
	return (edit(curtag->tag_file));
}

static int curtag_match(char constant *line, POSITION linepos)
{
	/*
	 * Test the line to see if we have a match.
	 * Use strncmp because the pattern may be
	 * truncated (in the tags file) if it is too long.
	 * If tagendline is set, make sure we match all
	 * the way to end of line (no extra chars after the match).
	 */
	int len = (int) strlen(curtag->tag_pattern);
	if (strncmp(curtag->tag_pattern, line, len) == 0 &&
	    (!curtag->tag_endline || line[len] == '\0' || line[len] == '\r'))
	{
		curtag->tag_linenum = find_linenum(linepos);
		return 1;
	}
	return 0;
}

/*
 * Search for a tag.
 * This is a stripped-down version of search().
 * We don't use search() for several reasons:
 *   -  We don't want to blow away any search string we may have saved.
 *   -  The various regular-expression functions (from different systems:
 *      regcmp vs. re_comp) behave differently in the presence of 
 *      parentheses (which are almost always found in a tag).
 */
static POSITION ctagsearch(void)
{
	POSITION pos, linepos;
	LINENUM linenum;
	int line_len;
	char *line;
	int found;

	pos = ch_zero();
	linenum = find_linenum(pos);

	for (found = 0; !found;)
	{
		/*
		 * Get lines until we find a matching one or 
		 * until we hit end-of-file.
		 */
		if (ABORT_SIGS())
			return (NULL_POSITION);

		/*
		 * Read the next line, and save the 
		 * starting position of that line in linepos.
		 */
		linepos = pos;
		pos = forw_raw_line(pos, &line, &line_len);
		if (linenum != 0)
			linenum++;

		if (pos == NULL_POSITION)
		{
			/*
			 * We hit EOF without a match.
			 */
			error("Tag not found", NULL_PARG);
			return (NULL_POSITION);
		}

		/*
		 * If we're using line numbers, we might as well
		 * remember the information we have now (the position
		 * and line number of the current line).
		 */
		if (linenums)
			add_lnum(linenum, pos);

		if (ctldisp != OPT_ONPLUS)
		{
			if (curtag_match(line, linepos))
				found = 1;
		} else
		{
			int cvt_ops = CVT_ANSI;
			int cvt_len = cvt_length(line_len, cvt_ops);
			int *chpos = cvt_alloc_chpos(cvt_len);
			char *cline = (char *) ecalloc(1, cvt_len);
			cvt_text(cline, line, chpos, &line_len, cvt_ops);
			if (curtag_match(cline, linepos))
				found = 1;
			free(chpos);
			free(cline);
		}
	}

	return (linepos);
}

/*******************************************************************************
 * gtags
 */

/*
 * Find tags in the GLOBAL's tag file.
 * The findgtag() will try and load information about the requested tag.
 * It does this by calling "global -x tag" and storing the parsed output
 * for future use by gtagsearch().
 * Sets curtag to the first tag entry.
 */
static enum tag_result findgtag(char *tag, int type)
{
	char buf[1024];
	FILE *fp;
	struct tag *tp;

	if (type != T_CTAGS_X && tag == NULL)
		return TAG_NOFILE;

	cleantags();
	total = 0;

	/*
	 * If type == T_CTAGS_X then read ctags's -x format from stdin
	 * else execute global(1) and read from it.
	 */
	if (type == T_CTAGS_X)
	{
		fp = stdin;
		/* Set tag default because we cannot read stdin again. */
		tags = ztags;
	} else
	{
#if !HAVE_POPEN
		return TAG_NOFILE;
#else
		char *command;
		char *flag;
		char *qtag;
		constant char *cmd = lgetenv("LESSGLOBALTAGS");

		if (isnullenv(cmd))
			return TAG_NOFILE;
		/* Get suitable flag value for global(1). */
		switch (type)
		{
		case T_GTAGS:
			flag = "" ;
			break;
		case T_GRTAGS:
			flag = "r";
			break;
		case T_GSYMS:
			flag = "s";
			break;
		case T_GPATH:
			flag = "P";
			break;
		default:
			return TAG_NOTYPE;
		}

		/* Get our data from global(1). */
		qtag = shell_quote(tag);
		if (qtag == NULL)
			qtag = tag;
		command = (char *) ecalloc(strlen(cmd) + strlen(flag) +
				strlen(qtag) + 5, sizeof(char));
		sprintf(command, "%s -x%s %s", cmd, flag, qtag);
		if (qtag != tag)
			free(qtag);
		fp = popen(command, "r");
		free(command);
#endif
	}
	if (fp != NULL)
	{
		while (fgets(buf, sizeof(buf), fp))
		{
			char *name, *file, *line;
			int len;

			if (sigs)
			{
#if HAVE_POPEN
				if (fp != stdin)
					pclose(fp);
#endif
				return TAG_INTR;
			}
			len = (int) strlen(buf);
			if (len > 0 && buf[len-1] == '\n')
				buf[len-1] = '\0';
			else
			{
				int c;
				do {
					c = fgetc(fp);
				} while (c != '\n' && c != EOF);
			}

			if (getentry(buf, &name, &file, &line))
			{
				/*
				 * Couldn't parse this line for some reason.
				 * We'll just pretend it never happened.
				 */
				break;
			}

			/* Make new entry and add to list. */
			tp = maketagent(name, file, (LINENUM) atoi(line), NULL, 0);
			TAG_INS(tp);
			total++;
		}
		if (fp != stdin)
		{
			if (pclose(fp))
			{
				curtag = NULL;
				total = curseq = 0;
				return TAG_NOFILE;
			}
		}
	}

	/* Check to see if we found anything. */
	tp = taglist.tl_first;
	if (tp == TAG_END)
		return TAG_NOTAG;
	curtag = tp;
	curseq = 1;
	return TAG_FOUND;
}

static int circular = 0;        /* 1: circular tag structure */

/*
 * Return the filename required for the next gtag in the queue that was setup
 * by findgtag().  The next call to gtagsearch() will try to position at the
 * appropriate tag.
 */
static char * nextgtag(void)
{
	struct tag *tp;

	if (curtag == NULL)
		/* No tag loaded */
		return NULL;

	tp = curtag->next;
	if (tp == TAG_END)
	{
		if (!circular)
			return NULL;
		/* Wrapped around to the head of the queue */
		curtag = taglist.tl_first;
		curseq = 1;
	} else
	{
		curtag = tp;
		curseq++;
	}
	return (curtag->tag_file);
}

/*
 * Return the filename required for the previous gtag in the queue that was
 * setup by findgtat().  The next call to gtagsearch() will try to position
 * at the appropriate tag.
 */
static char * prevgtag(void)
{
	struct tag *tp;

	if (curtag == NULL)
		/* No tag loaded */
		return NULL;

	tp = curtag->prev;
	if (tp == TAG_END)
	{
		if (!circular)
			return NULL;
		/* Wrapped around to the tail of the queue */
		curtag = taglist.tl_last;
		curseq = total;
	} else
	{
		curtag = tp;
		curseq--;
	}
	return (curtag->tag_file);
}

/*
 * Position the current file at at what is hopefully the tag that was chosen
 * using either findtag() or one of nextgtag() and prevgtag().  Returns -1
 * if it was unable to position at the tag, 0 if successful.
 */
static POSITION gtagsearch(void)
{
	if (curtag == NULL)
		return (NULL_POSITION);  /* No gtags loaded! */
	return (find_pos(curtag->tag_linenum));
}

/*
 * The getentry() parses both standard and extended ctags -x format.
 *
 * [standard format]
 * <tag>   <lineno>  <file>         <image>
 * +------------------------------------------------
 * |main     30      main.c         main(argc, argv)
 * |func     21      subr.c         func(arg)
 *
 * The following commands write this format.
 *      o Traditinal Ctags with -x option
 *      o Global with -x option
 *              See <http://www.gnu.org/software/global/global.html>
 *
 * [extended format]
 * <tag>   <type>  <lineno>   <file>        <image>
 * +----------------------------------------------------------
 * |main     function 30      main.c         main(argc, argv)
 * |func     function 21      subr.c         func(arg)
 *
 * The following commands write this format.
 *      o Exuberant Ctags with -x option
 *              See <http://ctags.sourceforge.net>
 *
 * Returns 0 on success, -1 on error.
 * The tag, file, and line will each be NUL-terminated pointers
 * into buf.
 */
static int getentry(char *buf, char **tag, char **file, char **line)
{
	char *p = buf;

	for (*tag = p;  *p && !IS_SPACE(*p);  p++)      /* tag name */
		;
	if (*p == 0)
		return (-1);
	*p++ = 0;
	for ( ;  *p && IS_SPACE(*p);  p++)              /* (skip blanks) */
		;
	if (*p == 0)
		return (-1);
	/*
	 * If the second part begin with other than digit,
	 * it is assumed tag type. Skip it.
	 */
	if (!IS_DIGIT(*p))
	{
		for ( ;  *p && !IS_SPACE(*p);  p++)     /* (skip tag type) */
			;
		for (;  *p && IS_SPACE(*p);  p++)       /* (skip blanks) */
			;
	}
	if (!IS_DIGIT(*p))
		return (-1);
	*line = p;                                      /* line number */
	for (*line = p;  *p && !IS_SPACE(*p);  p++)
		;
	if (*p == 0)
		return (-1);
	*p++ = 0;
	for ( ; *p && IS_SPACE(*p);  p++)               /* (skip blanks) */
		;
	if (*p == 0)
		return (-1);
	*file = p;                                      /* file name */
	for (*file = p;  *p && !IS_SPACE(*p);  p++)
		;
	if (*p == 0)
		return (-1);
	*p = 0;

	/* value check */
	if (strlen(*tag) && strlen(*line) && strlen(*file) && atoi(*line) > 0)
		return (0);
	return (-1);
}

#endif
