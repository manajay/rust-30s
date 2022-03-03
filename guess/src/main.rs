use rand::{random, Rng};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100); // 语法经常会变
    println!("神秘数字: {}", secret_number);
    println!("Hello, world!");
}
