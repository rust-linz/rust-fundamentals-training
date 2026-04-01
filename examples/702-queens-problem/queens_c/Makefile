CC     = gcc
CFLAGS = -O2 -Wall -fPIC

# -fPIC (Position-Independent Code) is required for shared libraries and
# harmless for static ones, so we use it for the single object file that
# feeds both targets.

all: libqueens.a libqueens.dylib

queens.o: queens.c queens.h
	$(CC) $(CFLAGS) -c queens.c -o queens.o

libqueens.a: queens.o
	ar rcs libqueens.a queens.o

# On macOS, -dynamiclib produces a .dylib (equivalent to -shared on Linux).
libqueens.dylib: queens.o
	$(CC) -dynamiclib -o libqueens.dylib queens.o

clean:
	rm -f *.o *.a *.dylib

.PHONY: all clean
