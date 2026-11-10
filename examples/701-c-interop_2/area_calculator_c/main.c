#include <stdio.h>
#include "area_calculator_rs.h"

int main(void) {
    Ellipse ellipse = { 10.0, 20.0 };
    double area = area_ellipse(&ellipse);
    printf("Area of ellipse: %f\n", area);

    /* The Rust function handles NULL safely and returns NaN. */
    printf("Area of NULL: %f\n", area_ellipse(NULL));
    return 0;
}
