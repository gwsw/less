#include <stdio.h>
#include <string.h>

int main(int argc, char **argv)
{
	char *p, buf[1024];  /* enough for the longest prototype at funcs.h */
	int cmd = 0, ok = 0;  /* fail if stdin was empty, pipefail-ish */

	if (argc == 2 && !strcmp(argv[1], "funcs"))
	{
		for (cmd = 1; fgets(buf, sizeof buf, stdin); ok = 1)
			if (!strncmp(buf, "public ", 7) && !strchr(buf, ';'))
				printf("%s;", buf);  /* "<proto>\n;" is ok */
	}

	if (argc == 2 && !strcmp(argv[1], "help"))
	{
		printf("/* generated by '%s help' from less.hlp */\n", *argv);
		puts("#include \"less.h\"\n");
		puts("constant char helpdata[] = {");

		for (cmd = 1; fgets(buf, sizeof buf, stdin); puts(""), ok = 1)
			for (p = buf; *p; ++p)
				if (p[0] != '\r' || p[1] != '\n')
					printf("%d,", (int)(unsigned char)*p);

		puts("0 };\n");
		puts("constant int size_helpdata = sizeof(helpdata) - 1;");
	}

	if (cmd && ok)
		return 0;
	fprintf(stderr, !cmd ? "stdin -> stdout: %s MODE  (funcs or help)\n"
	                     : "%s: error: empty input\n", *argv);
	return 1;
}
