# myOS

使用Rust编写的简单操作系统

# 构建环境

## Rust 开发环境配置

首先安装 Rust 版本管理器 `rustup` 和 Rust 包管理器 `cargo：`

```bash
curl https://sh.rustup.rs -sSf | sh
```

安装完成后，重新打开一个终端来让之前设置的环境变量生效。

接下来，确认一下正确安装了`Rust`工具链：

```bash
rustc --version
```

通过如下命令安装 `rustc` 的 `nightly` 版本，并把该版本设置为 `rustc` 的缺省版本。

```bash
rustup install nightly
rustup default nightly
```

换源，如果有必要的话。打开（如果没有就新建）` ~/.cargo/config` 文件，并把内容修改为：

```bash
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

安装Rust相关的软件包：

```bash
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils --vers =0.3.3
rustup component add llvm-tools-preview
rustup component add rust-src
```
## Qemu 模拟器安装

使用 Qemu 5.0.0 版本安装运行。

### 安装编译所需的依赖包

安装qemu及其依赖，之后仅卸载qemu保留依赖包，因为需要安装的是qemu5.0.0。

```bash
sudo pacman -S qemu
sudo pacman -R qemu
``` 

### 下载源码包

如果下载速度过慢可以使用百度网盘链接：[https://pan.baidu.com/s/1z-iWIPjxjxbdFS2Qf-NKxQ](https://pan.baidu.com/s/1z-iWIPjxjxbdFS2Qf-NKxQ)
提取码: `8woe`

```bash
wget https://download.qemu.org/qemu-5.0.0.tar.xz
```

### 解压

```bash
tar xvJf qemu-5.0.0.tar.xz
```

### 编译安装并配置 RISC-V 支持

```bash
cd qemu-5.0.0
./configure --target-list=riscv64-softmmu,riscv64-linux-user
make -j$(nproc)
sudo make install
```

确认 Qemu 的版本：

```bash
qemu-system-riscv64 --version
qemu-riscv64 --version
```

## K210 真机串口通信

为了能在 K210 真机上运行 Tutorial，我们还需要安装基于 Python 的串口通信库和简易的串口终端。

```bash
pip3 install pyserial
sudo apt install python3-serial
```

## GDB 调试支持

[Ubuntu平台下载](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14.tar.gz)


# 运行
```bash
python __main__.py
```

# 开发日志

## 2022年4月12日

BUG原因：误删user/src/syscall.rs文件
