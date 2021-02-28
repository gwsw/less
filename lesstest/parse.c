#include <stdio.h>
#include "lesstest.h"

extern int verbose;

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

int parse_setup_name(TestSetup* setup, char const* line, int line_len) {
	setup->setup_name = parse_qstring(&line);
	setup->width = parse_int(&line);
	setup->height = parse_int(&line);
	return 1;
}

int parse_command(TestSetup* setup, char const* line, int line_len) {
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

int parse_textfile(TestSetup* setup, char const* line, int line_len, FILE* fd) {
	char const* filename = parse_qstring(&line);
	if (access(filename, F_OK) == 0) {
		fprintf(stderr, "%s already exists\n", filename);
		return 0;
	}
	int fsize = parse_int(&line);
	int len = strlen(filename)+1;
	setup->textfile = malloc(len);
	strcpy(setup->textfile, filename);
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

TestSetup* new_test_setup(void) {
	TestSetup* setup = (TestSetup*) malloc(sizeof(TestSetup));
	setup->setup_name = NULL;
	setup->textfile = NULL;
	setup->argv = NULL;
	setup->argc = 0;
	setup->width = setup->height = 0;
	return setup;
}

void free_test_setup(TestSetup* setup) {
	unlink(setup->textfile);
	free(setup->setup_name);
	free(setup->textfile);
	int i;
	for (i = 0; i < setup->argc; ++i)
		free(setup->argv[i]);
	free((void*)setup->argv);
	free(setup);
}

int read_zline(FILE* fd, char* line, int line_len) {
	int nread = 0;
	while (nread < line_len) {
		int ch = fgetc(fd);
		if (ch == EOF) return -1;
		if (ch == '\n') break;
		line[nread++] = (char) ch;
	}
	return nread;
}

TestSetup* read_test_setup(FILE* fd) {
	TestSetup* setup = new_test_setup();
	int hdr_complete = 0;
	while (!hdr_complete) {
		char line[10000];
		int line_len = read_zline(fd, line, sizeof(line));
		if (line_len < 0)
			break;
		if (line_len < 1)
			continue;
		switch (line[0]) {
		case ']':
			hdr_complete = 1;
			break;
		case '[':
			if (!parse_setup_name(setup, line+1, line_len-1)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case 'L':
			if (!parse_command(setup, line+1, line_len-1)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case 'F':
			if (!parse_textfile(setup, line+1, line_len-1, fd)) {
				free_test_setup(setup);
				return NULL;
			}
			break;
		case '!':
			break;
		default:
			break;
		}
	}
	if (setup->textfile == NULL || setup->argv == NULL || setup->width < 1 || setup->height < 1) {
		free_test_setup(setup);
		return NULL;
	}
	if (verbose) { fprintf(stderr, "setup: [%s] textfile %s, %dx%d\n", setup->setup_name, setup->textfile, setup->width, setup->height); print_strings("argv:", setup->argv); }
	return setup;
}

