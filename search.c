/*
 * Copyright (C) 1984-2023  Mark Nudelman
 *
 * You may distribute under the terms of either the GNU General Public
 * License or the Less License, as specified in the README file.
 *
 * For more information, see the README file.
 */


/*
 * Routines to search a file for a pattern.
 */

#include "less.h"
#include "position.h"
#include "charset.h"

#define MINPOS(a,b)     (((a) < (b)) ? (a) : (b))
#define MAXPOS(a,b)     (((a) > (b)) ? (a) : (b))

extern int sigs;
extern int how_search;
extern int caseless;
extern int linenums;
extern int jump_sline;
extern int bs_mode;
extern int proc_backspace;
extern int proc_return;
extern int ctldisp;
extern int status_col;
extern void *ml_search;
extern POSITION start_attnpos;
extern POSITION end_attnpos;
extern int utf_mode;
extern int sc_width;
extern int sc_height;
extern int hshift;
extern int match_shift;
extern int nosearch_header_lines;
extern int nosearch_header_cols;
extern int header_lines;
extern int header_cols;
extern char rscroll_char;
#if HILITE_SEARCH
extern int hilite_search;
extern size_t size_linebuf;
extern lbool squished;
extern int can_goto_line;
static int hide_hilite;
static POSITION prep_startpos;
static POSITION prep_endpos;
public POSITION header_start_pos = NULL_POSITION;
static POSITION header_end_pos;
public lbool search_wrapped = FALSE;

/*
 * Structures for maintaining a set of ranges for hilites and filtered-out
 * lines. Each range is stored as a node within a red-black tree, and we
 * try to extend existing ranges (without creating overlaps) rather than
 * create new nodes if possible. We remember the last node found by a
 * search for constant-time lookup if the next search is near enough to
 * the previous. To aid that, we overlay a secondary doubly-linked list
 * on top of the red-black tree so we can find the preceding/succeeding
 * nodes also in constant time.
 *
 * Each node is allocated from a series of pools, each pool double the size
 * of the previous (for amortised constant time allocation). Since our only
 * tree operations are clear and node insertion, not node removal, we don't
 * need to maintain a usage bitmap or freelist and can just return nodes
 * from the pool in-order until capacity is reached.
 */
struct hilite
{
	POSITION hl_startpos;
	POSITION hl_endpos;
	int hl_attr;
};
struct hilite_node
{
	struct hilite_node *parent;
	struct hilite_node *left;
	struct hilite_node *right;
	struct hilite_node *prev;
	struct hilite_node *next;
	int red;
	struct hilite r;
};
struct hilite_storage
{
	size_t capacity;
	size_t used;
	struct hilite_storage *next;
	struct hilite_node *nodes;
};
struct hilite_tree
{
	struct hilite_storage *first;
	struct hilite_storage *current;
	struct hilite_node *root;
	struct hilite_node *lookaside;
};
#define HILITE_INITIALIZER() { NULL, NULL, NULL, NULL }
#define HILITE_LOOKASIDE_STEPS 2

static struct hilite_tree hilite_anchor = HILITE_INITIALIZER();
static struct hilite_tree filter_anchor = HILITE_INITIALIZER();
static struct pattern_info *filter_infos = NULL;

#endif

/*
 * These are the static variables that represent the "remembered"
 * search pattern and filter pattern.
 */
struct pattern_info {
	PATTERN_TYPE compiled;
	char* text;
	int search_type;
	lbool is_ucase_pattern;
	struct pattern_info *next;
};

#if NO_REGEX
#define info_compiled(info) ((void*)0)
#else
#define info_compiled(info) ((info)->compiled)
#endif
	
static struct pattern_info search_info;
public int is_caseless;

#if OSC8_SEARCH
struct osc8_node
{
	struct osc8_node *prev;
	struct osc8_node *next;
	char *id;
	char *link;
	unsigned int id_len;
	unsigned int link_len;
};

static struct osc8_node *osc8_match_list;
#endif

/*
 * Are there any uppercase letters in this string?
 */
static lbool is_ucase(constant char *str)
{
	constant char *str_end = str + strlen(str);
	LWCHAR ch;

	while (str < str_end)
	{
		ch = step_charc(&str, +1, str_end);
		if (IS_UPPER(ch))
			return (TRUE);
	}
	return (FALSE);
}

/*
 * Discard a saved pattern.
 */
static void clear_pattern(struct pattern_info *info)
{
	if (info->text != NULL)
		free(info->text);
	info->text = NULL;
#if !NO_REGEX
	uncompile_pattern(&info->compiled);
#endif
}

/*
 * Compile and save a search pattern.
 */
static int set_pattern(struct pattern_info *info, constant char *pattern, int search_type, int show_error)
{
	/*
	 * Ignore case if -I is set OR
	 * -i is set AND the pattern is all lowercase.
	 */
	info->is_ucase_pattern = (pattern == NULL) ? FALSE : is_ucase(pattern);
	is_caseless = (info->is_ucase_pattern && caseless != OPT_ONPLUS) ? 0 : caseless;
#if !NO_REGEX
	if (pattern == NULL)
		SET_NULL_PATTERN(info->compiled);
	else if (compile_pattern(pattern, search_type, show_error, &info->compiled) < 0)
		return -1;
#endif
	/* Pattern compiled successfully; save the text too. */
	if (info->text != NULL)
		free(info->text);
	info->text = NULL;
	if (pattern != NULL)
	{
		info->text = (char *) ecalloc(1, strlen(pattern)+1);
		strcpy(info->text, pattern);
	}
	info->search_type = search_type;
	return 0;
}

/*
 * Initialize saved pattern to nothing.
 */
static void init_pattern(struct pattern_info *info)
{
	SET_NULL_PATTERN(info->compiled);
	info->text = NULL;
	info->search_type = 0;
	info->next = NULL;
}

/*
 * Initialize search variables.
 */
public void init_search(void)
{
	init_pattern(&search_info);
}

/*
 * Determine which text conversions to perform before pattern matching.
 */
public int get_cvt_ops(int search_type)
{
	int ops = 0;

	if (is_caseless && (!re_handles_caseless || (search_type & SRCH_NO_REGEX)))
		ops |= CVT_TO_LC;
	if (proc_backspace == OPT_ON || (bs_mode == BS_SPECIAL && proc_backspace == OPT_OFF))
		ops |= CVT_BS;
	if (proc_return == OPT_ON || (bs_mode != BS_CONTROL && proc_backspace == OPT_OFF))
		ops |= CVT_CRLF;
	if (ctldisp == OPT_ONPLUS)
		ops |= CVT_ANSI;
	return (ops);
}

/*
 * Is there a previous (remembered) search pattern?
 */
static int prev_pattern(struct pattern_info *info)
{
#if !NO_REGEX
	if ((info->search_type & SRCH_NO_REGEX) == 0)
		return (!is_null_pattern(info->compiled));
#endif
	return (info->text != NULL);
}

#if HILITE_SEARCH
/*
 * Repaint the hilites currently displayed on the screen.
 * Repaint each line which contains highlighted text.
 * If on==0, force all hilites off.
 */
public void repaint_hilite(int on)
{
	int sindex;
	POSITION pos;
	int save_hide_hilite;

	if (squished)
		repaint();

	save_hide_hilite = hide_hilite;
	if (!on)
	{
		if (hide_hilite)
			return;
		hide_hilite = 1;
	}

	if (!can_goto_line)
	{
		repaint();
		hide_hilite = save_hide_hilite;
		return;
	}

	for (sindex = TOP;  sindex < TOP + sc_height-1;  sindex++)
	{
		pos = position(sindex);
		if (pos == NULL_POSITION)
			continue;
		(void) forw_line(pos);
		goto_line(sindex);
		clear_eol();
		put_line();
	}
	overlay_header();
	lower_left();
	hide_hilite = save_hide_hilite;
}
#endif

/*
 * Clear the attn hilite.
 */
