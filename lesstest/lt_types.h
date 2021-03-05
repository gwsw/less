#include <signal.h>

typedef unsigned long wchar;
typedef unsigned char byte;
typedef unsigned char Attr;
typedef unsigned char Color;

#define ESC                '\33'

#define ATTR_BOLD          (1<<0)
#define ATTR_UNDERLINE     (1<<1)
#define ATTR_STANDOUT      (1<<2)
#define ATTR_BLINK         (1<<3)

#define LTSIG_SCREEN_READY  SIGUSR1
#define LTSIG_SCREEN_DUMP   SIGUSR2

#undef countof
#define countof(a) (sizeof(a)/sizeof(*a))
