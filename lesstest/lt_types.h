#include <signal.h>

typedef unsigned long wchar;
typedef unsigned char byte;
typedef unsigned char Attr;
typedef unsigned char Color;

#define ATTR_BOLD          (1<<0)
#define ATTR_UNDERLINE     (1<<1)
#define ATTR_STANDOUT      (1<<2)
#define ATTR_BLINK         (1<<3)

#define ESC                '\33'
#define UNICODE_MAX_BYTES   4
#define MAX_SCREENBUF_SIZE  8192
#define LT_ENV_PREFIX       "LT_"

#define LTSIG_SCREEN_READY  SIGUSR1
#define LTSIG_SCREEN_DUMP   SIGUSR2

#define RUN_OK              0
#define RUN_ERR             1

#define is_ascii(ch)        ((ch) >= ' ' && (ch) < 0x7f)
#define pr_ascii(ch)        (is_ascii(ch) ? ((char)ch) : '.')

#undef countof
#define countof(a) (sizeof(a)/sizeof(*a))
