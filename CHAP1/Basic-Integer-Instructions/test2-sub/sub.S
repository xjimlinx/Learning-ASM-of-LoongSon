.text
    .align      2
    .global     sub_2
    .type       sub_2,@function
sub_2:
    sub.w       $a0,$a0,$a1
    // 返回
    jr          $ra
    // .-sub_2 是一个特殊符号，表示
    // 当前地址（.）减去 sub_2 的起始地址
    // 即函数体的大小。
    .size       sub_2, .-sub_2
