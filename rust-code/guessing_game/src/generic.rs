#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn main() {
    let point1 = Point { x: 1, y: true };

    let point2 = Point { x: 'a', y: 1.1 };

    let result = point1.mixup(point2);

    println!("{:#?}", result);
}
