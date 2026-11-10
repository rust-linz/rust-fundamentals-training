#ifndef AREA_CALCULATOR_H
#define AREA_CALCULATOR_H

typedef struct {
    double a;
    double b;
} ellipse_t;

/* Calculates the area of `ellipse` and stores it in `area`.
 * Both pointers must be valid and non-null. */
void area_ellipse(
    const ellipse_t *ellipse,
    double *area
);

#endif /* AREA_CALCULATOR_H */
