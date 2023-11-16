#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//メソッド定義
impl Rectangle {
    // 長方形の面積を求める
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 長方形が他の長方形を包含しているかどうかを判定する
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // 関連関数
    // 長方形を生成する
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width = 30;
    // let height = 50;
    // let rect = (30, 50);
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // 正方形
    let square = Rectangle::square(10);

    // println!("長方形の面積は、{}平方ピクセルです", area(width, height));
    // println!("長方形の面積は、{}平方ピクセルです", area(rect));
    // println!("長方形の面積は、{}平方ピクセルです", area(&rect));
    println!("長方形の面積は、{}平方ピクセルです", rect.area());
    println!("正方形の面積は、{}平方ピクセルです", square.area());

    println!("rect に rect2 は入る? {}", rect.can_hold(&rect2));
    println!("rect に rect3 は入る? {}", rect.can_hold(&rect3));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
