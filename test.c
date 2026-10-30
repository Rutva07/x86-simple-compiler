#include<stdio.h>
#include<stdlib.h>
extern unsigned long int foo (unsigned long int, unsigned long, unsigned long, unsigned long, unsigned long);

int main(int arg, char** argv){
    unsigned long int a = 1, b = 2, c = 3, d=4, e=5, result = 0;
    result = foo(a, b, c, d, e);
    printf("the result is %lu\n", result);
    return 0;
}