// 関数の宣言は型の指定が必須
fn add(x: i32, y: i32) -> i32 {
    println!("add:");
    x + y // returnの省略
}

// 文字列の長さを返す関数
fn len(s: &str) -> usize {
    s.len()
}
// 型
fn main() {
    // 型推論
    let name = "john doe";
    println!("{} ", name);
    let age = 33;
    // 型を明示的に指定
    let name2: &str = "john doe";
    let age2: i32 = 30;
    println!("{},{}", name2, add(age, age2));
    // 浮動小数点数 指定しない場合はf64
    let f = 10.123;
    println!("f:{}", f);
    // 理論値
    let t = true;
    println!("t:{}", t);
    // String型を使った文字列
    let s = String::from("hello");
    println!("s:{}", s);
    // format!マクロを使った文字列
    let s2 = format!("{} {}", s, name);
    println!("s2:{}", s2);
    // タプル
    let tup = (1, 2, 3, "hello", "world");
    println!("tup:{} {}", tup.3, tup.4);
    // 配列
    let arr = [1, 2, 3, 4, 5];
    println!("arr:{}", arr[0]);
    // 参照と借用
    let s3 = String::from("hello");
    let s4 = s3;
    // println!("s3:{}", s3); // s4はs3の所有権を奪ったのでs3は使えない
    println!("s4:{}", s4);
    println!("len:{}", len(&s4)); // 参照渡し
    println!("s4:{}", s4);
    // 束縛
    let i = 10;
    println!("i:{}", i);
    // i = 20; // エラー
    let mut i2 = 20; // mutをつけると再代入可能
    println!("i2:{}", i2);
    i2 = 30; // 再代入可能
    println!("i2:{}", i2);
    // シャドーイング
    let i3 = 10;
    println!("i3:{}", i3);
    let i3 = 20; // 同じ名前の変数を定義できる
    println!("i3:{}", i3);
    let i3 = "hello"; // 型も変更できる
    println!("i3:{}", i3);
    // 構造体
    let s = Sample::new(10, 20);
    println!("s.x:{}", s.x);
    println!("s.y:{}", s.y);
    // 構造体のメソッド
    println!("s.add:{}", s.add());
    // クロージャ
    let c = |x: i32, y: i32| -> i32 { x + y };
    println!("c:{}", c(10, 20));
    // Rustの変数スコープは、ブロック、関数、構造体、クロージャ単位がある
}

struct Sample {
    x: i32,
    y: i32,
}

impl Sample {
    fn new(x: i32, y: i32) -> Sample {
        Sample { x, y }
    }
    fn add(&self) -> i32 {
        self.x + self.y
    }
}
