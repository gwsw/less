// os_windows_defs.h - defines to trim windows.h down as much as possible
// ---------------------------------------------------------------------------

// If defined, the following flags inhibit definition of the indicated items.

#define WIN32_LEAN_AND_MEAN	// - Excludes less common APIs like Cryptography DDE, RPC, Shell and Windows Sockets
#define NOGDICAPMASKS 		// - CC_*, LC_*, PC_*, CP_*, TC_*, RC_
#define NOWINSTYLES 		// - WS_*, CS_*, ES_*, LBS_*, SBS_*, CBS_*
#define NOSYSMETRICS 		// - SM_*
#define NOMENUS 			// - MF_*
#define NOICONS 			// - IDI_*
#define NOKEYSTATES 		// - MK_*
#define NOSYSCOMMANDS 		// - SC_*
#define NORASTEROPS 		// - Binary and Tertiary raster ops
#define NOSHOWWINDOW 		// - SW_*
#define OEMRESOURCE 		// - OEM Resource values
#define NOATOM				// - Atom Manager routines
#define NOCLIPBOARD 		// - Clipboard routines
#define NOCOLOR 			// - Screen colors
#define NOCTLMGR 			// - Control and Dialog routines
#define NODRAWTEXT 			// - DrawText() and DT_*
#define NOGDI 				// - All GDI defines and routines
#define NOKERNEL 			// - All KERNEL defines and routines
#define NOMEMMGR 			// - GMEM_*, LMEM_*, GHND, LHND, associated routines
#define NOMETAFILE 			// - typedef METAFILEPICT
#define NOMINMAX 			// - Macros min(a,b) and max(a,b)
#define NOMSG 				// - typedef MSG and associated routines
#define NOOPENFILE 			// - OpenFile(), OemToAnsi, AnsiToOem, and OF_*
#define NOSCROLL 			// - SB_* and scrolling routines
#define NOSERVICE 			// - All Service Controller routines, SERVICE_ equates, etc.
#define NOSOUND 			// - Sound driver routines
#define NOTEXTMETRIC 		// - typedef TEXTMETRIC and associated routines
#define NOWH 				// - SetWindowsHook and WH_*
#define NOWINOFFSETS 		// - GWL_*, GCL_*, associated routines
#define NOCOMM 				// - COMM driver routines
#define NOKANJI 			// - Kanji support stuff.
#define NOHELP 				// - Help engine interface.
#define NOPROFILER 			// - Profiler interface.
#define NODEFERWINDOWPOS 	// - DeferWindowPos routines
#define NOMCX 				// - Modem Configuration Extensions

// ---------------------------------------------------------------------------
