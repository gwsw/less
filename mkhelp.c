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

	printf("#include \"less.h\"\n");
	printf("constant char helpdata[] = {\n");
	while ((ch = getchar()) != EOF)
	{
		if (i > 0)
			printf(",");
		if ((i++ % 16) == 0)
			printf("\n");
		printf("0x%02x", ch);
	}
	printf("};\n");
	printf("int size_helpdata = sizeof(helpdata);\n");
	return (0);
}
