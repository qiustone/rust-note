# Insall rust for Windows
1. 打开网址 https://www.rust-lang.org/tools/install 并根据提示下载安装程序

2. 运行安装程序 rustup-init.exe 并根据提示安装

3. Rust 由 rustup 工具安装和管理, 默认安装路径为 C:\Users\Administrator\\.cargo\bin 和 C:\Users\Administrator\\.rustup

4. 配置环境变量, 在安装时可选自动加入环境变量PATH, 路径为 C:\Users\Administrator\.cargo\bin

5. 打开CMD, 输入 `rustc --version` 检查是否已安装好 rust  
   
# Updating and Uninstalling
* `rustup update`
* `rustup self uninstall`
