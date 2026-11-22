/*
 * Copyright (C) 1984-2026  Mark Nudelman
 *
 * You may distribute under the terms of either the GNU General Public
 * License or the Less License, as specified in the README file.
 *
 * For more information, see the README file.
 */

#include "lang.h"
#include "xbuf.h"

struct lesskey_table
{
	constant struct lesskey_cmdname *names;
	struct xbuffer buf;
	int is_var;
};

struct lesskey_tables
{
	struct lesskey_table *currtable;
	struct lesskey_table cmdtable;
	struct lesskey_table edittable;
	struct lesskey_table vartable;
};

