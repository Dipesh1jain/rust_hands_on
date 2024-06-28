use std::io;

fn main() {
    let arr: [i32; 10] = [3, 5, 67, 78, 64, 2, 47, 9, 8, 0];
    let mut getindex = String::new();
    io::stdin()
        .read_line(&mut getindex)
        .expect("Failed to read line ");
    let getindex: usize = getindex
        .trim()
        .parse()
        .expect("index entered is not an number ");
    let condition: &str = "firstName";
    let name = if condition == "firstName" {
        "dipesh"
    } else {
        "kala"
    };

    println!("Name is : {}", name);
    if getindex >= arr.len() {
        println!("please enter the number within the range ");
    } else {
        println!(
            "element present at the index {} is : {}",
            getindex, arr[getindex]
        )
    }
  for element in arr {
    println!("arr number is {}",element)
  }
  for range in 0..=arr.len() - 5{
    println!("arr number is using range {}",range)
  }
}
