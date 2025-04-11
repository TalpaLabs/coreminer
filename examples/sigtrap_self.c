#include <signal.h>
#include <stdio.h>
#include <unistd.h>

static int g_sigterm_selftrigger = 0;

void handle_sigtrap(int signum) {
  if (signum == SIGTRAP) {
    g_sigterm_selftrigger++;
  }
}

int main(int argc, char **argv) {

  // register signal handler
  signal(SIGTRAP, handle_sigtrap);
  __asm__("int3;"); // this instruction will raise SIGTRAP,
                    // which usually picked up by a debugger
  if (g_sigterm_selftrigger != 1) {
    fprintf(stderr, "DEBUGGER DETECTED\n");
    return 1;
  } else {
    printf("No debugger.\n");
  }
  return 0;
}
