/**
 * $ cargo --version        //查看cargo版本
 * 
 * $ cargo new hello_cargo  //使用cargo创建名称为hello_cargo的项目
 * 
 * 我们可以发现cargo 帮助我们创建了2个文件和一个目录：
 * Cargo.toml文件、src目录和src目录下的main.rs文件, Cargo希望你的源文件放在src目录下
 * 项目顶层目录仅用于放置 README、license、configuration(配置)文件，已经其他与代码无关的文件。
 * 如果你的项目一开始未使用Cargo, 后期也可以转为Cargo项目, 仅需将代码放入src目录，
 * 并在外层创建一个合适的 Cargo.toml 文件即可。
 * 
 * $ cargo build            //编译项目，第一次运行会生成一个 Cargo.lock文件, 并生成编译结果文件夹target
 * 
 * Cargo.lock 此文件用于跟踪项目中依赖项的确切版本, 你永远都不需要手动修改这个文件，Cargo会帮你管理它。
 * 
 * $ cargo build --release  //编译 release 版本
 * 
 * $ cargo run              //编译并运行
 * 
 * $ cargo check            //这个命令可以快速检查你的代码，确保它可以编译，但不会生成可执行文件:
 * 
 */
fn main() {
    println!("Hello, world!");
}
