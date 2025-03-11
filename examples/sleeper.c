#include <stdio.h>
#include <unistd.h>

int main() {
  printf("starting...");
  while (1) {
    printf("wakeup\n");
    sleep(1);
  }
}
