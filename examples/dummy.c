#include <stdio.h>
void printer(int i) { printf("foobar %i\n", i); }
int main() {
  for (int i = 0; i < 20; i++) {
    printer(i);
  }
}
