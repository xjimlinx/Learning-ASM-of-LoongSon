.text
    .align      2
    .global     add_2
    .type       add_2,@function
add_2:
    add.w       $a0,$a0,$a1
    // 返回
    jr          $ra
    .size       add_2, .-add_2

    .global     add_2d
    .type       add_2d,@function
add_2d:
    add.d       $a0,$a0,$a1
    // 返回
    jr          $ra
    .size       add_2d, .-add_2d