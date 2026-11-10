#include "area_calculator.h"
#include <math.h>

/* M_PI is POSIX, not standard C. Provide it if <math.h> does not. */
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

void area_ellipse(
    const ellipse_t *ellipse,
    double *area
) {
    *area = ellipse->a * ellipse->b * M_PI;
}
