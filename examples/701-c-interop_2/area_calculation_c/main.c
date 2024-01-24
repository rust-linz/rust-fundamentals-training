#include <stdio.h>
#include "area_calculation_rs.h"

void main() {
    ellipse_t ellipse = { 10.0, 20.0 };
    double area = area_ellipse(&ellipse);
    printf("Area of ellipse: %f\n", area);
}