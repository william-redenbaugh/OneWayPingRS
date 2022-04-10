IDIR =../include
CC=g++
CFLAGS=-I$(IDIR) -O2 -Wall -Werror -ansi `pkg-config gtk+-2.0 gstreamer-1.0 --cflags`

ODIR=obj
LDIR =../lib

LIBS=-lm  -lpthread `pkg-config gtk+-2.0 gstreamer-1.0 --libs`

_DEPS = one_way_ping.h 
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

_OBJ = main.o 
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o  $@ $< $(CFLAGS)

onewayping: $(OBJ) 
	$(CC) -o $@ $^ $(CFLAGS) $(LIBS) 

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~ 