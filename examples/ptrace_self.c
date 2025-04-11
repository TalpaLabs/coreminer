#include <stdio.h>
#include <stdlib.h>
#include <sys/ptrace.h>
int main() {
  if (ptrace(PTRACE_TRACEME, 0) < 0) { // if fails, some other process
    printf("Program is being traced"); // is already tracing
    exit(1);
  }
  printf("Program is not being traced");
  return 0;
}
