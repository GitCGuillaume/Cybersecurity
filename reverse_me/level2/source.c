//atoi
//printf
//scanf
//strcmp
//memset
//main
//fflush
//exit
//strlen
//ok
//n
//no
//xd
//xxd
//xxxd
//ww
//xyxxd

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void    no() {
    printf("Nope.\n");
    exit(1);
}

void    ok() {
    printf("Good job.\n");
    exit(0);
}

int main(void) {
    int a;
    char c[100];

    a = 0;
    printf("Please enter key:\n");
    a = scanf("%23s", c);
    if (a == 0)
        no();
    if (c[1] != '0') {
        no();
    }
    if (c[0] != c[1]) {
        no();
    }
    fflush(stdin);
    memset(c, 0, sizeof(c));
    a = strcmp(&c[2], "delabere");
    return 0;
}