#include <stdio.h>
extern int add_2(int a, int b);
extern long add_2d(long a, long b);
int main() {
    // 2^31 越界
    int ret1 = add_2(2147483648, 0);
    long ret2 = add_2d(2147483648*2147483648, 2147483648*2147483648 - 1);
    printf("ret1 = %ld\n", ret1);
    printf("ret2 = %ld\n", ret2);
    return 0;
}