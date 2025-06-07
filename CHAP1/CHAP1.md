# CHAP 1

首先编写一个文件add.S

```assembly
.text
    .align      2
    .global     add_f
    .type       add_f,@function
add_f:
    add.w       $a0,$a0,$a1
    add.w       $a0,$a0,$a2
    add.w       $a0,$a0,$a3
    // 返回
    jr          $ra
    .size       add_f,  .-add_f
```

然后编写一个test.c文件

```c
#include <stdio.h>
extern int add_f(int a, int b, int c, int d);
int main() {
    int ret = add_f(1, 2, 3, 4);
    printf("ret = %d\n", ret);
    return 0;
}
```

然后在终端命令行中执行：

```shell
loong-arch64-unknown-linux-gnu-gcc test.c add.S -o test_add
```

得到test_add程序，然后使用qemu-loongarch64来执行该程序

```shell
qemu-loongarch64 ./test_add
```

然后就得到了理想的输出：

```shell
ret = 10
```
