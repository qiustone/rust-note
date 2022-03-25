/**
 * > rustc main.rs
 * > .\main.exe
 * Hello, world!
 * 
 * println! is Rust macro.
 * 
 * 该程序编译完成后有三个文件，分别是
 * main.rs main.pdb man.exe
 * 其中 main.pdb 包含了调试信息
 */

fn main() {
    println!("Hello, world!");
}