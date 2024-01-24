#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Ellipse {
  double a;
  double b;
} Ellipse;

double area_ellipse(const struct Ellipse *ellipse);
