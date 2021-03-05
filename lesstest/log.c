#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include "lesstest.h"

static FILE* logf = NULL;

int log_open(const char* logfile) {
	if (logf != NULL) fclose(logf);
	logf = (strcmp(logfile, "-") == 0) ? stdout : fopen(logfile, "w");
	if (logf == NULL) {
		fprintf(stderr, "cannot create %s\n", logfile);
		return 0;
	}
	return 1;
}

void log_close(void) {
	if (logf == NULL) return;
	if (logf == stdout) return;
	fclose(logf);
	logf = NULL;
}

int log_file_header(void) {
	if (logf == NULL) return 0;
	fprintf(logf, "!lesstest!\n");
	return 1;
}

int log_test_header(const char* testname, int screen_width, int screen_height, const char* charset, char* const* argv, int argc, const char* textfile) {
	if (logf == NULL) return 0;
	fprintf(logf, "[ \"%s\" %d %d \"%s\"\n", testname, screen_width, screen_height, charset != NULL ? charset : "");
	if (!log_command(argv, argc))
		return 0;
	if (!log_textfile(textfile))
		return 0;
	fprintf(logf, "]\n");
	return 1;
}

int log_tty_char(wchar ch) {
	if (logf == NULL) return 0;
	fprintf(logf, "+%lx\n", ch);
	return 1;
}

int log_screen(const byte* img, int len) {
	if (logf == NULL) return 0;
	fwrite("=", 1, 1, logf);
	fwrite(img, 1, len, logf);
	fwrite("\n", 1, 1, logf);
	return 1;
}

int log_command(char* const* argv, int argc) {
	if (logf == NULL) return 0;
	fprintf(logf, "L");
	int a;
	for (a = 1; a < argc; ++a)
		fprintf(logf, " \"%s\"", argv[a]);
	fprintf(logf, "\n");
	return 1;
}

int log_textfile(const char* textfile) {
	if (logf == NULL) return 0;
	struct stat st;
	if (stat(textfile, &st) < 0) {
		fprintf(stderr, "cannot stat %s\n", textfile);
		return 0;
	}
	FILE* fd = fopen(textfile, "r");
	if (fd == NULL) {
		fprintf(stderr, "cannot open %s\n", textfile);
		return 0;
	}
	const char* basename = rindex(textfile, '/');
	if (basename == NULL) basename = textfile; else ++basename;
	fprintf(logf, "F \"%s\" %ld\n", basename, (long) st.st_size);
	off_t nread = 0;
	while (nread < st.st_size) {
		char buf[4096];
		size_t n = fread(buf, 1, sizeof(buf), fd);
		if (n <= 0) {
			fprintf(stderr, "read only %ld/%ld from %s\n", (long) nread, (long) st.st_size, textfile);
			fclose(fd);
			return 0;
		}
		nread += n;
		fwrite(buf, 1, n, logf);
	}
	fclose(fd);
	return 1;
}