public void clear_attn(void)
{
#if HILITE_SEARCH
	int sindex;
	POSITION old_start_attnpos;
	POSITION old_end_attnpos;
	POSITION pos;
	POSITION epos;
	int moved = 0;

	if (start_attnpos == NULL_POSITION)
		return;
	old_start_attnpos = start_attnpos;
	old_end_attnpos = end_attnpos;
	start_attnpos = end_attnpos = NULL_POSITION;

	if (!can_goto_line)
	{
		repaint();
		return;
	}
	if (squished)
		repaint();

	for (sindex = TOP;  sindex < TOP + sc_height-1;  sindex++)
	{
		pos = position(sindex);
		if (pos == NULL_POSITION)
			continue;
		epos = position(sindex+1);
		if (pos <= old_end_attnpos &&
		     (epos == NULL_POSITION || epos > old_start_attnpos))
		{
			(void) forw_line(pos);
			goto_line(sindex);
			clear_eol();
			put_line();
			moved = 1;
		}
	}
	if (overlay_header())
		moved = 1;
	if (moved)
		lower_left();
#endif
}

/*
 * Toggle or clear search string highlighting.
 */
public void undo_search(int clear)
{
	clear_pattern(&search_info);
#if HILITE_SEARCH
	if (clear)
	{
		clr_hilite();
	} else
	{
		if (hilite_anchor.first == NULL)
		{
			error("No previous regular expression", NULL_PARG);
			return;
		}
		hide_hilite = !hide_hilite;
	}
	repaint_hilite(1);
#endif
}

#if HILITE_SEARCH
/*
 * Clear the hilite list.
 */
public void clr_hlist(struct hilite_tree *anchor)
{
	struct hilite_storage *hls;
	struct hilite_storage *nexthls;

	for (hls = anchor->first;  hls != NULL;  hls = nexthls)
	{
		nexthls = hls->next;
		free((void*)hls->nodes);
		free((void*)hls);
	}
	anchor->first = NULL;
	anchor->current = NULL;
	anchor->root = NULL;

	anchor->lookaside = NULL;

	prep_startpos = prep_endpos = NULL_POSITION;
}

public void clr_hilite(void)
{
	clr_hlist(&hilite_anchor);
}

public void clr_filter(void)
{
	clr_hlist(&filter_anchor);
}

/*
 * Find the node covering pos, or the node after it if no node covers it,
 * or return NULL if pos is after the last range. Remember the found node,
 * to speed up subsequent searches for the same or similar positions (if
 * we return NULL, remember the last node.)
 */
static struct hilite_node* hlist_find(struct hilite_tree *anchor, POSITION pos)
{
	struct hilite_node *n, *m;

	if (anchor->lookaside)
	{
		int steps = 0;
		int hit = 0;

		n = anchor->lookaside;

		for (;;)
		{
			if (pos < n->r.hl_endpos)
			{
				if (n->prev == NULL || pos >= n->prev->r.hl_endpos)
				{
					hit = 1;
					break;
				}
			} else if (n->next == NULL)
			{
				n = NULL;
				hit = 1;
				break;
			}

			/*
			 * If we don't find the right node within a small
			 * distance, don't keep doing a linear search!
			 */
			if (steps >= HILITE_LOOKASIDE_STEPS)
				break;
			steps++;

			if (pos < n->r.hl_endpos)
				anchor->lookaside = n = n->prev;
			else
				anchor->lookaside = n = n->next;
		}

		if (hit)
			return n;
	}

	n = anchor->root;
	m = NULL;

	while (n != NULL)
	{
		if (pos < n->r.hl_startpos)
		{
			if (n->left != NULL)
			{
				m = n;
				n = n->left;
				continue;
			}
			break;
		}
		if (pos >= n->r.hl_endpos)
		{
			if (n->right != NULL)
			{
				n = n->right;
				continue;
			}
			if (m != NULL)
			{
				n = m;
			} else
			{
				m = n;
				n = NULL;
			}
		}
		break;
	}

	if (n != NULL)
		anchor->lookaside = n;
	else if (m != NULL)
		anchor->lookaside = m;

	return n;
}

/*
 * Should any characters in a specified range be highlighted?
 */
static int hilited_range_attr(POSITION pos, POSITION epos)
{
	struct hilite_node *n = hlist_find(&hilite_anchor, pos);
	if (n == NULL)
		return 0;
	if (epos != NULL_POSITION && epos <= n->r.hl_startpos)
		return 0;
	return n->r.hl_attr;
}

/*
 * Set header parameters.
 */
public void set_header(POSITION pos)
{
	header_start_pos = (header_lines == 0) ? NULL_POSITION : pos;
	if (header_start_pos != NULL_POSITION)
	{
		int ln;
		for (ln = 0; ln < header_lines; ++ln)
		{
			pos = forw_raw_line(pos, NULL, NULL);
			if (pos == NULL_POSITION) break;
		}
		header_end_pos = pos;
	}
}

/*
 * Is a position within the header lines?
 */
static int pos_in_header(POSITION pos)
{
	return (header_start_pos != NULL_POSITION &&
	        pos >= header_start_pos && pos < header_end_pos);
}

/* 
 * Is a line "filtered" -- that is, should it be hidden?
 */
public lbool is_filtered(POSITION pos)
{
	struct hilite_node *n;

	if (ch_getflags() & CH_HELPFILE)
		return (FALSE);
	if (pos_in_header(pos))
		return (FALSE);

	n = hlist_find(&filter_anchor, pos);
	return (n != NULL && pos >= n->r.hl_startpos);
}

/*
 * If pos is hidden, return the next position which isn't, otherwise
 * just return pos.
 */
public POSITION next_unfiltered(POSITION pos)
{
	struct hilite_node *n;

	if (ch_getflags() & CH_HELPFILE)
		return (pos);
	if (pos_in_header(pos))
		return (pos);

	n = hlist_find(&filter_anchor, pos);
	while (n != NULL && pos >= n->r.hl_startpos)
	{
		pos = n->r.hl_endpos;
		n = n->next;
	}
	return (pos);
}

/*
 * If pos is hidden, return the previous position which isn't or 0 if
 * we're filtered right to the beginning, otherwise just return pos.
 */
public POSITION prev_unfiltered(POSITION pos)
{
	struct hilite_node *n;

	if (ch_getflags() & CH_HELPFILE)
		return (pos);
	if (pos_in_header(pos))
		return (pos);

	n = hlist_find(&filter_anchor, pos);
	while (n != NULL && pos >= n->r.hl_startpos)
	{
		pos = n->r.hl_startpos;
		if (pos == 0)
			break;
		pos--;
		n = n->prev;
	}
	return (pos);
}

/*
 * Set the hshift for the line starting at line_pos so that the string 
 * between start_off and end_off is visible on the screen.
 */
static void shift_visible(POSITION line_pos, size_t start_off, size_t end_off)
{
	POSITION start_pos = line_pos + start_off;
	POSITION end_pos = line_pos + end_off;
	int start_col = get_col(line_pos, start_pos, NULL_POSITION, -1);
	int end_col = get_col(line_pos, end_pos, start_pos, start_col);
	int swidth = sc_width - line_pfx_width() - (rscroll_char ? 1 : 0);
	int new_hshift;
	if (start_col < 0 || end_col < 0)
		return;
	if (end_col < swidth) /* whole string is in first screen */
		new_hshift = 0;
	else if (start_col > hshift && end_col < hshift + swidth)
		new_hshift = hshift; /* already visible; leave hshift unchanged */
	else 
	{
		int eol_col = get_col(line_pos, NULL_POSITION, end_pos, end_col) - swidth;
		if (start_col >= eol_col) /* whole string is in last screen */
			new_hshift = eol_col;
		else /* shift it to column match_shift */
			new_hshift = (start_col < match_shift) ? 0 : start_col - match_shift;
	}
	if (new_hshift != hshift)
	{
		hshift = new_hshift;
		screen_trashed();
	}
}

