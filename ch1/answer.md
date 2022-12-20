# 课后练习
## 编程题
1.  实现一个linux应用程序A，显示当前目录下的文件名。（用C或Rust编程）
    ```rust
    use std::fs;

    fn main() {
        let paths = fs::read_dir("./").unwrap();
    
        // paths.for_each(|a| {
        //     println!("{:#?}", a.unwrap().path());
        // });
            println!("{}\n{}","./","../");
        for path in paths {
            println!("{}", path.unwrap().path().display())
        }
    }
    ```

2. 实现一个linux应用程序B，能打印出调用栈链信息。（用C或Rust编程）
    ```rust
    use std::backtrace::Backtrace;

    fn main() {
        println!("Custom backtrace: {}", Backtrace::capture());

        // ... or forcibly capture the backtrace regardless of environment variable configuration
        println!("Custom backtrace: {}", Backtrace::force_capture());
    }
    ```
3. 实现一个基于rcore/ucore tutorial的应用程序C，用sleep系统调用睡眠5秒（in rcore/ucore tutorial v3: Branch ch1）
