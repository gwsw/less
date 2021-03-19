CC = gcc
CFLAGS = -Wall -g
TERMLIB = -lncurses

all: lesstest lt_screen 

LESSTEST_OBJ = display.o env.o lesstest.o parse.o pipeline.o log.o run.o term.o wchar.o
lesstest: $(LESSTEST_OBJ)
	$(CC) $(CFLAGS) -o lesstest $(LESSTEST_OBJ) $(TERMLIB)

LT_SCREEN_OBJ = lt_screen.o unicode.o wchar.o
lt_screen: $(LT_SCREEN_OBJ)
	$(CC) $(CFLAGS) -o lt_screen $(LT_SCREEN_OBJ)

*.o: lesstest.h lt_types.h wchar.h

clean:
	rm -f lesstest lt_screen *.o
