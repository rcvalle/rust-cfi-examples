#include <stdio.h>
#include <stdlib.h>

extern void hello_from_rust(void);

int
main(int argc, char *argv[])
{
    printf("Hello, world!\n");
    hello_from_rust();
}
