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

//101
//108
//97
//98
//101
//114
//101
int main(void) {
    int a;
    char c[100];
    char d[9];
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
    memset(d, 0, sizeof(d));
    d[0] = 0x64:
    d[1] = 0;
    d[2] = 0x2;
    d[3] = 0x1;

    //strlen(char)
    //strlen(str entière)
    //cmp 0x8
    //atoi(ABC)atoi(DEF...)
    //addition pour créer delabere
    //strcmp()
    while (1) {
        a = 0;
        if (strlen(d) != 0x8) {
            a = strlen(c);//ecx:23 eax:2 puis 5
            //test 0x1 $al (test $al & 1)
            //jne
            //jmp(break?)
        }
        //cmp (test & 1)
        if (!(a & 1))
            break ;
        int a1 = c[a];
        int a2 = c[a + 1];
        int a3 = c[a + 2];
        //atoi
        //c++?
        i++;
    }
    if (strcmp(d), "delabere")
        no();
    ok();
    //a = strcmp(&c[2], "delabere");
    printf("%x\n", '\n');
    printf("res:%d\n", atoi("del"));

    printf("res:%d\n", atoi("abe"));

    printf("res:%d\n", atoi("re"));
    return 0;
}