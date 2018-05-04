// Kompilieren mit: clang -std=c11 -g -lbsd -o main_c main.c && ./main_c
#include <stdio.h>

const char *data[] = {
    "Peter Arbeitsloser",
    "Sandra Systemadministratorin",
    "Peter Koch",
};

int main() {
  const int length = sizeof(data) / sizeof(data[0]);
  if (length != 3) {
    printf("length: %i; should be 3\n", length);
    return 1;
  }

  int index = 0;
kopf:
  if (!(index < length)) {
    goto ende;
  }
  const char *name = data[index];
  printf("%i: %s\n", index, name);
  index += 1;
  goto kopf;
ende:


  for (int index = 0; index < length; index++) {
    const char *name = data[index];
    printf("%i: %s\n", index, name);
  }
}
