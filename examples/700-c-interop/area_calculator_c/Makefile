libareacalc.a: area_calculator.o
	ar rcs libareacalc.a area_calculator.o

area_calculator.o: area_calculator.c area_calculator.h
	gcc -Wall -std=c11 -c area_calculator.c -o area_calculator.o

.PHONY: clean
clean:
	rm -f *.o *.a
