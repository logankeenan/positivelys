#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

char *make_app_request(const char *to);

void make_app_request_free(char *s);