/*
 * Should any characters in a specified range be highlighted?
 * If nohide is nonzero, don't consider hide_hilite.
 */
public int is_hilited_attr(POSITION pos, POSITION epos, int nohide, int *p_matches)
{
	int attr;

	if (p_matches != NULL)
		*p_matches = 0;

	if (!status_col &&
	    start_attnpos != NULL_POSITION && 
	    pos <= end_attnpos &&
	     (epos == NULL_POSITION || epos >= start_attnpos))
		/*
		 * The attn line overlaps this range.
		 */
		return (AT_HILITE|AT_COLOR_ATTN);

	attr = hilited_range_attr(pos, epos);
	if (attr == 0)
		return (0);

	if (p_matches == NULL)
		/*
		 * Kinda kludgy way to recognize that caller is checking for
		 * hilite in status column. In this case we want to return
		 * hilite status even if hiliting is disabled or hidden.
		 */
		return (attr);

	/*
	 * Report matches, even if we're hiding highlights.
	 */
	*p_matches = 1;

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

	return (attr);
}

/*
 * Tree node storage: get the current block of nodes if it has spare
 * capacity, or create a new one if not.
 */
static struct hilite_storage * hlist_getstorage(struct hilite_tree *anchor)
{
	size_t capacity = 1;
	struct hilite_storage *s;

	if (anchor->current)
	{
		if (anchor->current->used < anchor->current->capacity)
			return anchor->current;
		capacity = anchor->current->capacity * 2;
	}

	s = (struct hilite_storage *) ecalloc(1, sizeof(struct hilite_storage));
	s->nodes = (struct hilite_node *) ecalloc(capacity, sizeof(struct hilite_node));
	s->capacity = capacity;
	s->used = 0;
	s->next = NULL;
	if (anchor->current)
		anchor->current->next = s;
	else
		anchor->first = s;
	anchor->current = s;
	return s;
}

/*
 * Tree node storage: retrieve a new empty node to be inserted into the
 * tree.
 */
static struct hilite_node * hlist_getnode(struct hilite_tree *anchor)
{
	struct hilite_storage *s = hlist_getstorage(anchor);
	return &s->nodes[s->used++];
}

/*
 * Rotate the tree left around a pivot node.
 */
static void hlist_rotate_left(struct hilite_tree *anchor, struct hilite_node *n)
{
	struct hilite_node *np = n->parent;
	struct hilite_node *nr = n->right;
	struct hilite_node *nrl = n->right->left;

	if (np != NULL)
	{
		if (n == np->left)
			np->left = nr;
		else
			np->right = nr;
	} else
	{
		anchor->root = nr;
	}
	nr->left = n;
	n->right = nrl;

	nr->parent = np;
	n->parent = nr;
	if (nrl != NULL)
		nrl->parent = n;
}

/*
 * Rotate the tree right around a pivot node.
 */
static void hlist_rotate_right(struct hilite_tree *anchor, struct hilite_node *n)
{
	struct hilite_node *np = n->parent;
	struct hilite_node *nl = n->left;
	struct hilite_node *nlr = n->left->right;

	if (np != NULL)
	{
		if (n == np->right)
			np->right = nl;
		else
			np->left = nl;
	} else
	{
		anchor->root = nl;
	}
	nl->right = n;
	n->left = nlr;

	nl->parent = np;
	n->parent = nl;
	if (nlr != NULL)
		nlr->parent = n;
}


/*
 * Add a new hilite to a hilite list.
 */
static void add_hilite(struct hilite_tree *anchor, struct hilite *hl)
{
	struct hilite_node *p, *n, *u;

	/* Ignore empty ranges. */
	if (hl->hl_startpos >= hl->hl_endpos)
		return;

	p = anchor->root;

	/* Inserting the very first node is trivial. */
	if (p == NULL)
	{
		n = hlist_getnode(anchor);
		n->r = *hl;
		anchor->root = n;
		anchor->lookaside = n;
		return;
	}

	/*
	 * Find our insertion point. If we come across any overlapping
	 * or adjoining existing ranges, shrink our range and discard
	 * if it become empty.
	 */
	for (;;)
	{
		if (hl->hl_startpos < p->r.hl_startpos)
		{
			if (hl->hl_endpos > p->r.hl_startpos && hl->hl_attr == p->r.hl_attr)
				hl->hl_endpos = p->r.hl_startpos;
			if (p->left != NULL)
			{
				p = p->left;
				continue;
			}
			break;
		}
		if (hl->hl_startpos < p->r.hl_endpos && hl->hl_attr == p->r.hl_attr) {
			hl->hl_startpos = p->r.hl_endpos;
			if (hl->hl_startpos >= hl->hl_endpos)
				return;
		}
		if (p->right != NULL)
		{
			p = p->right;
			continue;
		}
		break;
	}

	/*
	 * Now we're at the right leaf, again check for contiguous ranges
	 * and extend the existing node if possible to avoid the
	 * insertion. Otherwise insert a new node at the leaf.
	 */
	if (hl->hl_startpos < p->r.hl_startpos) {
		if (hl->hl_attr == p->r.hl_attr)
		{
			if (hl->hl_endpos == p->r.hl_startpos)
			{
				p->r.hl_startpos = hl->hl_startpos;
				return;
			}
			if (p->prev != NULL && p->prev->r.hl_endpos == hl->hl_startpos)
			{
				p->prev->r.hl_endpos = hl->hl_endpos;
				return;
			}
		}
		p->left = n = hlist_getnode(anchor);
		n->next = p;
		if (p->prev != NULL)
		{
			n->prev = p->prev;
			p->prev->next = n;
		}
		p->prev = n;
	} else {
		if (hl->hl_attr == p->r.hl_attr)
		{
			if (p->r.hl_endpos == hl->hl_startpos)
			{
				p->r.hl_endpos = hl->hl_endpos;
				return;
			}
			if (p->next != NULL && hl->hl_endpos == p->next->r.hl_startpos) {
				p->next->r.hl_startpos = hl->hl_startpos;
				return;
			}
		}
		p->right = n = hlist_getnode(anchor);
		n->prev = p;
		if (p->next != NULL)
		{
			n->next = p->next;
			p->next->prev = n;
		}
		p->next = n;
	}
	n->parent = p;
	n->red = 1;
	n->r = *hl;

	/*
	 * The tree is in the correct order and covers the right ranges
	 * now, but may have become unbalanced. Rebalance it using the
	 * standard red-black tree constraints and operations.
	 */
	for (;;)
	{
		/* case 1 - current is root, root is always black */
		if (n->parent == NULL)
		{
			n->red = 0;
			break;
		}

		/* case 2 - parent is black, we can always be red */
		if (!n->parent->red)
			break;

		/*
		 * constraint: because the root must be black, if our
		 * parent is red it cannot be the root therefore we must
		 * have a grandparent
		 */

		/*
		 * case 3 - parent and uncle are red, repaint them black,
		 * the grandparent red, and start again at the grandparent.
		 */
		u = n->parent->parent->left;
		if (n->parent == u) 
			u = n->parent->parent->right;
		if (u != NULL && u->red)
		{
			n->parent->red = 0;
			u->red = 0;
			n = n->parent->parent;
			n->red = 1;
			continue;
		}

		/*
		 * case 4 - parent is red but uncle is black, parent and
		 * grandparent on opposite sides. We need to start
		 * changing the structure now. This and case 5 will shorten
		 * our branch and lengthen the sibling, between them
		 * restoring balance.
		 */
		if (n == n->parent->right &&
		    n->parent == n->parent->parent->left)
		{
			hlist_rotate_left(anchor, n->parent);
			n = n->left;
		} else if (n == n->parent->left &&
			   n->parent == n->parent->parent->right)
		{
			hlist_rotate_right(anchor, n->parent);
			n = n->right;
		}

		/*
		 * case 5 - parent is red but uncle is black, parent and
		 * grandparent on same side
		 */
		n->parent->red = 0;
		n->parent->parent->red = 1;
		if (n == n->parent->left)
			hlist_rotate_right(anchor, n->parent->parent);
		else
			hlist_rotate_left(anchor, n->parent->parent);
		break;
	}
}

