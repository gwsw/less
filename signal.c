/*@@copyright@@*/


/*
 * Routines dealing with signals.
 *
 * A signal usually merely causes a bit to be set in the "signals" word.
 * At some convenient time, the mainline code checks to see if any
 * signals need processing by calling psignal().
 * If we happen to be reading from a file [in iread()] at the time
 * the signal is received, we call intread to interrupt the iread.
 */

#include "less.h"
#include <signal.h>

/*
 * "sigs" contains bits indicating signals which need to be processed.
 */
public int sigs;

extern int sc_width, sc_height;
extern int screen_trashed;
extern int lnloop;
extern int linenums;
extern int wscroll;
extern int reading;
extern int quit_on_intr;
extern int secure;
extern long jump_sline_fraction;

/*
 * Interrupt signal handler.
 */
#if MSDOS_COMPILER!=WIN32C
	/* ARGSUSED*/
	static RETSIGTYPE
u_interrupt(type)
	int type;
{
	bell();
#if OS2
	LSIGNAL(SIGINT, SIG_ACK);
#endif
	LSIGNAL(SIGINT, u_interrupt);
	sigs |= S_INTERRUPT;
#if MSDOS_COMPILER==DJGPPC
	/*
	 * If a keyboard has been hit, it must be Ctrl-C
	 * (as opposed to Ctrl-Break), so consume it.
	 * (Otherwise, Less will beep when it sees Ctrl-C from keyboard.)
	 */
	if (kbhit())
		getkey();
#endif
#if HILITE_SEARCH
	set_filter_pattern(NULL, 0);
#endif
	if (reading)
		intread(); /* May longjmp */
}
#endif

#ifdef SIGTSTP
/*
 * "Stop" (^Z) signal handler.
 */
	/* ARGSUSED*/
	static RETSIGTYPE
stop(type)
	int type;
{
	LSIGNAL(SIGTSTP, stop);
	sigs |= S_STOP;
	if (reading)
		intread();
}
#endif

#undef SIG_LESSWINDOW
#ifdef SIGWINCH
#define SIG_LESSWINDOW SIGWINCH
#else
#ifdef SIGWIND
#define SIG_LESSWINDOW SIGWIND
#endif
#endif

#ifdef SIG_LESSWINDOW
/*
 * "Window" change handler
 */
	/* ARGSUSED*/
	public RETSIGTYPE
winch(type)
	int type;
{
	LSIGNAL(SIG_LESSWINDOW, winch);
	sigs |= S_WINCH;
	if (reading)
		intread();
}
#endif

#if MSDOS_COMPILER==WIN32C
/*
 * Handle CTRL-C and CTRL-BREAK keys.
 */
#include "os_defs.h"
#include <windows.h>

	static BOOL WINAPI 
wbreak_handler(dwCtrlType)
	DWORD dwCtrlType;
{
	switch (dwCtrlType)
	{
	case CTRL_C_EVENT:
	case CTRL_BREAK_EVENT:
		sigs |= S_INTERRUPT;
#if HILITE_SEARCH
		set_filter_pattern(NULL, 0);
#endif
		return (TRUE);
	default:
		break;
	}
	return (FALSE);
}
#endif

	static RETSIGTYPE
terminate(type)
	int type;
{
	quit(15);
}

/*
 * Set up the signal handlers.
 */
	public void
init_signals(on)
	int on;
{
	if (on)
	{
		/*
		 * Set signal handlers.
		 */
#if MSDOS_COMPILER==WIN32C
		SetConsoleCtrlHandler(wbreak_handler, TRUE);
#else
		(void) LSIGNAL(SIGINT, u_interrupt);
#endif
#ifdef SIGTSTP
		(void) LSIGNAL(SIGTSTP, secure ? SIG_IGN : stop);
#endif
#ifdef SIGWINCH
		(void) LSIGNAL(SIGWINCH, winch);
#endif
#ifdef SIGWIND
		(void) LSIGNAL(SIGWIND, winch);
#endif
#ifdef SIGQUIT
		(void) LSIGNAL(SIGQUIT, SIG_IGN);
#endif
#ifdef SIGTERM
		(void) LSIGNAL(SIGTERM, terminate);
#endif
	} else
	{
		/*
		 * Restore signals to defaults.
		 */
#if MSDOS_COMPILER==WIN32C
		SetConsoleCtrlHandler(wbreak_handler, FALSE);
#else
		(void) LSIGNAL(SIGINT, SIG_DFL);
#endif
#ifdef SIGTSTP
		(void) LSIGNAL(SIGTSTP, SIG_DFL);
#endif
#ifdef SIGWINCH
		(void) LSIGNAL(SIGWINCH, SIG_IGN);
#endif
#ifdef SIGWIND
		(void) LSIGNAL(SIGWIND, SIG_IGN);
#endif
#ifdef SIGQUIT
		(void) LSIGNAL(SIGQUIT, SIG_DFL);
#endif
#ifdef SIGTERM
		(void) LSIGNAL(SIGTERM, SIG_DFL);
#endif
	}
}

/*
 * Process any signals we have received.
 * A received signal cause a bit to be set in "sigs".
 */
	public void
psignals(VOID_PARAM)
{
	int tsignals;

	if ((tsignals = sigs) == 0)
		return;
	sigs = 0;

#ifdef SIGTSTP
	if (tsignals & S_STOP)
	{
		/*
		 * Clean up the terminal.
		 */
#ifdef SIGTTOU
		LSIGNAL(SIGTTOU, SIG_IGN);
#endif
		clear_bot();
		deinit();
		flush();
		raw_mode(0);
#ifdef SIGTTOU
		LSIGNAL(SIGTTOU, SIG_DFL);
#endif
		LSIGNAL(SIGTSTP, SIG_DFL);
		kill(getpid(), SIGTSTP);
		/*
		 * ... Bye bye. ...
		 * Hopefully we'll be back later and resume here...
		 * Reset the terminal and arrange to repaint the
		 * screen when we get back to the main command loop.
		 */
		LSIGNAL(SIGTSTP, stop);
		raw_mode(1);
		init();
		screen_trashed = 1;
		tsignals |= S_WINCH;
	}
#endif
#ifdef S_WINCH
	if (tsignals & S_WINCH)
	{
		int old_width, old_height;
		/*
		 * Re-execute scrsize() to read the new window size.
		 */
		old_width = sc_width;
		old_height = sc_height;
		get_term();
		if (sc_width != old_width || sc_height != old_height)
		{
			wscroll = (sc_height + 1) / 2;
			calc_jump_sline();
			calc_shift_count();
		}
		screen_trashed = 1;
	}
#endif
	if (tsignals & S_INTERRUPT)
	{
		if (quit_on_intr)
			quit(QUIT_INTERRUPT);
	}
}
