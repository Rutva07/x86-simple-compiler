#include <stdio.h>
#include<stdlib.h>
extern unsigned long foo(unsigned long a, unsigned long b);

int main(void) {
    unsigned long a = 100, b = 200;
    printf("the result is %ld\n", foo(a, b));
    return 0;
}