// Kompilieren mit: clang -std=c11 -g -lbsd -o n_von_pi main.c && ./n_von_pi

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>
#include <errno.h>

int main(int argc, char *argv[]) {
  char *pi = "3.1415926f32";
  int stelle = -1;
  int err = 1;

  while(1) {
    printf("wie vielte Stelle? ");
    err = scanf("%d", &stelle);

    if (err == 0 || errno != 0) {
      printf("invalid entry\n");
      errno = 0;
      while (getchar() != '\n');
      continue;
    }

    printf("Eingabe: %d\n", stelle);
    printf("Gewünschte Stelle: '%c'\n", pi[stelle]);
  }

  // unreachable
  return 0;
}
