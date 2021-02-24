CC = gcc
CFLAGS = -Wall -O2
TERMLIB = -ltinfo

all: lesstest lt_screen 

lesstest: lesstest.o pipeline.o
	$(CC) $(CFLAGS) -o lesstest lesstest.o pipeline.o $(TERMLIB)

lt_screen: lt_screen.o
	$(CC) $(CFLAGS) -o lt_screen lt_screen.o

clean:
	rm -f lesstest lt_screen *.o
