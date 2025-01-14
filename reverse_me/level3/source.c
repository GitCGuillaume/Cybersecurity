#include <stdlib.h>
#include <string.h>
#include <stdio.h>

void    ___syscall_malloc(void) {
    puts("Nope.");
    exit(1);
}

void    ____syscall_malloc(void) {
    puts("Good job.");
}

int main(void) {
    char c[100];
    char d[9];
    char  e[33] = {0};
    //int  f;
    int  g;
    int  h;
    printf("Please enter key:\n");
    int res = scanf("%23s", c);
printf("r:%d\n", res);
    if (res != 1)
        ___syscall_malloc();
    //cmp eax:50(2)
    if (c[1] != '2')
        ___syscall_malloc();
    if (c[0] != '4')
        ___syscall_malloc();
    fflush(stdin);
    memset(d, 0, sizeof(d));
    e[0] = 42;
    e[1] = 0;
    g = 2;
    h = 1;
    res = 0;
    while (1) {
        if (8 < strlen(e)) {
            res = g < strlen(c);
        }
        printf("br:%d\n", res & 1);
        if (!(res & 1))
            break ;
        char e[4];
        e[0] = c[g];
        e[1] = c[g + 1];
        e[2] = c[g + 2];
        e[3] = 0;
        res = atoi(e);
        g += 3;
        d[h] = res;
        h += 1;
    }
    //strcmp(d, ********)
}