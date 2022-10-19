#include "lesstest.h"

extern TermInfo terminfo;

void env_init(EnvBuf* env) {
	env->env_estr = (char*) env->env_buf;
	env->env_list = env->env_buf + sizeof(env->env_buf)/sizeof(char*);
	*--(env->env_list) = NULL;
}

static void env_check(EnvBuf* env) {
	if (env->env_estr >= (const char*) env->env_list) {
		fprintf(stderr, "ENVBUF_SIZE too small!\n");
		abort();
	}
}

static void env_addchar(EnvBuf* env, char ch) {
	*(env->env_estr)++ = ch;
	env_check(env);
}

static void env_addlstr(EnvBuf* env, const char* str, int strlen) {
	while (strlen-- > 0)
		env_addchar(env, *str++);
}

static void env_addstr(EnvBuf* env, const char* str) {
	env_addlstr(env, str, strlen(str));
}

static void env_addlpair(EnvBuf* env, const char* name, int namelen, const char* value) {
	*--(env->env_list) = env->env_estr;
	env_check(env);
	env_addlstr(env, name, namelen);
	env_addstr(env, "=");
	env_addstr(env, value);
	env_addchar(env, '\0');
}

void env_addpair(EnvBuf* env, const char* name, const char* value) {
	env_addlpair(env, name, strlen(name), value);
}

void env_addintpair(EnvBuf* env, const char* name, int value) {
	char buf[64];
	snprintf(buf, sizeof(buf), "%d", value);
	env_addpair(env, name, buf);
}

static int is_less_env(const char* name, int name_len) {
	static char* const less_names[] = {
		"LESS*", "COLUMNS", "LINES", "LANG", "LC_CTYPE", "MORE", NULL
	};
	for (char* const* n = less_names; *n != NULL; ++n) {
		int ln = strlen(*n);
		if (ln == name_len && strncmp(*n, name, ln) == 0)
			return 1;
		if ((*n)[ln-1] == '*' && strncmp(*n, name, ln-1) == 0)
			return 1;
	}
	return 0;
}

static void env_setup(EnvBuf* env, char* const* prog_env, int interactive) {
	struct tcvar { char const* name; char const* value; } tcvars[] = {
		{ "LESS_TERMCAP_am", "1" },
		{ "LESS_TERMCAP_cd", "\33S" },
		{ "LESS_TERMCAP_ce", "\33L" },
		{ "LESS_TERMCAP_cl", "\33A" },
		{ "LESS_TERMCAP_cr", "\33<" },
		{ "LESS_TERMCAP_cm", "\33%p2%d;%p1%dj" },
		{ "LESS_TERMCAP_ho", "\33h" },
		{ "LESS_TERMCAP_ll", "\33l" },
		{ "LESS_TERMCAP_mb", "\33b" },
		{ "LESS_TERMCAP_md", "\33[1m" },
		{ "LESS_TERMCAP_me", "\33[m" },
		{ "LESS_TERMCAP_se", "\33[m" },
		{ "LESS_TERMCAP_so", "\33[7m" },
		{ "LESS_TERMCAP_sr", "\33r" },
		{ "LESS_TERMCAP_ue", "\33[24m" },
		{ "LESS_TERMCAP_us", "\33[4m" },
		{ "LESS_TERMCAP_vb", "\33g" },
		{ "LESS_TERMCAP_kr", terminfo.key_right ? terminfo.key_right : "" },
		{ "LESS_TERMCAP_kl", terminfo.key_left ? terminfo.key_left : "" },
		{ "LESS_TERMCAP_ku", terminfo.key_up ? terminfo.key_up : "" },
		{ "LESS_TERMCAP_kd", terminfo.key_down ? terminfo.key_down : "" },
		{ "LESS_TERMCAP_kh", terminfo.key_home ? terminfo.key_home : "" },
		{ "LESS_TERMCAP_@7", terminfo.key_end ? terminfo.key_end : "" },
	};
	for (int i = 0; i < countof(tcvars); ++i) {
		struct tcvar* tc = &tcvars[i];
		env_addpair(env, tc->name, tc->value);
		log_env(tc->name, strlen(tc->name), tc->value);
	}
	for (char* const* envp = prog_env; *envp != NULL; ++envp) {
		const char* ename = *envp;
		const char* eq = strchr(ename, '=');
		if (eq == NULL) continue;
		if (!interactive || is_less_env(ename, eq-ename)) {
			env_addlpair(env, ename, eq-ename, eq+1);
			log_env(ename, eq-ename, eq+1);
		}
	}
}

const char* get_envp(char* const* envp, const char* name) {
	for (; *envp != NULL; ++envp) {
		const char* ename = *envp;
		const char* eq = strchr(ename, '=');
		if (eq != NULL && strlen(name) == eq-ename && strncmp(name, ename, eq-ename) == 0)
			return eq+1;
	}
	return NULL;
}

char* const* less_envp(char* const* envp, int interactive) {
	static EnvBuf less_env;
	static int init = 0;
	if (!init) {
		env_init(&less_env);
		env_setup(&less_env, envp, interactive);
		init = 1;
	}
	return less_env.env_list;
}
