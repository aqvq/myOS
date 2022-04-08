# os/src/task/switch.asm
.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm

.altmacro
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm

    .section .text
    .globl __switch

__switch:
    # 阶段1
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    # 阶段2
    # save kernel stack of current task
    sd sp, 8(a0)
    # save ra & s0~s11 of current execution
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr

    # 阶段3
    # restore ra & s0~s11 of current executrion
    ld ra, 8(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
    .endr
    # restore kernel stack of next task
    ld sp, 8(a1)
    # 阶段4
    ret
