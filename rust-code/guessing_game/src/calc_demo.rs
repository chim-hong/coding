#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    pub fn square(width: u32) -> Rectangle {
        Rectangle {
            width,
            height: width,
        }
    }
}

fn calc_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

pub fn main() {
    let rect_info = Rectangle {
        width: 12,
        height: 12,
    };
    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };
    let rect2 = Rectangle {
        width: 13,
        height: 10,
    };
    let area = calc_area(&rect_info);

    let area2 = rect_info.area();

    println!("area:{},area2:{}", area, area2);

    println!("==========");
    println!("canb hold:{}", rect_info.can_hold(&rect1));
    println!("canb hold:{}", rect_info.can_hold(&rect2));
    println!("==========");

    let square = Rectangle::square(10);

    println!("{:#?}", square);

    println!("square.area:{}", square.area());
}
