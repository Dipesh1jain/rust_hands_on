

fn main() {
let dimensions = Rectangle {
    width: 30 ,
    height : 50,
};
println!("area of a rectanlge is {dimensions:#?}" );
println!("area of a rectanlge is {}", calculate_area(&dimensions));

}
fn calculate_area (rectangle : &Rectangle)-> i32 {
     dbg! (rectangle.width * rectangle.height)
}
#[derive(Debug)]
struct Rectangle {
    width : i32 ,
    height : i32
}