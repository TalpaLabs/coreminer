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

  signal(SIGTRAP, handle_sigtrap);

  __asm__("int3;");

  if (g_sigterm_selftrigger != 1) {
    fprintf(stderr, "NONONO EVIL DEBUGGER DETECTED\n");
    return 1;
  } else {
    printf("OK :) No evil debugger.\n");
  }
}
