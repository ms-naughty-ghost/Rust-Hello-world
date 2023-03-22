use rand::Rng;
// ランダムな整数を生成する
fn random() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)
}

fn main() {
    // if文
    let age = random();
    if age >= 18 {
        println!("{} is adult", age);
    } else {
        println!("{} is child", age);
    }
    let r = if age >= 18 { "adult" } else { "child" }; // ifは式なので変数に代入できる
    println!("{} is {}", age, r);
    // 足し算
    let x = 10 + 20;
    println!("x:{}", x);
    // 引き算
    let y = 10 - 20;
    println!("y:{}", y);
    // 掛け算
    let z = 10 * 20;
    println!("z:{}", z);
    // 割り算
    let w = 10 / 3; // 整数同士の割り算は整数になる
    println!("w:{}", w);
    let w2 = 10.0 / 3.0; // 浮動小数点数同士の割り算は浮動小数点数になる
    println!("w2:{}", w2);
    // 剰余
    let r = 10 % 20;
    println!("r:{}", r);
    // 論理演算子
    let t = true;
    let f = false;
    println!("t && f:{}", t && f);
    println!("t || f:{}", t || f);
    println!("!t:{}", !t);
    // 比較演算子
    let a = 10;
    let b = 20;
    println!("a == b:{}", a == b);
    println!("a != b:{}", a != b);
    println!("a > b:{}", a > b);
    println!("a < b:{}", a < b);
    println!("a >= b:{}", a >= b);
    println!("a <= b:{}", a <= b);
    // ビット演算子
    let a = 0b1010;
    let b = 0b1100;
    println!("a & b:{:04b}", a & b);
    println!("a | b:{:04b}", a | b);
    // シフト演算子
    let a = 0b1010;
    println!("a << 1:{:04b}", a << 1);
    println!("a >> 1:{:04b}", a >> 1);
    // for文
    for i in 0..10 {
        print!("i:{} ", i);
    }
    print!("\n");
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        print!("arr:{} ", i);
    }
    print!("\n");
    for i in arr.iter() {
        print!("arr:{} ", i);
    }
    print!("\n");
    // インデックス付きfor文
    for (i, v) in arr.iter().enumerate() {
        print!("arr[{}]:{} ", i, v);
    }
    print!("\n");
    // while文
    let mut i = 0;
    while i < 10 {
        print!("i:{} ", i);
        i += 1;
    }
    print!("\n");
    // loop文
    let mut i = 0;
    loop {
        print!("i:{} ", i);
        i += 1;
        if i >= 10 {
            break;
        }
    }
    print!("\n");
    // match文
    #[derive(Debug)] // デバッグ用に文字列を表示するために必要
    enum Hololive {
        USADA_PEKORA,
        SAKURA_MIKO,
        SHIRAKAMI_FUBUKI,
        NAKIRI_AYAME,
        MURASAKI_SHION,
        MINATO_AQUA,
        HOSHIMACHI_SUISEI,
        YAGOO,
    }
    let talent = Hololive::USADA_PEKORA;
    let name = match talent {
        Hololive::USADA_PEKORA => "Usada Pekora",
        Hololive::SAKURA_MIKO => "Sakura Miko",
        Hololive::SHIRAKAMI_FUBUKI => "Shirakami Fubuki",
        Hololive::NAKIRI_AYAME => "Nakiri Ayame",
        Hololive::MURASAKI_SHION => "Murasaki Shion",
        Hololive::MINATO_AQUA => "Minato Aqua",
        Hololive::HOSHIMACHI_SUISEI => "Hoshimachi Suisei",
        Hololive::YAGOO => "Yagoo",
    };
    println!("talent: {}", name);
}