/*
 * Highlight every character in a range of displayed characters.
 */
static void create_hilites(POSITION linepos, constant char *line, constant char *sp, constant char *ep, int attr, int *chpos)
{
	size_t start_index = ptr_diff(sp, line); /*{{type-issue}}*/
	size_t end_index = ptr_diff(ep, line);
	struct hilite hl;
	size_t i;

	/* Start the first hilite. */
	hl.hl_startpos = linepos + chpos[start_index];
	hl.hl_attr = attr;

	/*
	 * Step through the displayed chars.
	 * If the source position (before cvt) of the char is one more
	 * than the source pos of the previous char (the usual case),
	 * just increase the size of the current hilite by one.
	 * Otherwise (there are backspaces or something involved),
	 * finish the current hilite and start a new one.
	 */
	for (i = start_index+1;  i <= end_index;  i++)
	{
		if (chpos[i] != chpos[i-1] + 1 || i == end_index)
		{
			hl.hl_endpos = linepos + chpos[i-1] + 1;
			add_hilite(&hilite_anchor, &hl);
			/* Start new hilite unless this is the last char. */
			if (i < end_index)
			{
				hl.hl_startpos = linepos + chpos[i];
			}
		}
	}
}

/*
 * Make a hilite for each string in a physical line which matches 
 * the current pattern.
 * sp,ep delimit the first match already found.
 */
static void hilite_line(POSITION linepos, constant char *line, size_t line_len, int *chpos, constant char **sp, constant char **ep, int nsp)
{
	constant char *searchp;
	constant char *line_end = line + line_len;

	/*
	 * sp[0] and ep[0] delimit the first match in the line.
	 * Mark the corresponding file positions, then
	 * look for further matches and mark them.
	 * {{ This technique, of calling match_pattern on subsequent
	 *    substrings of the line, may mark more than is correct
	 *    if the pattern starts with "^".  This bug is fixed
	 *    for those regex functions that accept a notbol parameter
	 *    (currently POSIX, PCRE and V8-with-regexec2). }}
	 * sp[i] and ep[i] for i>0 delimit subpattern matches.
	 * Color each of them with its unique color.
	 */
	searchp = line;
	do {
		constant char *lep = sp[0];
		int i;
		if (sp[0] == NULL || ep[0] == NULL)
			break;
		for (i = 1;  i < nsp;  i++)
		{
			if (sp[i] == NULL || ep[i] == NULL)
				break;
			if (ep[i] > sp[i])
			{
				create_hilites(linepos, line, lep, sp[i],
					AT_HILITE | AT_COLOR_SEARCH, chpos);
				create_hilites(linepos, line, sp[i], ep[i], 
					AT_HILITE | AT_COLOR_SUBSEARCH(i), chpos);
				lep = ep[i];
			}
		}
		create_hilites(linepos, line, lep, ep[0], 
			AT_HILITE | AT_COLOR_SEARCH, chpos);

		/*
		 * If we matched more than zero characters,
		 * move to the first char after the string we matched.
		 * If we matched zero, just move to the next char.
		 */
		if (ep[0] > searchp)
			searchp = ep[0];
		else if (searchp != line_end)
			searchp++;
		else /* end of line */
			break;
	} while (match_pattern(info_compiled(&search_info), search_info.text,
			searchp, ptr_diff(line_end, searchp), sp, ep, nsp, 1, search_info.search_type));
}
#endif

#if HILITE_SEARCH
/*
 * Find matching text which is currently on screen and highlight it.
 */
static void hilite_screen(void)
{
	struct scrpos scrpos;

	get_scrpos(&scrpos, TOP);
	if (scrpos.pos == NULL_POSITION)
		return;
	prep_hilite(scrpos.pos, position(BOTTOM_PLUS_ONE), -1);
	repaint_hilite(1);
}

/*
 * Change highlighting parameters.
 */
public void chg_hilite(void)
{
	/*
	 * Erase any highlights currently on screen.
	 */
	clr_hilite();
	hide_hilite = 0;

	if (hilite_search == OPT_ONPLUS)
		/*
		 * Display highlights.
		 */
		hilite_screen();
}
#endif

/*
 * Figure out where to start a search.
 */
static POSITION search_pos(int search_type)
{
	POSITION pos;
	int sindex;

	if (empty_screen())
	{
		/*
		 * Start at the beginning (or end) of the file.
		 * The empty_screen() case is mainly for 
		 * command line initiated searches;
		 * for example, "+/xyz" on the command line.
		 * Also for multi-file (SRCH_PAST_EOF) searches.
		 */
		if (search_type & SRCH_FORW)
		{
			pos = ch_zero();
		} else
		{
			pos = ch_length();
			if (pos == NULL_POSITION)
			{
				(void) ch_end_seek();
				pos = ch_length();
			}
		}
		sindex = 0;
	} else 
	{
		int add_one = 0;

		if (how_search == OPT_ON)
		{
			/*
			 * Search does not include current screen.
			 */
			if (search_type & SRCH_FORW)
				sindex = sc_height-1; /* BOTTOM_PLUS_ONE */
			else
				sindex = 0; /* TOP */
		} else if (how_search == OPT_ONPLUS && !(search_type & SRCH_AFTER_TARGET))
		{
			/*
			 * Search includes all of displayed screen.
			 */
			if (search_type & SRCH_FORW)
				sindex = 0; /* TOP */
			else
				sindex = sc_height-1; /* BOTTOM_PLUS_ONE */
		} else 
		{
			/*
			 * Search includes the part of current screen beyond the jump target.
			 * It starts at the jump target (if searching backwards),
			 * or at the jump target plus one (if forwards).
			 */
			sindex = sindex_from_sline(jump_sline);
			if (search_type & SRCH_FORW) 
				add_one = 1;
		}
		pos = position(sindex);
		if (add_one)
			pos = forw_raw_line(pos, NULL, NULL);
	}

	/*
	 * If the line is empty, look around for a plausible starting place.
	 */
	if (search_type & SRCH_FORW) 
	{
		while (pos == NULL_POSITION)
		{
			if (++sindex >= sc_height)
				break;
			pos = position(sindex);
		}
	} else 
	{
		while (pos == NULL_POSITION)
		{
			if (--sindex < 0)
				break;
			pos = position(sindex);
		}
	}
	return (pos);
}

/*
 * Check to see if the line matches the filter pattern.
 * If so, add an entry to the filter list.
 */
#if HILITE_SEARCH
static int matches_filters(POSITION pos, char *cline, size_t line_len, int *chpos, POSITION linepos, constant char **sp, constant char **ep, int nsp)
{
	struct pattern_info *filter;

	for (filter = filter_infos; filter != NULL; filter = filter->next)
	{
		int line_filter = match_pattern(info_compiled(filter), filter->text,
			cline, line_len, sp, ep, nsp, 0, filter->search_type);
		if (line_filter)
		{
			struct hilite hl;
			hl.hl_startpos = linepos;
			hl.hl_endpos = pos;
			add_hilite(&filter_anchor, &hl);
			free(cline);
			free(chpos);
			return (1);
		}
	}
	return (0);
}
#endif

/*
 * Get the position of the first char in the screen line which
 * puts tpos on screen.
 */
static POSITION get_lastlinepos(POSITION pos, POSITION tpos, int sheight)
{
	int nlines;

	for (nlines = 0;;  nlines++)
	{
		POSITION npos = forw_line(pos);
		if (npos > tpos)
		{
			if (nlines < sheight)
				return NULL_POSITION;
			return pos;
		}
		pos = npos;
	}
}

