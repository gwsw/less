/*
 * Silly little program to generate the help.c source file
 * from the less.hlp text file.
 * help.c just contains a char array whose contents are 
 * the contents of less.hlp.
 */

#include <stdio.h>

	int
main(argc, argv)
	int argc;
	char *argv[];
{
	int ch;
	int i = 0;

	printf("/* This file was generated from less.hlp */\n");
	printf("#include \"less.h\"\n");
	printf("constant char helpdata[] = {\n");
	while ((ch = getchar()) != EOF)
	{
		if (ch >= ' ' && ch < 0x7f && 
		    ch != '\'' && ch != '\\')
			printf("'%c'", ch);
		else
			printf("0x%02x", ch);
		printf(",");
		if (ch == '\n' || ch == '\r')
			printf("\n");
	}
	printf("};\n");
	printf("constant int size_helpdata = sizeof(helpdata);\n");
	return (0);
}
