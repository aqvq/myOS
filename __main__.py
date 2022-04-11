# user/build.py

import enum
import os
from sys import prefix

base_address = 0x80400000
step = 0x20000
linker = 'user/src/linker.ld'
prefix = '[build.py]'
DEBUG_OS = False
K210SERIALPORT = "/dev/ttyUSB0"
BOOTLOADER = "./bootloader/rustsbi-k210.bin"
KERNEL_BIN = "./os/target/riscv64gc-unknown-none-elf/release/os.bin"
K210_BOOTLOADER_SIZE = "131072"
def exit_function():
    """
    exit_function some actions when exit
    """    
    exit()

def build_apps(apps):
    """
    build_apps build user applications

    Args:
        apps (list): user application name list 
    """
    for app_id,app in enumerate(apps):
        if os.system('cd user && cargo build --bin {} --release'.format(app)) != 0:
            print('{} Error on running cargo build apps!'.format(prefix))
            exit_function()
        if os.system('cd user && rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/{0}  -O binary target/riscv64gc-unknown-none-elf/release/{0}.bin'.format(app)) !=0 :
            print('{} Error on running rust objcopy'.format(prefix))
            exit_function()
        print('{} application {} start with address {}'.format(prefix, app, hex(base_address)))
    print('{} Building apps succeeded!'.format(prefix))


def build_os():
    """
    build_os build operation system
    """
    if os.system('cd os && cargo build --release') != 0:
        print('{} Error on running cargo build os'.format(prefix))
        exit_function()
    if os.system('cd os && rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin') != 0:
        print('{} Error on running rust objcopy'.format(prefix))
        exit_function()
    print('{} Building OS succeeded!'.format(prefix))

def run_os():
    """
    run_os run operation system
    """
    if os.system('qemu-system-riscv64 -machine virt -nographic -bios ./bootloader/rustsbi-qemu.bin -device loader,file=./os/target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000') != 0:
        print('{} Error on running qemu'.format(prefix))
        exit_function()

def debug_os():
    """
    debug_os debug operation system
    """
    if os.system('qemu-system-riscv64 -machine virt -nographic -bios ./bootloader/rustsbi-qemu.bin -device loader,file=./os/target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 -s -S &') != 0:
        print('{} Error on running qemu'.format(prefix))
        exit_function()
    if os.system(r"riscv64-unknown-elf-gdb -ex 'file ./os/target/riscv64gc-unknown-none-elf/release/os' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'") != 0:
        print('{} Error on running gdb'.format(prefix))
        exit_function()


def generate_link_app(apps):
    """
    generate_link_app generate link_app assembly file

    Args:
        apps (list): user application list 
    """    
    with open('os/src/link_app.S', 'w') as f:
        f.write("""# This assembly file is automatically generated by python
# os/src/link_app.S

    .align 3
    .section .data
    .global _num_app
_num_app:
""")
        f.write("    .quad {}\n".format(len(apps)))
        for i in range(len(apps)):
            f.write("    .quad app_{}_start\n".format(i))
        f.write("    .quad app_{}_end\n".format(len(apps)-1))
        for i, app in enumerate(apps):
            f.write("""
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/{1}.bin"
app_{0}_end:
    """.format(i,app))
    print('{} Generating link_app assembly file succeeded!'.format(prefix))

def discover_app():
    """
    discover_app automatically discover user applications in directory: user/src/bin

    Returns:
        list: user application list 
    """
    apps = os.listdir('user/src/bin')
    ret = []
    for app in sorted(apps):
        ret.append(app[:app.find('.')])
    return ret

def remove_target_directory():
    """
    remove_target_directory remove os target directory and user target directory
    """    
    if os.system('rm -rf ./os/target') != 0:
        print('{} Error on remove os target directory'.format(prefix))
        exit_function()
    if os.system("rm -rf ./user/target") != 0:
        print('{} Error on remove user target directory'.format(prefix))
        exit_function()

def k210():
    with open ("./os/src/linker.ld" , "w") as f:
        f.writelines("""
OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80020000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    etext = .;
    
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;

    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;

    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    ebss = .;
    
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
""")
    remove_target_directory()
    apps = discover_app()
    generate_link_app(apps)
    build_apps(apps)
    build_os()
    os.system("cp {0} {0}.copy".format(BOOTLOADER))
    os.system("dd if={} of={}.copy bs={} seek=1".format(KERNEL_BIN,BOOTLOADER,K210_BOOTLOADER_SIZE))
    os.system("mv {}.copy {}".format(BOOTLOADER, KERNEL_BIN))
    os.system("kflash -p {} -b 1500000 {}".format(K210SERIALPORT,KERNEL_BIN))
    with open ("./os/src/linker.ld" , "w") as f:
        f.writelines("""
OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    etext = .;
    
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;

    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;

    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    ebss = .;
    
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
""")
    os.system("python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct {} 115200".format(K210SERIALPORT))

def qemu():
    remove_target_directory()
    apps = discover_app()
    generate_link_app(apps)
    build_apps(apps)
    build_os()
    if DEBUG_OS:
        debug_os()
    else:
        run_os()

def __main__():

    k210()


__main__()