libareacalc.a: area_calculator.o
	ar rcs libareacalc.a area_calculator.o

area_calculator.o: area_calculator.c
	gcc -c area_calculator.c -o area_calculator.o

clean:
	rm -f *.o *.a
	