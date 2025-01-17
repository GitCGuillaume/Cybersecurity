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

//101
//108
//97
//98
//101
//114
//101
//https://faydoc.tripod.com/cpu/jae.htm
int main(void) {
    int a;
    char c[100];
    char d[9];
    size_t j;
    int i;

    a = 0;
    printf("Please enter key: ");
    a = scanf("%23s", c);
    if (a != 1)
        no();
    if (c[1] != '0') {
        no();
    }
    if (c[0] != c[1]) {
        no();
    }
    fflush(stdin);
    memset(d, 0, sizeof(d));
    d[0] = 'd';
    d[1] = 0;
    j = 0x2;
    i = 0x1;
    while (1) {
        a = strlen(d);
        int b = 0;
        if (!(a >= 0x8)) {
            //cmp ecx(23) eax(j)
            //setb
            b = j < strlen(c);//set à 1
        }
        //test 0x1 $al (test $al & 1)
        //jne
        //jmp(break?)
        //line 286 test & 1
        if (!(b & 1))
            break ;
        char e[4];
        e[0] = c[j];
        e[1] = c[j + 1];
        e[2] = c[j + 2];
        e[3] = 0;
        a = atoi(e);
        j += 3;
        d[i] = a;
        i += 1;
    }
    if (strcmp(d, "delabere"))
        no();
    ok();
    return 0;
}