/* OSC 8 search */
#if OSC8_SEARCH
static void osc8_reset(void)
{
	if (osc8_match_list == NULL)
		return;

	while (osc8_match_list->prev != NULL)
		osc8_match_list = osc8_match_list->prev;

	while (osc8_match_list != NULL)
	{
		struct osc8_node *tmp;

		tmp = osc8_match_list;
		osc8_match_list = tmp->next;
		free(tmp);
	}
}

static struct osc8_node * osc8_dup(struct osc8_node *tonp)
{
	/* (less uses int for _line_ lengths) */
	struct osc8_node *nonp;
	void *vp;

	vp = ecalloc(1, sizeof(*nonp) + tonp->id_len +1 + tonp->link_len +1);

	nonp = (struct osc8_node *)vp;
	vp = (char *)vp + sizeof(*nonp);
	nonp->id = (char *)vp;
	nonp->id_len = tonp->id_len;
	vp = (char *)vp + tonp->id_len +1;
	nonp->link = (char *)vp;
	nonp->link_len = tonp->link_len;

	if (nonp->id_len > 0)
	{
		memcpy(nonp->id, tonp->id, nonp->id_len);
		nonp->id[nonp->id_len] = '\0';
	}
	if (nonp->link_len > 0)
	{
		memcpy(nonp->link, tonp->link, nonp->link_len);
		nonp->link[nonp->link_len] = '\0';
	}

	if ((tonp = osc8_match_list) != NULL)
		while (tonp->next != NULL)
			tonp = tonp->next;
	if ((nonp->prev = tonp) != NULL)
		tonp->next = nonp;
	else
		osc8_match_list = nonp;

	return (nonp);
}

static void osc8_search_xy(char const *cbuf, int mode, int only_search)
{
	struct pattern_info si_safe;
	PARG parg;
# if HILITE_SEARCH
        int save_hilite_search, d;
# endif
	char *cp;
	int save_how_search;

	/* We bypass the current pattern */
	memcpy(&si_safe, &search_info, sizeof(search_info));
	memset(&search_info, 0, sizeof(search_info));
# if HILITE_SEARCH
	clr_hilite();
	repaint_hilite(0);
	save_hilite_search = hilite_search;
	hilite_search = 0;
# endif
	save_how_search = how_search;

	if (cbuf != NULL)
	{
		osc8_reset();

		/* xxx winding ways, useless searches */
		how_search = OPT_ONPLUS;
		cp = (char *)cbuf;
		if (search(SRCH_FORW | SRCH_NO_REGEX | SRCH_OSC8, cp, 1) != 0 &&
		   search(SRCH_BACK | SRCH_NO_REGEX | SRCH_OSC8, cp, 1) != 0)
		{
			error("Pattern not found", NULL_PARG);
			cp = NULL;
		} else if(!only_search)
			osc8_open();
	} else
	{
		/* Determine if we can use osc_match_list of current line */
		static struct scrpos osp;
		struct scrpos sp;

		get_scrpos(&sp, TOP);

		how_search = OPT_OFF;
		if (sp.ln != osp.ln || sp.pos != osp.pos || sp.pos == NULL_POSITION)
		{}
		/* Try to use relative matches on current line */
		else if (osc8_match_list != NULL)
		{
			if (mode & SRCH_FORW)
			{
				if (osc8_match_list->next != NULL)
				{
					osc8_match_list = osc8_match_list->next;
					goto jgoto_found;
				}
			}
			else if (osc8_match_list->prev != NULL)
			{
				osc8_match_list = osc8_match_list->prev;
				goto jgoto_found;
			}
		}
		else
			how_search = OPT_ONPLUS;

		osc8_reset();

		/* We must not use NULL or "" as the search pattern xxx */
		mode |= SRCH_WRAP | SRCH_NO_REGEX | SRCH_OSC8;
		if (search(mode, "\005", 1) == 0)
		{
			if (mode & SRCH_BACK)
				while (osc8_match_list->next != NULL)
					osc8_match_list = osc8_match_list->next;

jgoto_found:
			cbuf = parg.p_string = osc8_match_list->link;
		}

		/* Repaint after ^O^[NP] for a clean screen without user input.
		 * And avoid repaint_hilite() repaint()s once again! */
		repaint();
		squished = 0;

		get_scrpos(&osp, TOP);

		if(cbuf != NULL)
		{
			if(only_search)
				error("OSC 8 link: %s", &parg);
			else
				osc8_open();
		} else
			error("No OSC 8 link found", NULL_PARG);
	}

	/* Restore search state, and possibly repaint as necessary */
	memcpy(&search_info, &si_safe, sizeof(search_info));
	how_search = save_how_search;
# if HILITE_SEARCH
	if((hilite_search = save_hilite_search) != 0 && search_info.text != NULL)
		repaint_hilite(1);
# endif
}

public void osc8_id_search(char const *cbuf, int only_search)
{
	osc8_search_xy(cbuf, 0, only_search);
}

public void osc8_goto(int mode)
{
	osc8_search_xy(NULL, mode, TRUE);
}

public void osc8_open(void)
{
	PARG parg;
# if HAVE_SYSTEM
	char *cp;
	unsigned int len;
	char const *cmd;
	int isman;
# endif
	char *buf, c;

	if (osc8_match_list == NULL)
	{
		error("No current OSC 8 link", NULL_PARG);
		return;
	}

	/* Only id= is silent */
	if (osc8_match_list->link[0] == '\0')
		return;

	/* Local reference via ourself */
	if (osc8_match_list->link[0] == '#')
	{
		buf = (char *)ecalloc(1, osc8_match_list->link_len);
		memcpy(buf, &osc8_match_list->link[1],
			osc8_match_list->link_len);
		osc8_id_search(buf, TRUE);
		goto jfree;
	}

	parg.p_string = osc8_match_list->link;

# if !HAVE_SYSTEM
	error("Command not available, cannot open %s", &parg);
# else
	isman = (osc8_match_list->link_len > 4 &&
		!memcmp(osc8_match_list->link, "man:", 4));

	cmd = lgetenv("LESSOSC8OPEN");
	if (cmd == NULL)
	{
		if (!isman)
		{
			error("LESSOSC8OPEN not set, cannot open %s", &parg);
			return;
		}
		cmd = "man";
		len = sizeof("man") -1;
	}
	else if ((len = strlen(cmd)) >= (unsigned int)(INT_MAX >> 2) ||
		osc8_match_list->link_len >= (unsigned int)(INT_MAX >> 4))
	{
		error("LESSOSC8OPEN and/or link value too long", NULL_PARG);
		return;
	}
	else
		isman = 0;

	/* We want to pass exactly one properly quoted argument, except in the
	 * built-in isman case, were we may pass two.
	 * sh(1) quoting is bitter, so simply perform single quote quoting;
	 * to deal with single quotes, we need to close, double-quote it, then
	 * reopen the single-quote: '"'"'.
	 * xxx We do not care to properly quote the command */
	buf = (char *)ecalloc(1, 2 + 2 + len + 5*osc8_match_list->link_len);

	cp = buf;
	cp[0] = '!';
	cp[1] = ' ';
	cp += 2;
	memcpy(cp, cmd, len);
	cp += len;

	/* The link, single quote quoted */
	cp[0] = ' ';
	cp[1] = '\'';
	cp += 2;

	/* For built-in: is there a "man:name(\(\d+\))?" section number? */
	if (isman && (cmd = strchr(osc8_match_list->link, '(')) != NULL)
	{
		while (*++cmd != ')')
		{
			if (*cmd == '\'')
			{
				error("Bogus man(1)ual link: %s", &parg);
				goto jfree;
			}
			*cp++ = *cmd;
		}

		cp[0] = '\'';
		cp[1] = ' ';
		cp[2] = '\'';
		cp += 3;
	}

	for (cmd = osc8_match_list->link; *cmd != '\0'; ++cmd)
	{
		if (isman > 0)
		{
			if (*cmd == ':')
				isman = -1;
		}
		else if (*cmd != '\'')
		{
			if (isman && *cmd == '(')
				break;
			*cp++ = *cmd;
		}
		else
		{
			cp[0] = '\'';
			cp[1] = '"';
			cp[2] = '\'';
			cp[3] = '"';
			cp[4] = '\'';
			cp += 5;
		}
	}

	cp[0] = '\'';
	cp[1] = '\0';

	if (1 /*XXX secure*/)
	{
		clear_bot();
		cmd_putstr(buf);
		if ((c = getcc()) != '\n' && c != '\r')
			goto jfree;
	}

	lsystem(buf, "!done");
# endif /* HAVE_SYSTEM */

jfree:
	free(buf);
}

