#include <stdio.h>
void printer(char i) { printf("foobar %i\n", i); }
int main() {
  for (char i = 0; i < 20; i++) {
    printer(i);
  }
}
