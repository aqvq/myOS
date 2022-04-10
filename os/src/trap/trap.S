# os/src/trap/trap.S

# 注意我们需要在 trap.S 开头加上 .altmacro 才能正常使用 .rept 命令
.altmacro
# 这是一个带参数的宏
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    # 还记得 BASE << 2 吗
    .align 2
__alltraps:
    # swap sp and sscratch
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8
    # save general-purpose registers
    # 栈的布局应该是这样的：
    # 高地址
    # sepc
    # sstatus
    # x31
    # ...
    # x2 (这里保存的应该是sscratch的值，即用户栈指针)
    # x1
    # x0
    # 低地址
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler

__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    # mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)

    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2

    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)

    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    sret