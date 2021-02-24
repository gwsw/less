CC = gcc
CFLAGS = -Wall -O2
TERMLIB = -ltinfo

LESSTEST_OBJ = lesstest.o parse.o pipeline.o log.o
all: lesstest lt_screen 

lesstest: $(LESSTEST_OBJ)
	$(CC) $(CFLAGS) -o lesstest $(LESSTEST_OBJ) $(TERMLIB)

lt_screen: lt_screen.o
	$(CC) $(CFLAGS) -o lt_screen lt_screen.o

clean:
	rm -f lesstest lt_screen *.o
