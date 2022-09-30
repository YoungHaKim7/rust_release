#include <stdio.h>

extern const char *get_string(void);

int main()
{
  char *string = get_string();
  printf("%s", string);
}