static struct osc8_node * osc8_inspect(char const *tpattern, char const *ps, char const *pe, char const *us, char const *ue)
{
	struct osc8_node on, *onp;
	unsigned int len, idlen; /* (int used for _line_ lengths: ok) */

	memset(&on, 0, sizeof(on));
	onp = NULL;

	/* Parameters.  In order to apply the "#" URI case below, always search for id= */
	if (/*tpattern != NULL &&*/ ps != pe)
	{
		for (len = (unsigned int)(pe - ps); len > sizeof("id=") -1;)
		{
			unsigned int pl;
			char const *pnp;

			if ((pnp = memchr(ps, ':', len)) != NULL)
			{
				pl = (unsigned int)(pnp++ - ps);
				--len;
			} else
				pl = len;
			len -= pl;

			if ((pe = memchr(ps, '=', pl)) != NULL &&
			   ((unsigned int)(pe - ps) == 2) &&
			   !memcmp(ps, "id", 2))
			{
				on.id = (char *)pe++;
				pl -= 3;
				idlen = pl;
				if (tpattern != NULL && pl == strlen(tpattern) && !memcmp(tpattern, pe, pl))
					on.id_len = idlen;
				break;
			}

			ps = pnp;
		}
	}

	/* URI - ignore sole "#" local anchor URIs _if_ there was an id= parameter */
	if (us != ue && ((len = (unsigned int)(ue - us)) != 1 || *us != '#' || idlen == 0))
	{
		on.link = (char *)us;
		on.link_len = len;
	}

	/* Just put anything sensible in osc8_match_list */
	if (on.id_len > 0 || on.link_len > 0)
	{
		onp = osc8_dup(&on);

		/* (Only) For ID searches an ID match is success! */
		if (tpattern == NULL || on.id_len == 0)
			onp = NULL;
	}

	return (onp);
}

static int matches_osc8(constant char *tpattern, constant char *line, int line_len)
{
	/* We unroll handling as much as possible, no cvt_text(), no match() */
	constant char *line_end;
	struct osc8_node *onp;

	/* xxx Is it _not_ an ID search? <-> osc8_goto() */
	if (tpattern != NULL && tpattern[0] == '\005' && tpattern[1] == '\0')
		tpattern = NULL;

	onp = NULL;

	for (line_end = &line[(unsigned int)line_len]; line < line_end;)
	{
		constant char *ps, *pe, *us, *ue;
		int as;
		struct ansi_state *pansi;
		LWCHAR ch;

		ch = step_char((char**)&line, +1, line_end); /* XXX logical const */
		if ((pansi = ansi_start(ch)) == NULL)
			continue;

		as = ANSI_ERR;
		ps = pe = us = ue = NULL;

		/* Skip to end of ANSI escape sequence. */
		while (line < line_end)
		{
			if ((as = ansi_step(pansi, ch)) != ANSI_MID)
				break;

			if (pansi->hlink)
			{
				/* OSC 8 prologue finished? */
				if (ps == NULL)
					ps = line;
				/* End of parameters, start of URI?
				 * Condition not verified by ansi_step() */
				else if (pe == NULL && ch == ';')
				{
					pe = &line[-1];
					us = line;
				}

			}
			ch = *line++;
		}

		/* If we fully parsed a hlink, inspect.
		 * Note OSC 8 links are explicitly "stream based", end markers
		 * are optional (add_attr_normal() thus "closes it") */
		if (as == ANSI_END && pansi->hlink && pe != NULL)
		{
			/* Mirror exact semantics of ansi_step()! */
			ue = &line[-(1 + (pansi->prev_esc && ch == '\\'))];

			/* Invalid?  End marker?  Or inspect it */
			if (ps <= pe && us <= ue && (ps != pe || us != ue))
			{
				struct osc8_node *xonp;

				xonp = osc8_inspect(tpattern, ps, pe, us, ue);

				/* Only used for id= search: take first */
				if (xonp != NULL && onp == NULL)
					onp = xonp;
			}
		}

		ansi_done(pansi);
	}

	/* For id= searches only an exact match is success */
	if (tpattern != NULL)
	{
		if (onp != NULL)
			osc8_match_list = onp;
		return (onp != NULL);
	}

	/* For link searches any OSC 8 link occurrance is success */
	/* XXX should call create_hilites() upon match with link pattern? */
	return (osc8_match_list != NULL);
}
#endif /* OSC8_SEARCH */

/*
 * Search a subset of the file, specified by start/end position.
 */
