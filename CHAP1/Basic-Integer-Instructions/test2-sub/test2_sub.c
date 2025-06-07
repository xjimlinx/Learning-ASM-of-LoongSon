#include <stdio.h>
extern int sub_2(int a, int b);
int main() {
    int ret = sub_2(1000000, 200);
    printf("ret = %d\n", ret);
    return 0;
}