CC     = gcc
CFLAGS = -O2 -Wall -fPIC

# -fPIC (Position-Independent Code) is required for shared libraries and
# harmless for static ones, so we use it for the single object file that
# feeds all targets.

# `make` / `make all` builds the static library plus the dynamic library
# for the current OS.
ifeq ($(shell uname -s),Darwin)
all: libqueens.a libqueens.dylib
else
all: libqueens.a libqueens.so
endif

queens.o: queens.c queens.h
	$(CC) $(CFLAGS) -c queens.c -o queens.o

libqueens.a: queens.o
	ar rcs libqueens.a queens.o

# macOS: -dynamiclib produces a .dylib (used by 03-dynamic-lib on macOS).
libqueens.dylib: queens.o
	$(CC) -dynamiclib -o libqueens.dylib queens.o

# Linux: -shared produces a .so (used by 03-dynamic-lib on Linux).
libqueens.so: queens.o
	$(CC) -shared -o libqueens.so queens.o

clean:
	rm -f *.o *.a *.dylib *.so

.PHONY: all clean
