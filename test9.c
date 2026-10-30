#include<stdio.h>
#include<stdlib.h>
extern unsigned long int foo (unsigned long int, unsigned long int, unsigned long int);

int main(int argc, char** argv){
    unsigned long u = 3, v = 4, w = 2;
    printf("the result is %ld\n", foo(u,v,w));
    return 0;
}