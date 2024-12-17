#include <stdio.h>
#include <string.h>

int main(void) {
    char c[100];

    printf("Please enter key:\n");
    scanf("%s", c);
    int ret = strcmp(c, "__stack_check");

    if (!ret) {
        printf("Good job.\n");
    } else {
        printf("Nope.\n");
    }
    return (0);
}
