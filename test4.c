#include<stdlib.h>
#include<stdio.h>

extern unsigned long int foo (unsigned long int);

int main(int argc, char** argv){
    unsigned long a = 10;
    printf("the result is %ld\n", foo(a));
    return 0;
}