

fn main() {
let dipesh = Variants::A;
let kala = Variants:: B;
println!("{:?}",dipesh);
println!("{:?}",kala);
let result = match kala {
    Variants::A => "dipesh",
    Variants::B => "kala",
};
if let Variants::A = dipesh{
    println!("hello");
}
println!("{:?}",result);

}
#[derive(Debug)]
enum Variants {
    A, B
}