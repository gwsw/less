#include <stdio.h>
#include "lesstest.h"

char* parse_qstring(const char** s) {
	while (*(*s) == ' ') ++(*s);
	if (*(*s)++ != '"') return NULL;
	char const* start = *s;
	while (*(*s) != '"' && *(*s) != '\0') ++(*s);
	char* ret = strndup(start, (*s)-start);
	if (*(*s) == '"') ++(*s);
	return ret;
}

int parse_int(const char** s) {
	return (int) strtol(*s, (char**)s, 0);
}

int parse_setup_name(TestSetup* setup, char const* line) {
	setup->setup_name = parse_qstring(&line);
	setup->width = parse_int(&line);
	setup->height = parse_int(&line);
	return 1;
}

int parse_command(TestSetup* setup, char const* line) {
	setup->argv = (char**) malloc(32*sizeof(char const*));
	setup->argc = 0;
	for (;;) {
		char const* arg = parse_qstring(&line);
		setup->argv[setup->argc] = (char*) arg;
		if (arg == NULL) break;
		setup->argc++;
	}
	return 1;
}

int parse_textfile(TestSetup* setup, char const* line, char const* testdir, FILE* fd) {
	char const* filename = parse_qstring(&line);
	int fsize = parse_int(&line);
	int len = strlen(testdir)+strlen(filename)+10;
	setup->textfile = malloc(len);
	snprintf(setup->textfile, len, "%s/%06d-%s", testdir, rand() % 1000000, filename);
	FILE* textfd = fopen(setup->textfile, "w");
	if (textfd == NULL) {
		fprintf(stderr, "cannot create %s\n", setup->textfile);
		return 0;
	}
	int nread = 0;
	while (nread < fsize) {
		char buf[4096];
		int chunk = fsize - nread;
		if (chunk > sizeof(buf)) chunk = sizeof(buf);
		size_t len = fread(buf, 1, chunk, fd);
		fwrite(buf, 1, len, textfd);
		nread += len;
	}
	fclose(textfd);
	return 1;
}
