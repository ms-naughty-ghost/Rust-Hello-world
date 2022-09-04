// 関数の宣言は型の指定が必須
fn add(x: i32, y: i32) -> i32 {
    println!("add:");
    x + y
}

fn main() {
    // 型推論
    let name = "john doe";
    let age = 33;
    // 型を明示的に指定
    let name2 : &str = "john doe";
    let age2: i32 = 30;
    println!("{}",add(age, age2))
    // 浮動小数点数 指定しない場合はf64
    let f = 10.123;
}