static int search_range(POSITION pos, POSITION endpos, int search_type, int matches, int maxlines, POSITION *plinepos, POSITION *pendpos, POSITION *plastlinepos)
{
	constant char *line;
	char *cline;
	size_t line_len;
	LINENUM linenum;
	#define NSP (NUM_SEARCH_COLORS+2)
	constant char *sp[NSP];
	constant char *ep[NSP];
	int line_match;
	int cvt_ops;
	size_t cvt_len;
	int *chpos;
	POSITION linepos, oldpos;
	int skip_bytes = 0;
	size_t swidth = (size_t) (sc_width - line_pfx_width()); /*{{type-issue}}*/
	size_t sheight = (size_t) (sc_height - sindex_from_sline(jump_sline));

	linenum = find_linenum(pos);
	if (nosearch_header_lines && linenum <= header_lines)
	{
		linenum = header_lines + 1;
		pos = find_pos(linenum);
	}
	if (pos == NULL_POSITION)
		return (-1);
	oldpos = pos;
	/* When the search wraps around, end at starting position. */
	if ((search_type & SRCH_WRAP) && endpos == NULL_POSITION)
		endpos = pos;
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

		if ((endpos != NULL_POSITION && !(search_type & SRCH_WRAP) &&
			(((search_type & SRCH_FORW) && pos >= endpos) ||
			 ((search_type & SRCH_BACK) && pos <= endpos))) || maxlines == 0)
		{
			/*
			 * Reached end position without a match.
			 */
			if (pendpos != NULL)
				*pendpos = pos;
			return (matches);
		}
		if (maxlines > 0)
			maxlines--;

		if (search_type & SRCH_FORW)
		{
			/*
			 * Read the next line, and save the 
			 * starting position of that line in linepos.
			 */
			linepos = pos;
			pos = forw_raw_line(pos, &line, &line_len);
			if (linenum != 0)
				linenum++;
		} else
		{
			/*
			 * Read the previous line and save the
			 * starting position of that line in linepos.
			 */
			pos = back_raw_line(pos, &line, &line_len);
			linepos = pos;
			if (linenum != 0)
				linenum--;
		}

		if (pos == NULL_POSITION)
		{
			/*
			 * Reached EOF/BOF without a match.
			 */
			if (search_type & SRCH_WRAP)
			{
				/*
				 * The search wraps around the current file, so
				 * try to continue at BOF/EOF.
				 */
				if (search_type & SRCH_FORW)
				{
					pos = ch_zero();
				} else
				{
					pos = ch_length();
					if (pos == NULL_POSITION)
					{
						(void) ch_end_seek();
						pos = ch_length();
					}
				}
				if (pos != NULL_POSITION) {
					/*
					 * Wrap-around was successful. Clear
					 * the flag so we don't wrap again, and
					 * continue the search at new pos.
					 */
					search_wrapped = TRUE;
					search_type &= ~SRCH_WRAP;
					linenum = find_linenum(pos);
					continue;
				}
			}
			if (pendpos != NULL)
				*pendpos = oldpos;
			return (matches);
		}

		/*
		 * If we're using line numbers, we might as well
		 * remember the information we have now (the position
		 * and line number of the current line).
		 * Don't do it for every line because it slows down
		 * the search.  Remember the line number only if
		 * we're "far" from the last place we remembered it.
		 */
		if (linenums && abs((int)(pos - oldpos)) > 2048)
			add_lnum(linenum, pos);
		oldpos = pos;

		/* OSC 8 search does not need CVT */
#if OSC8_SEARCH
		if (search_type & SRCH_OSC8) {
			if (matches_osc8(search_info.text, line, line_len))
			{
				if (plinepos != NULL)
					*plinepos = linepos;
				return (0);
			}
			osc8_reset();
			continue;
		}
#endif

#if HILITE_SEARCH
		if (is_filtered(linepos))
			continue;
#endif
		if (nosearch_header_cols)
			skip_bytes = skip_columns(header_cols, &line, &line_len);

		/*
		 * If it's a caseless search, convert the line to lowercase.
		 * If we're doing backspace processing, delete backspaces.
		 */
		cvt_ops = get_cvt_ops(search_type);
		cvt_len = cvt_length(line_len, cvt_ops);
		cline = (char *) ecalloc(1, cvt_len);
		chpos = cvt_alloc_chpos(cvt_len);
		cvt_text(cline, line, chpos, &line_len, cvt_ops);

#if HILITE_SEARCH
		/*
		 * If any filters are in effect, ignore non-matching lines.
		 */
		if (filter_infos != NULL &&
		   ((search_type & SRCH_FIND_ALL) ||
		     prep_startpos == NULL_POSITION ||
		     linepos < prep_startpos || linepos >= prep_endpos)) {
			if (matches_filters(pos, cline, line_len, chpos, linepos, sp, ep, NSP))
				continue;
		}
#endif

		/*
		 * Test the next line to see if we have a match.
		 * We are successful if we either want a match and got one,
		 * or if we want a non-match and got one.
		 */
		if (prev_pattern(&search_info))
		{
			line_match = match_pattern(info_compiled(&search_info), search_info.text,
				cline, line_len, sp, ep, NSP, 0, search_type);
			if (line_match)
			{
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
					hilite_line(linepos + skip_bytes, cline, line_len, chpos, sp, ep, NSP);
#endif
				} else if (--matches <= 0)
				{
					/*
					 * Found the one match we're looking for.
					 * Return it.
					 */
#if HILITE_SEARCH
					if (hilite_search == OPT_ON)
					{
						/*
						 * Clear the hilite list and add only
						 * the matches in this one line.
						 */
						clr_hilite();
						hilite_line(linepos + skip_bytes, cline, line_len, chpos, sp, ep, NSP);
					}
#endif
					if (chop_line())
					{
						/*
						 * If necessary, shift horizontally to make sure 
						 * search match is fully visible.
						 */
						if (sp[0] != NULL && ep[0] != NULL)
						{
							size_t start_off = ptr_diff(sp[0], cline);
							size_t end_off = ptr_diff(ep[0], cline);
							shift_visible(linepos, start_off, end_off);
						}
					} else if (plastlinepos != NULL)
					{
						/*
						 * If the line is so long that the highlighted match
						 * won't be seen when the line is displayed normally
						 * (starting at the first char) because it fills the whole 
						 * screen and more, scroll forward until the last char
						 * of the match appears in the last line on the screen.
						 * lastlinepos is the position of the first char of that last line.
						 */
						if (ep[0] != NULL)
						{
							size_t end_off = ptr_diff(ep[0], cline);
							if (end_off >= swidth * sheight / 4) /* heuristic */
								*plastlinepos = get_lastlinepos(linepos, linepos + chpos[end_off], (int) sheight);
						}
					}
					free(cline);
					free(chpos);
					if (plinepos != NULL)
						*plinepos = linepos;
					return (0);
				}
			}
		}
		free(cline);
		free(chpos);
	}
}

/*
 * search for a pattern in history. If found, compile that pattern.
 */
static int hist_pattern(int search_type)
{
#if CMD_HISTORY
	constant char *pattern;

	set_mlist(ml_search, 0);
	pattern = cmd_lastpattern();
	if (pattern == NULL)
		return (0);

	if (set_pattern(&search_info, pattern, search_type, 1) < 0)
		return (-1);

#if HILITE_SEARCH
	if (hilite_search == OPT_ONPLUS && !hide_hilite)
		hilite_screen();
#endif

	return (1);
#else /* CMD_HISTORY */
	return (0);
#endif /* CMD_HISTORY */
}

/*
 * Change the caseless-ness of searches.  
 * Updates the internal search state to reflect a change in the -i flag.
 */
