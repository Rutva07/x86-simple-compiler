#include<stdio.h>
#include<stdlib.h>
extern unsigned long int foo (unsigned long int, unsigned long int);

int main(int argc, char** argv){
    unsigned long u = 1, v = 10;
    printf("the result is %ld\n", foo(u,v));
    return 0;
}