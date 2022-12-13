# rust-os-study
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
./configure --target-list=riscv64-softmmu,riscv64-linux-user 
make -j$(nproc)

```

如果在`./configure`步骤中出现报错`ERROR: Cannot find Ninja`， 那么需要安装ninja-build`sudo apt install ninja-build -y`。[来源][https://nextgentips.com/2022/04/20/how-to-install-and-configure-qemu-7-on-ubuntu-20-04/]

接下来重新执行官网步骤

```shell
./configure --target-list=riscv64-softmmu,riscv64-linux-user 
make -j$(nproc)
```

会有接近1万个文件需要编译，是个超级超级大的工程。

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

## 1.4 检查是否安装正确（试运行）

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

