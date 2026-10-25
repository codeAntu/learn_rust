#[derive(Debug)]
// struct Book {
//     title: String,
//     pages: u32,
//     available: bool,
// }

// pub fn run() {
//     let my_book = Book {
//         title: String::from("The Rust Programming Language"),
//         pages: 500,
//         available: true,
//     };

//     let my_book_2 = Book {
//         title: String::from("Programming in Rust"),
//         ..my_book
//     };

//     println!("The value of my_book is: {:#?}", my_book);
//     println!("The value of my_book_2 is: {:#?}", my_book_2);
// }

// struct Points(f64, f64, f64 );

// impl Points {
//     fn distance_from_origin(&self) -> f64 {
//         (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
//     }
// }

// pub fn run ( ) {
//     let point = Points(3.0, 4.0, 5.0);

//     let distance = point.distance_from_origin();

//     println!("The distance from the origin is: {}", distance);

// }

struct Rect {
    hight: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
       dbg!(self.hight * self.width)
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.hight > other.hight && self.width > other.width
    }

    fn square(size: u32) -> Rect {
        Rect {
            hight: size,
            width: size,
        }
    }

    fn scale(&mut self, factor: u32) {
        self.hight *= factor;
        self.width *= factor;
    }
}

pub fn run() {
    let rect1 = Rect {
        hight: 30,
        width: 50,
    };

    let res2 = Rect::square(20);

    let rect3 = Rect {
        hight: 10,
        width: 40,
    };

    println!("The area of the rectangle is: {}", rect1.area());

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let mut rect4 = Rect { hight: 4, width: 2 };

    println!("Before area  {}", rect4.area());

    rect4.scale(2);

    println!("after area  {}", rect4.area());
}
