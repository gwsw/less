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
 *	lesskey [-o output] [input]
 *
 *	Make a .less file.
 *	If no input file is specified, standard input is used.
 *	If no output file is specified, $HOME/.less is used.
 *
 *	The .less file is used to specify (to "less") user-defined
 *	key bindings.  Basically any sequence of 1 to MAX_CMDLEN
 *	keystrokes may be bound to an existing less function.
 *
 *	The input file is an ascii file consisting of a 
 *	sequence of lines of the form:
 *		string <whitespace> action [chars] <newline>
 *
 *	"string" is a sequence of command characters which form
 *		the new user-defined command.  The command
 *		characters may be:
 *		1. The actual character itself.
 *		2. A character preceded by ^ to specify a
 *		   control character (e.g. ^X means control-X).
 *		3. A backslash followed by one to three octal digits
 *		   to specify a character by its octal value.
 *		4. A backslash followed by b, e, n, r or t
 *		   to specify \b, ESC, \n, \r or \t, respectively.
 *		5. Any character (other than those mentioned above) preceded 
 *		   by a \ to specify the character itself (characters which
 *		   must be preceded by \ include ^, \, and whitespace.
 *	"action" is the name of a "less" action, from the table below.
 *	"chars" is an optional sequence of characters which is treated
 *		as keyboard input after the command is executed.
 *
 *	Blank lines and lines which start with # are ignored.
 *
 *
 *	The output file is a non-ascii file, consisting of
 *	zero or more byte sequences of the form:
 *		string <0> <action>
 *	or
 *		string <0> <action|A_EXTRA> chars <0>
 *
 *	"string" is the command string.
 *	"<0>" is one null byte.
 *	"<action>" is one byte containing the action code (the A_xxx value).
 *	If action is ORed with A_EXTRA, the action byte is followed
 *		by the null-terminated "chars" string.
 */

#include "less.h"
#include "lesskey.h"
#include "cmd.h"

struct cmdname
{
	char *cn_name;
	int cn_action;
};

struct cmdname cmdnames[] = 
{
	"back-bracket",		A_B_BRACKET,
	"back-line",		A_B_LINE,
	"back-line-force",	A_BF_LINE,
	"back-screen",		A_B_SCREEN,
	"back-scroll",		A_B_SCROLL,
	"back-search",		A_B_SEARCH,
	"back-window",		A_B_WINDOW,
	"debug",		A_DEBUG,
	"display-flag",		A_DISP_OPTION,
	"display-option",	A_DISP_OPTION,
	"end",			A_GOEND,
	"examine",		A_EXAMINE,
	"first-cmd",		A_FIRSTCMD,
	"firstcmd",		A_FIRSTCMD,
	"flush-repaint",	A_FREPAINT,
	"forw-bracket",		A_F_BRACKET,
	"forw-forever",		A_F_FOREVER,
	"forw-line",		A_F_LINE,
	"forw-line-force",	A_FF_LINE,
	"forw-screen",		A_F_SCREEN,
	"forw-scroll",		A_F_SCROLL,
	"forw-search",		A_F_SEARCH,
	"forw-window",		A_F_WINDOW,
	"goto-end",		A_GOEND,
	"goto-line",		A_GOLINE,
	"goto-mark",		A_GOMARK,
	"help",			A_HELP,
	"index-file",		A_INDEX_FILE,
	"invalid",		A_UINVALID,
	"next-file",		A_NEXT_FILE,
	"noaction",		A_NOACTION,
	"percent",		A_PERCENT,
	"pipe",			A_PIPE,
	"prev-file",		A_PREV_FILE,
	"quit",			A_QUIT,
	"repaint",		A_REPAINT,
	"repaint-flush",	A_FREPAINT,
	"repeat-search",	A_AGAIN_SEARCH,
	"repeat-search-all",	A_T_AGAIN_SEARCH,
	"reverse-search",	A_REVERSE_SEARCH,
	"reverse-search-all",	A_T_REVERSE_SEARCH,
	"set-mark",		A_SETMARK,
	"shell",		A_SHELL,
	"status",		A_STAT,
	"toggle-flag",		A_OPT_TOGGLE,
	"toggle-option",	A_OPT_TOGGLE,
	"undo-hilite",		A_UNDO_SEARCH,
	"version",		A_VERSION,
	"visual",		A_VISUAL,
	NULL,			0
};

struct cmdname editnames[] = 
{
	"back-complete",	EC_B_COMPLETE,
	"backspace",		EC_BACKSPACE,
	"delete",		EC_DELETE,
	"down",			EC_DOWN,
	"end",			EC_END,
	"expand",		EC_EXPAND,
	"forw-complete",	EC_F_COMPLETE,
	"home",			EC_HOME,
	"insert",		EC_INSERT,
	"invalid",		EC_UINVALID,
	"kill-line",		EC_LINEKILL,
	"left",			EC_LEFT,
	"literal",		EC_LITERAL,
	"right",		EC_RIGHT,
	"up",			EC_UP,
	"word-backspace",	EC_W_BACKSPACE,
	"word-delete",		EC_W_DELETE,
	"word-left",		EC_W_RIGHT,
	"word-right",		EC_W_LEFT,
	NULL,			0
};

struct table
{
	struct cmdname *names;
	char *pbuffer;
	char buffer[MAX_USERCMD];
};

struct table cmdtable;
struct table edittable;
struct table *currtable = &cmdtable;

char fileheader[] = 
	{ C0_LESSKEY_MAGIC, C1_LESSKEY_MAGIC, 
	  C2_LESSKEY_MAGIC, C3_LESSKEY_MAGIC };
char filetrailer[] = 
	{ C0_END_LESSKEY_MAGIC, C1_END_LESSKEY_MAGIC, C2_END_LESSKEY_MAGIC };
char cmdsection[] = { CMD_SECTION };
char editsection[] = { EDIT_SECTION };
char endsection[] = { END_SECTION };

int linenum;
int errors;
char *infile;
char *outfile;

	void
parse_args(argc, argv)
	int argc;
	char **argv;
{
	outfile = NULL;
	while (--argc > 0 && **(++argv) == '-')
	{
		switch (argv[0][1])
		{
		case 'o':
			outfile = &argv[0][2];
			if (*outfile == '\0')
			{
				if (--argc <= 0)
					usage();
				outfile = *(++argv);
			}
			break;
		default:
			usage();
		}
	}
	if (argc > 1)
		usage();
	/*
	 * Open the input file, or use standard input if none specified.
	 */
	if (argc > 0)
		infile = *argv;
	else
		infile = "-";
}

	char *
find_outfile()
{
	char *p;
	char *filename;

	p = getenv("HOME");
	if (p == NULL || *p == '\0')
	{
		fprintf(stderr, "cannot find $HOME - using current directory\n");
		filename = calloc(strlen(LESSKEYFILE) + 1, sizeof(char));
		strcpy(filename, LESSKEYFILE);
	} else
	{
		filename = calloc(strlen(LESSKEYFILE) + strlen(p) + 2, sizeof(char));
		strcpy(filename, p);
#if MSOFTC
		strcat(filename, "\\");
#else
		strcat(filename, "/");
#endif
		strcat(filename, LESSKEYFILE);
	}
	return (filename);
}

	void
init_tables()
{
	cmdtable.names = cmdnames;
	cmdtable.pbuffer = cmdtable.buffer;

	edittable.names = editnames;
	edittable.pbuffer = edittable.buffer;
}

/*
 * Parse one character of a string.
 */
	int
tchar(pp)
	char **pp;
{
	register char *p;
	register char ch;
	register int i;

	p = *pp;
	switch (*p)
	{
	case '\\':
		++p;
		switch (*p)
		{
		case '0': case '1': case '2': case '3':
		case '4': case '5': case '6': case '7':
			/*
			 * Parse an octal number.
			 */
			ch = 0;
			i = 0;
			do
				ch = 8*ch + (*p - '0');
			while (*++p >= '0' && *p <= '7' && ++i < 3);
			*pp = p;
			return (ch);
		case 'b':
			*pp = p+1;
			return ('\r');
		case 'e':
			*pp = p+1;
			return (ESC);
		case 'n':
			*pp = p+1;
			return ('\n');
		case 'r':
			*pp = p+1;
			return ('\r');
		case 't':
			*pp = p+1;
			return ('\t');
		default:
			/*
			 * Backslash followed by any other char 
			 * just means that char.
			 */
			*pp = p+1;
			return (*p);
		}
	case '^':
		/*
		 * Carat means CONTROL.
		 */
		*pp = p+2;
		return (CONTROL(p[1]));
	}
	*pp = p+1;
	return (*p);
}

/*
 * Skip leading spaces in a string.
 */
	public char *
skipsp(s)
	register char *s;
{
	while (*s == ' ' || *s == '\t')	
		s++;
	return (s);
}

/*
 * Skip non-space characters in a string.
 */
	public char *
skipnsp(s)
	register char *s;
{
	while (*s != '\0' && *s != ' ' && *s != '\t')
		s++;
	return (s);
}

	char *
clean_line(s)
	char *s;
{
	register int i;

	s = skipsp(s);
	for (i = 0;  s[i] != '\n' && s[i] != '\0';  i++)
		if (s[i] == '#' && (i == 0 || s[i-1] != '\\'))
			break;
	s[i] = '\0';
	return (s);
}

	void
add_cmd_char(c)
	int c;
{
	if (currtable->pbuffer >= currtable->buffer + MAX_USERCMD)
	{
		error("too many commands");
		exit(1);
	}
	*(currtable->pbuffer)++ = c;
}

/*
 * See if we have a special "control" line.
 */
	int
control_line(s)
	char *s;
{
#define	PREFIX(str,pat)	(strncmp(str,pat,strlen(pat)-1) == 0)

	if (PREFIX(s, "#line-edit"))
	{
		currtable = &edittable;
		return (1);
	}
	if (PREFIX(s, "#command"))
	{
		currtable = &cmdtable;
		return (1);
	}
	if (PREFIX(s, "#stop"))
	{
		add_cmd_char('\0');
		add_cmd_char(A_END_LIST);
		return (1);
	}
	return (0);
}

/*
 * 
 */
	void
fputbytes(fd, buf, len)
	FILE *fd;
	char *buf;
	int len;
{
	while (len-- > 0)
	{
		fwrite(buf, sizeof(char), 1, fd);
		buf++;
	}
}

/*
 * 
 */
	void
fputint(fd, val)
	FILE *fd;
	unsigned int val;
{
	char c;

	if (val >= KRADIX*KRADIX)
	{
		fprintf(stderr, "error: integer too big (%d > %d)\n", 
			val, KRADIX*KRADIX);
		exit(1);
	}
	c = val % KRADIX;
	fwrite(&c, sizeof(char), 1, fd);
	c = val / KRADIX;
	fwrite(&c, sizeof(char), 1, fd);
}

	int
findaction(actname)
	char *actname;
{
	int i;

	for (i = 0;  currtable->names[i].cn_name != NULL;  i++)
		if (strcmp(currtable->names[i].cn_name, actname) == 0)
			return (currtable->names[i].cn_action);
	error("unknown action");
	return (A_INVALID);
}

usage()
{
	fprintf(stderr, "usage: lesskey [-o output] [input]\n");
	exit(1);
}

	void
error(s)
	char *s;
{
	fprintf(stderr, "line %d: %s\n", linenum, s);
	errors++;
}


/*
 * Parse a line from the lesskey file.
 */
	void
parse_line(line)
	char *line;
{
	char *p;
	int cmdlen;
	char *actname;
	int action;
	int c;

	/*
	 * See if it is a control line.
	 */
	if (control_line(line))
		return;
	/*
	 * Skip leading white space.
	 * Replace the final newline with a null byte.
	 * Ignore blank lines and comments.
	 */
	p = clean_line(line);
	if (*p == '\0')
		return;

	/*
	 * Parse the command string and store it in the current table.
	 */
	cmdlen = 0;
	do
	{
		c = tchar(&p);
		if (++cmdlen > MAX_CMDLEN)
			error("command too long");
		else
			add_cmd_char(c);
	} while (*p != ' ' && *p != '\t' && *p != '\0');
	/*
	 * Terminate the command string with a null byte.
	 */
	add_cmd_char('\0');

	/*
	 * Skip white space between the command string
	 * and the action name.
	 * Terminate the action name with a null byte.
	 */
	p = skipsp(p);
	if (*p == '\0')
	{
		error("missing action");
		return;
	}
	actname = p;
	p = skipnsp(p);
	c = *p;
	*p = '\0';

	/*
	 * Parse the action name and store it in the current table.
	 */
	action = findaction(actname);

	/*
	 * See if an extra string follows the action name.
	 */
	*p = c;
	p = skipsp(p);
	if (*p == '\0')
	{
		add_cmd_char(action);
	} else
	{
		/*
		 * OR the special value A_EXTRA into the action byte.
		 * Put the extra string after the action byte.
		 */
		add_cmd_char(action | A_EXTRA);
		while (*p != '\0')
			add_cmd_char(tchar(&p));
		add_cmd_char('\0');
	}
}

main(argc, argv)
	int argc;
	char *argv[];
{
	FILE *desc;
	FILE *out;
	char line[200];

	/*
	 * Process command line arguments.
	 */
	parse_args(argc, argv);
	init_tables();

	/*
	 * Open the input file.
	 */
	if ((desc = fopen(infile, "r")) == NULL)
	{
		perror(infile);
		exit(1);
	}

	/*
	 * Read and parse the input file, one line at a time.
	 */
	errors = 0;
	linenum = 0;
	while (fgets(line, sizeof(line), desc) != NULL)
	{
		++linenum;
		parse_line(line);
	}

	/*
	 * Write the output file.
	 * If no output file was specified, use "$HOME/.less"
	 */
	if (errors > 0)
	{
		fprintf(stderr, "%d errors; no output produced\n", errors);
		exit(1);
	}

	if (outfile == NULL)
		outfile = find_outfile();
	if ((out = fopen(outfile, "w")) == NULL)
	{
		perror(outfile);
		exit(1);
	}

	/* File header */
	fputbytes(out, fileheader, sizeof(fileheader));

	/* Command key section */
	fputbytes(out, cmdsection, sizeof(cmdsection));
	fputint(out, cmdtable.pbuffer - cmdtable.buffer);
	fputbytes(out, (char *)cmdtable.buffer, cmdtable.pbuffer-cmdtable.buffer);
	/* Edit key section */
	fputbytes(out, editsection, sizeof(editsection));
	fputint(out, edittable.pbuffer - edittable.buffer);
	fputbytes(out, (char *)edittable.buffer, edittable.pbuffer-edittable.buffer);

	/* File trailer */
	fputbytes(out, endsection, sizeof(endsection));
	fputbytes(out, filetrailer, sizeof(filetrailer));
	exit(0);
}
