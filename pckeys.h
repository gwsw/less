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
 * Definitions of keys on the PC.
 * Special (non-ASCII) keys on the PC send a two-byte sequence,
 * where the first byte is 0 and the second is as defined below.
 */
#define	PCK_SHIFT_TAB		'\017'
#define	PCK_ALT_E		'\022'
#define	PCK_CAPS_LOCK		'\072'
#define	PCK_F1			'\073'
#define	PCK_NUM_LOCK		'\105'
#define	PCK_HOME		'\107'
#define	PCK_UP			'\110'
#define	PCK_PAGEUP		'\111'
#define	PCK_LEFT		'\113'
#define	PCK_RIGHT		'\115'
#define	PCK_END			'\117'
#define	PCK_DOWN		'\120'
#define	PCK_PAGEDOWN		'\121'
#define	PCK_INSERT		'\122'
#define	PCK_DELETE		'\123'
#define	PCK_CTL_LEFT		'\163'
#define	PCK_CTL_RIGHT		'\164'
#define	PCK_CTL_DELETE		'\223'
