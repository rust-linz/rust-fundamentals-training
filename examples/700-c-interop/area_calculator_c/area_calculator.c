#include "area_calculator.h"
#include <math.h>

void area_ellipse(
    ellipse_t *ellipse,
    double *area
) {
    *area = ellipse->a * ellipse->b * M_PI;
}
