CC = gcc
CFLAGS = -Wall -O2
TERMLIB = -ltinfo

LESSTEST_OBJ = display.o lesstest.o parse.o pipeline.o log.o term.o wchar.o
LT_SCREEN_OBJ = lt_screen.o wchar.o
all: lesstest lt_screen 

lesstest: $(LESSTEST_OBJ)
	$(CC) $(CFLAGS) -o lesstest $(LESSTEST_OBJ) $(TERMLIB)

lt_screen: $(LT_SCREEN_OBJ)
	$(CC) $(CFLAGS) -o lt_screen $(LT_SCREEN_OBJ)

clean:
	rm -f lesstest lt_screen *.o
