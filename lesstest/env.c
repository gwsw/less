#include "lesstest.h"

extern TermInfo terminfo;

void env_init(EnvBuf* env) {
	env->env_estr = (char*) env->env_buf;
	env->env_list = env->env_buf + sizeof(env->env_buf)/sizeof(char*);
	*--(env->env_list) = NULL;
}

void env_check(EnvBuf* env) {
	if (env->env_estr >= (const char*) env->env_list) {
		fprintf(stderr, "ENVBUF_SIZE too small!\n");
		abort();
	}
}

void env_addchar(EnvBuf* env, char ch) {
	*(env->env_estr)++ = ch;
	env_check(env);
}

void env_addlstr(EnvBuf* env, const char* str, int strlen) {
	while (strlen-- > 0)
		env_addchar(env, *str++);
}

void env_addstr(EnvBuf* env, const char* str) {
	env_addlstr(env, str, strlen(str));
}

void env_addlpair(EnvBuf* env, const char* name, int namelen, const char* value) {
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

void env_setup(EnvBuf* env, char* const* prog_env, const char* env_prefix) {
	env_addpair(env, "LESS_TERMCAP_am", "1");
	env_addpair(env, "LESS_TERMCAP_cd", "\33S");
	env_addpair(env, "LESS_TERMCAP_ce", "\33L");
	env_addpair(env, "LESS_TERMCAP_cl", "\33A");
	env_addpair(env, "LESS_TERMCAP_cr", "\33<");
	env_addpair(env, "LESS_TERMCAP_cm", "\33%p2%d;%p1%dj");
	env_addpair(env, "LESS_TERMCAP_ho", "\33h");
	env_addpair(env, "LESS_TERMCAP_ll", "\33l");
	env_addpair(env, "LESS_TERMCAP_mb", "\33b");
	env_addpair(env, "LESS_TERMCAP_md", "\33d");
	env_addpair(env, "LESS_TERMCAP_md", "\33e");
	env_addpair(env, "LESS_TERMCAP_se", "\33t");
	env_addpair(env, "LESS_TERMCAP_so", "\33s");
	env_addpair(env, "LESS_TERMCAP_sr", "\33r");
	env_addpair(env, "LESS_TERMCAP_ue", "\33v");
	env_addpair(env, "LESS_TERMCAP_uo", "\33u");
	env_addpair(env, "LESS_TERMCAP_vb", "\33g");
	env_addpair(env, "LESS_TERMCAP_kr", terminfo.key_right ? terminfo.key_right : "");
	env_addpair(env, "LESS_TERMCAP_kl", terminfo.key_left ? terminfo.key_left : "");
	env_addpair(env, "LESS_TERMCAP_ku", terminfo.key_up ? terminfo.key_up : "");
	env_addpair(env, "LESS_TERMCAP_kd", terminfo.key_down ? terminfo.key_down : "");
	env_addpair(env, "LESS_TERMCAP_kh", terminfo.key_home ? terminfo.key_home : "");
	env_addpair(env, "LESS_TERMCAP_@7", terminfo.key_end ? terminfo.key_end : "");

	char* const* envp;
	int len = strlen(env_prefix);
	for (envp = prog_env; *envp != NULL; ++envp) {
		if (strncmp(*envp, env_prefix, len) == 0) {
			const char* ename = *envp + len;
			const char* eq = strchr(ename, '=');
			if (eq != NULL) {
				env_addlpair(env, ename, eq-ename, eq+1);
				log_env(ename, eq-ename, eq+1);
			}
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

char* const* less_envp(char* const* envp, const char* env_prefix) {
	static EnvBuf less_env;
	static int init = 0;
	if (!init) {
		env_init(&less_env);
		env_setup(&less_env, envp, env_prefix);
		init = 1;
	}
	return less_env.env_list;
}