public void chg_caseless(void)
{
	if (!search_info.is_ucase_pattern)
	{
		/*
		 * Pattern did not have uppercase.
		 * Set the search caselessness to the global caselessness.
		 */
		is_caseless = caseless;
		/*
		 * If regex handles caseless, we need to discard 
		 * the pattern which was compiled with the old caseless.
		 */
		if (!re_handles_caseless)
			/* We handle caseless, so the pattern doesn't change. */
			return;
	}
	/*
	 * Regenerate the pattern using the new state.
	 */
	clear_pattern(&search_info);
	(void) hist_pattern(search_info.search_type);
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
public int search(int search_type, constant char *pattern, int n)
{
	POSITION pos;
	POSITION opos;
	POSITION lastlinepos = NULL_POSITION;

	if (pattern == NULL || *pattern == '\0')
	{
		/*
		 * A null pattern means use the previously compiled pattern.
		 */
		search_type |= SRCH_AFTER_TARGET;
		if (!prev_pattern(&search_info))
		{
			int r = hist_pattern(search_type);
			if (r == 0)
				error("No previous regular expression", NULL_PARG);
			if (r <= 0)
				return (-1);
		}
		if ((search_type & SRCH_NO_REGEX) != 
		      (search_info.search_type & SRCH_NO_REGEX))
		{
			error("Please re-enter search pattern", NULL_PARG);
			return -1;
		}
#if HILITE_SEARCH
		if (hilite_search == OPT_ON || status_col)
		{
			/*
			 * Erase the highlights currently on screen.
			 * If the search fails, we'll redisplay them later.
			 */
			repaint_hilite(0);
		}
		if (hilite_search == OPT_ONPLUS && hide_hilite)
		{
			/*
			 * Highlight any matches currently on screen,
			 * before we actually start the search.
			 */
			hide_hilite = 0;
			hilite_screen();
		}
		hide_hilite = 0;
#endif
	} else
	{
		/*
		 * Compile the pattern.
		 */
		int show_error = !(search_type & SRCH_INCR);
		if (set_pattern(&search_info, pattern, search_type, show_error) < 0)
			return (-1);
#if HILITE_SEARCH
		if (hilite_search || status_col)
		{
			/*
			 * Erase the highlights currently on screen.
			 * Also permanently delete them from the hilite list.
			 */
			repaint_hilite(0);
			hide_hilite = 0;
			clr_hilite();
		}
		if (hilite_search == OPT_ONPLUS || status_col)
		{
			/*
			 * Highlight any matches currently on screen,
			 * before we actually start the search.
			 */
			hilite_screen();
		}
#endif
	}

	/*
	 * Figure out where to start the search.
	 */
	pos = search_pos(search_type);
	opos = position(sindex_from_sline(jump_sline));
	if (pos == NULL_POSITION)
	{
		/*
		 * Can't find anyplace to start searching from.
		 */
		if (search_type & SRCH_PAST_EOF)
			return (n);
#if HILITE_SEARCH
		if (hilite_search == OPT_ON || status_col)
			repaint_hilite(1);
#endif
		error("Nothing to search", NULL_PARG);
		return (-1);
	}

	n = search_range(pos, NULL_POSITION, search_type, n, -1,
			&pos, (POSITION*)NULL, &lastlinepos);
	/*
	 * This ABORT_SIGS check ensures that if the user presses interrupt,
	 * we don't continue and complete the search.
	 * That is, we leave the display unchanged.
	 * {{ Is this true? Do we always want to abort the search on interrupt? }}
	 */
	if (ABORT_SIGS())
		return (-1);
	if (n != 0)
	{
		/*
		 * Search was unsuccessful.
		 */
#if HILITE_SEARCH
		if ((hilite_search == OPT_ON || status_col) && n > 0)
			/*
			 * Redisplay old hilites.
			 */
			repaint_hilite(1);
#endif
		return (n);
	}

	if (!(search_type & SRCH_NO_MOVE))
	{
		/*
		 * Go to the matching line.
		 */
		if (lastlinepos != NULL_POSITION)
			jump_loc(lastlinepos, BOTTOM);
		else if (pos != opos)
			jump_loc(pos, jump_sline);
	}

#if HILITE_SEARCH
	if (hilite_search == OPT_ON || status_col)
		/*
		 * Display new hilites in the matching line.
		 */
		repaint_hilite(1);
#endif
	return (0);
}

#if HILITE_SEARCH
/*
 * Prepare hilites in a given range of the file.
 *
 * The pair (prep_startpos,prep_endpos) delimits a contiguous region
 * of the file that has been "prepared"; that is, scanned for matches for
 * the current search pattern, and hilites have been created for such matches.
 * If prep_startpos == NULL_POSITION, the prep region is empty.
 * If prep_endpos == NULL_POSITION, the prep region extends to EOF.
 * prep_hilite asks that the range (spos,epos) be covered by the prep region.
 */
public void prep_hilite(POSITION spos, POSITION epos, int maxlines)
{
	POSITION nprep_startpos = prep_startpos;
	POSITION nprep_endpos = prep_endpos;
	POSITION new_epos;
	POSITION max_epos;
	int result;
	int i;

	/*
	 * Search beyond where we're asked to search, so the prep region covers
	 * more than we need.  Do one big search instead of a bunch of small ones.
	 */
	POSITION SEARCH_MORE = (POSITION) (3*size_linebuf);

	if (!prev_pattern(&search_info) && !is_filtering())
		return;

	/*
	 * Make sure our prep region always starts at the beginning of
	 * a line. (search_range takes care of the end boundary below.)
	 */
	spos = back_raw_line(spos+1, NULL, NULL);

	/*
	 * If we're limited to a max number of lines, figure out the
	 * file position we should stop at.
	 */
	if (maxlines < 0)
		max_epos = NULL_POSITION;
	else
	{
		max_epos = spos;
		for (i = 0;  i < maxlines;  i++)
			max_epos = forw_raw_line(max_epos, NULL, NULL);
	}

	/*
	 * Find two ranges:
	 * The range that we need to search (spos,epos); and the range that
	 * the "prep" region will then cover (nprep_startpos,nprep_endpos).
	 */

	if (prep_startpos == NULL_POSITION ||
	    (epos != NULL_POSITION && epos < prep_startpos) ||
	    spos > prep_endpos)
	{
		/*
		 * New range is not contiguous with old prep region.
		 * Discard the old prep region and start a new one.
		 */
		clr_hilite();
		clr_filter();
		if (epos != NULL_POSITION)
			epos += SEARCH_MORE;
		nprep_startpos = spos;
	} else
	{
		/*
		 * New range partially or completely overlaps old prep region.
		 */
		if (epos == NULL_POSITION)
		{
			/*
			 * New range goes to end of file.
			 */
			;
		} else if (epos > prep_endpos)
		{
			/*
			 * New range ends after old prep region.
			 * Extend prep region to end at end of new range.
			 */
			epos += SEARCH_MORE;
		} else /* (epos <= prep_endpos) */
		{
			/*
			 * New range ends within old prep region.
			 * Truncate search to end at start of old prep region.
			 */
			epos = prep_startpos;
		}

		if (spos < prep_startpos)
		{
			/*
			 * New range starts before old prep region.
			 * Extend old prep region backwards to start at 
			 * start of new range.
			 */
			if (spos < SEARCH_MORE)
				spos = 0;
			else
				spos -= SEARCH_MORE;
			nprep_startpos = spos;
		} else /* (spos >= prep_startpos) */
		{
			/*
			 * New range starts within or after old prep region.
			 * Trim search to start at end of old prep region.
			 */
			spos = prep_endpos;
		}
	}

	if (epos != NULL_POSITION && max_epos != NULL_POSITION &&
	    epos > max_epos)
		/*
		 * Don't go past the max position we're allowed.
		 */
		epos = max_epos;

	if (epos == NULL_POSITION || epos > spos)
	{
		int search_type = SRCH_FORW | SRCH_FIND_ALL;
		search_type |= (search_info.search_type & (SRCH_NO_REGEX|SRCH_SUBSEARCH_ALL));
		for (;;) 
		{
			result = search_range(spos, epos, search_type, 0, maxlines, (POSITION*)NULL, &new_epos, (POSITION*)NULL);
			if (result < 0)
				return;
			if (prep_endpos == NULL_POSITION || new_epos > prep_endpos)
				nprep_endpos = new_epos;

			/*
			 * Check both ends of the resulting prep region to
			 * make sure they're not filtered. If they are,
			 * keep going at least one more line until we find
			 * something that isn't filtered, or hit the end.
			 */
			if (prep_endpos == NULL_POSITION || nprep_endpos > prep_endpos)
			{
				if (new_epos >= nprep_endpos && is_filtered(new_epos-1))
				{
					spos = nprep_endpos;
					epos = forw_raw_line(nprep_endpos, NULL, NULL);
					if (epos == NULL_POSITION)
						break;
					maxlines = 1;
					continue;
				}
			}

			if (prep_startpos == NULL_POSITION || nprep_startpos < prep_startpos)
			{
				if (nprep_startpos > 0 && is_filtered(nprep_startpos))
				{
					epos = nprep_startpos;
					spos = back_raw_line(nprep_startpos, NULL, NULL);
					if (spos == NULL_POSITION)
						break;
					nprep_startpos = spos;
					maxlines = 1;
					continue;
				}
			}
			break;
		}
	}
	prep_startpos = nprep_startpos;
	prep_endpos = nprep_endpos;
}

/*
 * Set the pattern to be used for line filtering.
 */
public void set_filter_pattern(constant char *pattern, int search_type)
{
	struct pattern_info *filter;

	clr_filter();
	if (pattern == NULL || *pattern == '\0')
	{
		/* Clear and free all filters. */
		for (filter = filter_infos; filter != NULL; )
		{
			struct pattern_info *next_filter = filter->next;
			clear_pattern(filter);
			free(filter);
			filter = next_filter;
		}
		filter_infos = NULL;
	} else
	{
		/* Create a new filter and add it to the filter_infos list. */
		filter = ecalloc(1, sizeof(struct pattern_info));
		init_pattern(filter);
		if (set_pattern(filter, pattern, search_type, 1) < 0)
		{
			free(filter);
			return;
		}
		filter->next = filter_infos;
		filter_infos = filter;
	}
	screen_trashed();
}

/*
 * Is there a line filter in effect?
 */
public int is_filtering(void)
{
	if (ch_getflags() & CH_HELPFILE)
		return (0);
	return (filter_infos != NULL);
}
#endif

#if HAVE_V8_REGCOMP
/*
 * This function is called by the V8 regcomp to report 
 * errors in regular expressions.
 */
public int reg_show_error = 1;

void regerror(constant char *s)
{
	PARG parg;

	if (!reg_show_error)
		return;
	parg.p_string = s;
	error("%s", &parg);
}
#endif

