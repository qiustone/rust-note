use std::io;
use rand::Rng;

// cargo doc --open 将生成本项目相关方法的本地文档并在浏览器中打开

fn main() {
    let foo = 5;       // immutable  let 被用于创建变量
    let mut bar = foo; // mutable    Rust中变量默认是不可变的, 如果想使变量值可变，需要使用mut关键字修饰

    println!("The bar:{}", bar);
    
    bar = 6;
    
    println!("The bar:{}", bar);

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is : {}", secret_number);

    println!("Guess the number!");

    println!("Please input your guess.");

    //:: 表明 new() 是String type的关联函数(associated function),有的语言称之为 static method
    let mut guess = String::new();  

    // & 表明参数是一个 reference(引用), reference在默认情况下是不可变的, 所以这里不能写 &guess
    io::stdin().read_line(&mut guess).expect("Failed to read line"); 
    
    println!("You guessed: {}", guess);
}
