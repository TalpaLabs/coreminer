#include <signal.h>
#include <stdio.h>
#include <unistd.h>

static int REALLY_STOP_NOW = 0;

void sig_hanlder(int signum) {
  if (signum == SIGTERM || signum == SIGINT) {
    REALLY_STOP_NOW++;
  }

  printf("got signal %d\n", signum);
}

int main(void) {

  for (int sig = 0; sig < NSIG; sig += 1) {
    // cant handle these
    if (sig == SIGKILL || sig == SIGSTOP) {
      continue;
    }
    signal(sig, sig_hanlder);
  }

  while (REALLY_STOP_NOW < 3) {
    sleep(1);
  }
  printf("got many requests to actually exit, so I'm exiting now");
}
