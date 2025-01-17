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
    size_t  g;
    int  h;

    printf("Please enter key: ");
    int res = scanf("%23s", c);

    if (res != 1)
        ___syscall_malloc();
    if (c[1] != '2')
        ___syscall_malloc();
    if (c[0] != '4')
        ___syscall_malloc();
    fflush(stdin);
    memset(d, 0, sizeof(d));
    d[0] = 42;
    d[1] = 0;
    g = 2;
    h = 1;
    while (1) {
        res = 0;
        if (!(strlen(d) >= 0x8)) {
            res = g < strlen(c);
        }
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
    unsigned int res2 = strcmp(d, "********");
    unsigned int i = res2;

    res2 -= 0xfffffffe;
    if (res2) {
        res2 = i;
        res2 -= 0xffffffff;
        if (res2) {
            res2 = i;
            if (res) { //test eax eax
                res2 = i;
                res2 -= 1;
                if (res) {
                    res2 = i;
                    res2 -= 2;
                     if (res) {
                        res2 = i;
                        res2 -= 3;
                        if (res) {
                            res2 = i;
                            res2 -= 4;
                            if (res) {
                                res2 = i;
                                res2 -= 5;
                                if (res) {
                                    res2 = i;
                                    res2 -= 0x73;
                                     if (res) {
                                        ___syscall_malloc();
                                        return (1);
                                    }
                                    ___syscall_malloc();
                                    return (1);
                                }
                                ___syscall_malloc();
                                return (1);
                            }
                            ___syscall_malloc();
                            return (1);
                        }
                        ___syscall_malloc();
                        return (1);
                    }
                    ___syscall_malloc();
                    return (1);
                }
                ___syscall_malloc();
                return (1);
            }
            ____syscall_malloc();
            return (0) ;
        }
        ___syscall_malloc();
        return (1);
    }
    ___syscall_malloc();
    return (1);
}