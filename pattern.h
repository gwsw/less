/*@@copyright@@*/

#if HAVE_GNU_REGEX
#define __USE_GNU 1
#include <regex.h>
#define PATTERN_TYPE          struct re_pattern_buffer *
#define SET_NULL_PATTERN(name)   name = NULL
#endif

#if HAVE_POSIX_REGCOMP
#include <regex.h>
#ifdef REG_EXTENDED
#define REGCOMP_FLAG    REG_EXTENDED
#else
#define REGCOMP_FLAG    0
#endif
#define PATTERN_TYPE          regex_t *
#define SET_NULL_PATTERN(name)   name = NULL
#endif

#if HAVE_PCRE
#include <pcre.h>
#define PATTERN_TYPE          pcre *
#define SET_NULL_PATTERN(name)   name = NULL
#endif

#if HAVE_PCRE2
#define PCRE2_CODE_UNIT_WIDTH 8
#include <pcre2.h>
#define PATTERN_TYPE          pcre2_code *
#define SET_NULL_PATTERN(name)   name = NULL
#endif

#if HAVE_RE_COMP
char *re_comp LESSPARAMS ((char*));
int re_exec LESSPARAMS ((char*));
#define PATTERN_TYPE          int
#define SET_NULL_PATTERN(name)   name = 0
#endif

#if HAVE_REGCMP
char *regcmp LESSPARAMS ((char*));
char *regex LESSPARAMS ((char**, char*));
extern char *__loc1;
#define PATTERN_TYPE          char **
#define SET_NULL_PATTERN(name)   name = NULL
#endif

#if HAVE_V8_REGCOMP
#include "regexp.h"
extern int reg_show_error;
#define PATTERN_TYPE          struct regexp *
#define SET_NULL_PATTERN(name)   name = NULL
#endif

#if NO_REGEX
#define PATTERN_TYPE          void *
#define SET_NULL_PATTERN(name)   
#endif
