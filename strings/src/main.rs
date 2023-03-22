// 文字列
fn main() {
    let a = 'A'; // char型はシングルクォーテーションで囲む.
    println!("{}", a);
    let u = a as u8; // u8型に変換.ASCIIコードに変換.
    println!("{}", u);
    let a = u as char; // char型に変換.
    println!("{}", a);
    let s = "Hello, world!"; // 文字列はダブルクォーテーションで囲む.
    println!("s is {}", s);
    let hello = &s[0..5]; // 0から5文字目までの文字列を取り出す.
    let world = &s[7..12]; // 7から12文字目までの文字列を取り出す.
    println!("{} {}", hello, world);
    // 文字列の長さを取得.
    println!(
        "s.len() is {}\ns.chars().count() is {}",
        s.len(),
        s.chars().count()
    );
    // 空の文字列を作成.
    let mut s1 = String::new();
    // 文字列を結合.
    s1.push_str(hello);
    s1.push_str(" rust ");
    s1.push_str(world);
    println!("s1 is {}", s1);
    // format!マクロで文字列を結合.
    let s2 = format!("{} rust {}", hello, world);
    println!("s2 is {}", s2);
    // 日本語分割がうまくいかない例.
    let s3 = "こんにちは、世界！";
    println!("s3 is {}", s3);
    // let s4 = &s3[0..5]; // 0から5文字目までの文字列を取り出す.実行時エラーになる.
    // println!("{}", s4);
    // s3の文字数を取得.
    println!(
        "s3.len() is {}\ns3.chars().count() is {}",
        s3.len(),           // 文字列のバイト数が取得される.
        s3.chars().count()  // 文字列の文字数が取得される.
    );
    // 日本語を正しく扱う方法.
    // 文字列からchar型のベクター型を作成.
    let s = "This is ネコ🐱neko in Japanese.";
    let mut v: Vec<char> = s.chars().collect();
    for c in s.chars() {
        v.push(c);
    }
    let v = &v[8..15]; // 8から15文字目までの文字列を取り出す.
    let s = v.iter().collect::<String>(); // char型のベクター型を文字列に変換.
    println!("{}", s);
    // 文字列の先頭をきりだす.
    let s = "This is ネコ🐱neko in Japanese.";
    let s = &s[..1]; // 0は省略可能.
    println!("first char is {}", s);
    // 文字列の末尾をきりだす.
    let s = "This is ネコ🐱neko in Japanese.";
    let s = &s[8..]; // 8から最後まで.
    println!("last char is {}", s);
}
