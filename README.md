# 零、当前位置

根据[2022 年开源操作系统培训计划][https://github.com/LearningOS/rust-based-os-comp2022]。 

在2022年11月到12月，完成了[rustling作业][https://github.com/os2edu/rustlings-anicetrip]。开始进入os的初步学习。

在安装环境的地方，选择使用[长版教程][http://rcore-os.cn/rCore-Tutorial-Book-v3/]。

# 一、在ubuntu22.04上安装环境

在官方教程中，使用了包括wsl2，虚拟机，裸机安装等多种形式部署环境，基于自己电脑硬盘已满，黑五忘记购买新的硬盘的情况下，决定使用之前买的4核服务器。因为官方网站没有提供docker的安装源，所以决定直接部署。

## 1.1 c开发环境部署

```shell
# 常规升级
sudo apt-get update && sudo apt-get upgrade
# 因为不知道最后用了哪些包，所以按最多的一个教程写了，如果不需要的话之后autoremove掉就好了。
sudo apt-get install git-email libaio-dev libbluetooth-dev libcapstone-dev libbrlapi-dev libbz2-dev libcap-ng-dev libcurl4-gnutls-dev libgtk-3-dev libibverbs-dev libjpeg8-dev libncurses5-dev libnuma-dev librbd-dev librdmacm-dev  libsasl2-dev libsdl2-dev libseccomp-dev libsnappy-dev libssh-dev libvde-dev libvdeplug-dev libvte-2.91-dev libxen-dev liblzo2-dev valgrind xfslibs-dev libnfs-dev libiscsi-dev git build-essential gdb-multiarch gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu libglib2.0-dev pkg-config  libpixman-1-dev -y
```

## 1.2 qemu安装

由于在官方教程中，要求使用qemu7.0以上的版本，所以必须手动编译安装。

```shell
# 基于官网
wget https://download.qemu.org/qemu-7.2.0-rc4.tar.xz
tar xvJf qemu-7.2.0-rc4.tar.xz
cd qemu-7.2.0-rc4
#这里和官网不一样是按教程的
./configure --target-list=riscv64-softmmu,riscv64-linux-user 
make -j$(nproc)

```

如果在`./configure`步骤中出现报错`ERROR: Cannot find Ninja`， 那么需要安装ninja-build`sudo apt install ninja-build -y`。[来源][https://nextgentips.com/2022/04/20/how-to-install-and-configure-qemu-7-on-ubuntu-20-04/]

接下来重新执行官网步骤

```shell
./configure --target-list=riscv64-softmmu,riscv64-linux-user 
make -j$(nproc)
```

第一次直接按官网的操作了，然后会有接近1万个文件需要编译，是个超级超级大的工程，然后如果按教程给的操作只要编译两千个就好了，节约了很多。

然后配置path，我参考的如下

> Edit `.bashrc` in your home directory and add the following line:
>
> ```
> export PATH=/path/to/dir:$PATH
> ```
>
> You will need to source your `.bashrc` or logout/login (or restart the terminal) for the changes to take effect. To source your `.bashrc`, simply type
>
> ```
> source ~/.bashrc
> ```

所所以首先打开`.bashrc`然后在最后一行加入

```shel
export PATH=$PATH:/root/qemu/qemu-7.2.0-rc4/build
```

接下来`source ~/.bashrc`即可。

最后检查一下是否成功

```shell
qemu-system-riscv64 --version
qemu-riscv64 --version
```

## 1.3 安装rust

```
curl https://sh.rustup.rs -sSf | sh
```

注意下载完成后会自动进入脚本，然后选定制化，然后第一个默认，第二个选nightly，第三个随意，然后进入正常安装流程。安装完成后检查一下版本`rustc --version`。

如果发现装不了的话不止rust需要配置代理，记得也把git配置代理，之前国内学的时候血泪的教训。

接下来安装rust必要包

```shell
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup component add rust-src
```

## 1.4安装riscv-gdb调试器

首先上[下载链接][https://github.com/sifive/freedom-tools/releases/tag/v2020.12.0]，之所以要下载这个是因为ubuntu22不再可以直接安装，我看了一下源码安装源码就6个g，从我服务器硬盘角度来说根本无法接受，所以找了一下，运气很好的找到了已经编译好直接可用的版本。

直接`tar -zxvf <下载的文件名>`然后在bin文件中，直接打开之后配置shell即可。



## 1.5 检查是否安装正确（试运行）

```
git clone  ``Github-Classroom帮你生成的某个OS实验的仓库``
$ cd  ``刚克隆的本地某个OS实验的仓库``
$ make setupclassroom_``实验编号``  //注意：这一步很重要，是用于github classroom自动评测你的工作。这一步只需在首次克隆项目仓库时执行一次，以后一般就不用执行了，除非 .github/workflows/classroom.yml发生了变化。实验编号是与某个实验匹配的编号
```

**编号表**

实验名称 ：实验编号

- lab0-0 : test1
- lab0-1：test2
- lab1：test3
- lab2：test4
- lab3：test5
- lab4：test6
- lab5：test8

这个编号表我第一次看根本没找到orz

接下来就是测试还有运行了，具体看所谓的[简版教程][https://learningos.github.io/rust-based-os-comp2022/chapter1/0intro.html]。

## 第零章

### API与ABI

>ABI: 二进制接口。定义二进制机器码级别规范，与处理器核核内存地址等硬件架构相关。
>
>- 规则包括基本数据类型、通用寄存器的使用、参数的传递规则、以及堆栈的使用等
>- 用来约束链接器 (Linker) 和汇编器 (Assembler) 
>
>API:代码片段接口。定义源码级规范，用来约束编译器 。
>
>- 定义了一个源码级（如 C 语言）函数的参数，参数的类型，函数的返回值等
>- 一个 API 是给编译器的一些指令，它规定了源代码可以做以及不可以做哪些事

### 操作系统的特征

#### 虚拟性

* 内存虚拟化： **内存地址虚拟化** 和 **内存大小虚拟化**
* CPU 虚拟化

#### 并发性

> - 并行 (Parallel) 是指同一个任务划分多个子任务利用多核在同一时刻运行；
>
> - 并发 (Concurrent) 是指两个或多个事件通过时间分片等在单核上模拟并行运行。
>
>   **教程里定义不准确！！**

#### 异步性

指的是操作系统的调用和中断

#### 共享性

多个应用并发运行时，宏观上体现他们可同时访问同一个资源。

#### 持久性

保证文件系统可以持久保存到可存储介质。



# 第一章

## 原理理解

内存地址对齐：由于计算机通过数据总线和地址总线链接内存和cpu，因此RISC-V处理器要求数据访存时，数据应对齐。

## Qemu模拟器运行原理

### 运行指令demo

```shell
qemu-system-riscv64 \
2    -machine virt \
3    -nographic \
4    -bios ../bootloader/rustsbi-qemu.bin \
5    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```

* `-machine virt` 模拟满足virt平台的计算机硬件
* `-nographic` 无图像输出
* `-bios ../bootloader/rustsbi-qemu.bin`设置开机时用来初始化引导加载程序。注意这是已经编译好的代码，直接从`bootloader/rustsbi-qemu.bin`中获取。
* `-device loader, file =<file path>, addr = <address>`其中loader属性可以在qemu模拟器开机之前将一个宿主机上的文件载于到特定qemu中的物理地址。

### 启动流程

`virt` 平台上，物理内存的起始物理地址为 `0x80000000` 。**在启动时**，bootloader的`rustbi-qemu.bin`被加载到物理内存以`0x80000000`开始的位置中，同时按`loader`将`os.bin`加载到`0x80200000`的物理地址中。

Qemu 模拟的启动流程则可以分为三个阶段：第一个阶段由固化在 Qemu 内的一小段汇编程序负责；第二个阶段由 bootloader 负责；第三个阶段则由内核镜像负责。

* 第一阶段：Qemu CPU 的程序计数器（PC）会被初始化为 `0x1000` ，因此 Qemu 实际执行的第一条指令位于物理地址 `0x1000` ，执行数条指令并跳转到物理地址 `0x80000000` 。
* 第二阶段：在 `0x80000000`的bootloader负责初始化计算机，并跳转到下一阶段（内核）的程序入口。选用的RustSBI将其固定为`0x80200000`。
* 第三阶段：执行内核的代码。

## 内存布局模式

|             |         |              |
| ----------- | ------- | ------------ |
|             |         | High Address |
| --↓---      | stack   |              |
|             | --↓---  |              |
|             | --↑---  |              |
| Data Memory | heap    |              |
|             | .bss    |              |
|             | .data   |              |
| --↑---      | .rodata |              |
| Code Memory | .text   | Low Address  |
|             |         |              |

* `.rodata`只读全局数据（常量）
* `.data`全局变量
* `.bss`未初始的全局数据
* 栈： 在 RISC-V 架构中，栈是从高地址向低地址增长的
  * 栈帧（stack frame）标识一个函数所使用的栈的一部分区域
  * 一般而言，当前执行函数的栈帧的两个边界分别由栈指针 (Stack Pointer)寄存器和栈帧指针（frame pointer）寄存器来限定。

## 编译流程

编译器→汇编器→连接器

* 编译器： 高级语言转汇编
* 汇编器：汇编转二进制
  * 按内存布局，拼装所有相关的汇编文件。
  * 将符号替换为具体地址
    * 将（内部）函数，变量等替换为对应的函数，变量地址。
* 连接器：二进制文件与必要的外部依赖文件链接到一起获得可执行文件。
  * 将（外部）函数，变量替换为对应地址。

## 代码分析

### 配置

#### cargo 配置

```toml
# os/.cargo/config
# 修改编译运行环境为riscv64，这里的elf 表示没有标准的运行时库（表明没有任何系统调用的封装支持）
[build]
target = "riscv64gc-unknown-none-elf"

# -T表示强制使用特定的编译器配置脚本，而不是使用默认配置。
# -C不懂，force-frame-pointers会多使用一个寄存器用于保存栈指针，提供给调试工具查找变量等内容，缺点在于会影响性能。
 [target.riscv64gc-unknown-none-elf]
 rustflags = [
     "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes"
 ] 

```

#### 汇编预处理配置

 ```asm
# os/src/entry.asm
    .section .text.entry # 将第二行到第六行的部分放在 .text.entry的部分
    .globl _start # 设置 _start为全局符号
_start: # 冒号将指针指向紧跟着的内容
    la sp, boot_stack_top #栈指针 sp 设置为先前分配的启动栈栈顶地址，这样 Rust 代码在进行函数调用和返回的时候就可以正常在启动栈上分配和回收栈帧了
    call rust_main #通过伪指令 call 调用 Rust 编写的内核入口点 rust_main 将控制权转交给 Rust 代码

    .section .bss.stack # 将第八行到13行的部分放在 .bss.stack块中
    .globl boot_stack_lower_bound # 设置全局变量
boot_stack_lower_bound: #冒号将指针指向紧跟着的内容
    .space 4096 * 16 #从low的地方往上预留了一块大小为 4096 * 16 字节也就是  的空间用作接下来要运行的程序的栈空间
    .globl boot_stack_top # 设置boot_stack_top为全局符号
boot_stack_top: #冒号将指针指向紧跟着的内容
 ```

#### 连接器配置

```livescript
OUTPUT_ARCH(riscv) # 设置目标平台
ENTRY(_start) # 设置启动点
BASE_ADDRESS = 0x80200000; # 定义常量为初始化代码需要被放置的地方。
# "SECTIONS "命令告诉链接器如何将输入部分映射到输出部分，以及如何将输出部分放在内存中。
SECTIONS
{
    . = BASE_ADDRESS; # 将内存指针指向初始化位置，链接器链接是按照SECTIONS里的段顺序排列的，前面的排列完之后就能计算出当前地址
    skernel = .; # 开始内核代码
	# 代码段
    stext = .; # 开始.test部分
    .text : {
        *(.text.entry) # 让.text.entry在其他的.text之前
        *(.text .text.*)
    }

    . = ALIGN(4K); #4k对齐
    etext = .; #结束代码段
    srodata = .; #设置只读数据起点
    .rodata : { #只读存储位置
        *(.rodata .rodata.*) # 存入只读内容
        *(.srodata .srodata.*) #微型只读数据，因为可以通过特定的方式更快获取数据所以额外增加模型。
    }

    . = ALIGN(4K);
    erodata = .; # 设置只读数据终点
    # data段用于将flash中的数据和函数复制到内存并初始化。
    sdata = .;
    .data : { 
        *(.data .data.*)
        *(.sdata .sdata.*) # 微型data
    }

    . = ALIGN(4K);
    edata = .;
    # .bss段用于未初始的数据，这个段中的内容在启动时会被清零
    .bss : {
        *(.bss.stack) # 这里是之前汇编中设置的栈的位置
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;
   # 不存入
    /DISCARD/ : {
        *(.eh_frame) # 用于异常处理相关的堆栈表
    }
}
```

其中`srodata`的解释[来源][https://www.reddit.com/r/gcc/comments/pgjhey/gcc_sections/]

#### rust-sbi接口配置

```rust
// os/src/sbi.rs
// 服务类型常量
#![allow(unused)] // 此行请放在该文件最开头
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

// 调用sbi接口， which中为sbi_call的接口名，接下来可以接受最多3个参数
use core::arch::asm;
#[inline(always)] // 建议编译器将指定的函数体插入并取代每一处调用该函数的地方
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret, //inlateout(<reg>) <in expr> => <out expr> 先存入，然后再获取返回值（非原子情况返回的内容可能会提前洗掉输入）
            in("x11") arg1, //in(<reg>) <expr>， reg: 寄存器类或者实际寄存器。 expr： 传入的变量
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}


pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}


pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}
```

对于x^n^ :(只看说明有的部分,其他忽略即可)

| Register | ABI Name | Description                      | Description-CN      | Saver  | 说明                                                         |
| -------- | -------- | -------------------------------- | ------------------- | ------ | ------------------------------------------------------------ |
| x0       | zero     | Hard-wired zero                  | 设为不可变0         | --     | 恒为零，函数调用不会对它产生影响                             |
| x1       | ra       | Return address                   | 返回地址            | Caller | 是调用者保存的，但是与其他调用者保存的寄存器不同的是它会被保存在被调用者函数的栈帧中。前面提到过它的作用是让被调用者函数能够正确跳转回调用者函数继续向下执行。 |
| x2       | sp       | Stack pointer                    | 栈指针              | Callee |                                                              |
| x3       | gp       | Global pointer                   | 全局指针            | --     |                                                              |
| x4       | tp       | Thread pointer                   | 线程指针            | -–     |                                                              |
| x5-7     | t0–2     | Temporaries                      | 临时变量            | Caller | 作为临时寄存器使用，在被调函数中可以随意使用无需保存。       |
| x8       | s0/fp    | Saved register/frame pointer     |                     | Callee | 作为临时寄存器使用，被调函数保存后才能在被调函数中使用。<br />它既可作为s0临时寄存器，也可作为栈帧指针（Frame Pointer）寄存器，表示当前栈帧的起始位置，是一个被调用者保存寄存器。 （标识一个函数所使用的栈的一部分区域） |
| x9       | s1       | Saved register                   |                     | Callee | 作为临时寄存器使用，被调函数保存后才能在被调函数中使用。     |
| x10-11   | a0–1     | Function arguments/return values | 函数参数/函数返回值 | Caller | 用来传递输入参数, 同时a0, a1用来保存返回值。                 |
| x12-17   | a2–7     | Function arguments               | 函数参数            | Caller | 用来传递输入参数                                             |
| x18-27   | s2–11    | Saved registers                  |                     | Callee | 作为临时寄存器使用，被调函数保存后才能在被调函数中使用。     |
| x28-31   | t3–6     | Temporaries                      |                     | Caller | 作为临时寄存器使用，在被调函数中可以随意使用无需保存。       |
| f0-7     | ft0–7    | FP temporaries                   |                     | Caller |                                                              |
| f8-9     | fs0–1    | FP saved registers               |                     | Callee |                                                              |
| f10-11   | fa0–1    | FP arguments/return values       |                     | Caller |                                                              |
| f12-17   | fa2–7    | FP arguments                     |                     | Caller |                                                              |
| f18-27   | fs2–11   | FP saved registers               |                     | Callee |                                                              |
| f28-31   | ft8–11   | FP temporaries                   |                     | Caller |                                                              |

一些从[doc][https://riscv.org/wp-content/uploads/2015/01/riscv-calling.pdf]拿到的暂时不相关内容：

> In addition to the argument and return value registers, seven integer registers t0–t6 and twelve floating-point registers ft0–ft11 are temporary registers that are volatile across calls and must be saved by the caller if later used. Twelve integer registers s0–s11 and twelve floating-point registers fs0–fs11 are preserved across calls and must be saved by the callee if used. 
>
> the stack grows downward and the stack pointer is always kept 16-byte aligned.

### 代码块

#### 终端打印文件

宏魔法跳过

```rust
// os/src/console.rs
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
```

#### 错误处理

由于rust core中没有包含panic的错误处理程序，所以需要手写一个处理，可读性挺好的。

```rust
// os/src/lang_items.rs
use crate::sbi::shutdown;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
```

### main模块

#### 预处理模块

```rust
// os/src/main.rs
#![no_std] // 不使用 Rust 标准库 std 转而使用核心库 core（core库不需要操作系统的支持）
#![no_main] //停用对于main的预处理过程
#![feature(panic_info_message)]
#[macro_use] //一定在最前面否则会报错找不到
mod console;//一定在最前面否则会报错找不到
mod sbi;
mod lang_items;

use core::arch::global_asm;
//调用汇编
use sbi::console_putchar;
global_asm!(include_str!("entry.asm"));
```

#### 功能函数

```rust
fn clear_bss() {
    extern "C" {//extern “C” 可以引用一个外部的 C 函数接口
        //sbss 和 ebss ，它们由链接脚本 linker.ld 给出，并分别指出需要被清零的 .bss 段的起始和终止地址。
        fn sbss();
        fn ebss();
    }
    //设为u8因为u8为一个位
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

```



#### 核心函数

打印单个ASSIC字母

```rust
#[no_mangle] //告知编译器不混淆、替换函数名
pub fn rust_main() -> ! {
    clear_bss();
    console_putchar(44); 
    panic!("Shutdown machine!");
}
```

打印整句

```rust
#[no_mangle]
pub fn rust_main() -> ! {
     clear_bss();
     println!("Hello, world!");
     panic!("Shutdown machine!");
}
```

