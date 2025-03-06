#include <stdio.h>
#include <unistd.h>

// gdb apparently opens FD(s) 3,4,5 (whereas a typical prog uses only stdin=0,
// stdout=1,stderr=2)
//
// This doesnt work anymore in 2025 (Debian testing)
int main(void) {
  int rc = 0;
  FILE *f = fopen("/tmp", "r");

  int max_fd = fileno(f);
  printf("got fd %d", max_fd);
  if (max_fd > 5) {
    rc = 1;
  }

  fclose(f);
  return rc;
}
