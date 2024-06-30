

fn main() {
let dimensions = Rectangle {
    width: 30 ,
    height : 50,
};
let rect1  = Rectangle {
    width: 10,
    height: 30 
};
let rect2= Rectangle {
    height: 50,
    width: 80
};
println!("can default rectangle hold rect1 {} ", dimensions.can_hold(&rect1));
println!("can default rectangle hold rect2 {}", dimensions.can_hold(&rect2));
// println!("square {}", dimensions::square(10));
println!("area of a rectanlge is {dimensions:#?}" );
println!("area of a rectanlge is {}", calculate_area(&dimensions));
println!("area of a rectanlge through method is {}", dimensions.area());

}
fn calculate_area (rectangle : &Rectangle)-> i32 {
     dbg! (rectangle.width * rectangle.height)
}
#[derive(Debug)]
struct Rectangle {
    width : i32 ,
    height : i32
}

impl Rectangle {


    // fn square (size : i32)-> Self {
    //     Self {
    //         width: size,
    //         height: size ,
    //     }
    // }
    fn area(&self) -> i32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle)->bool{
        self.width >= other.width && self.height>= other.height
    }
}