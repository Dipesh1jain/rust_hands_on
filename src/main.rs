

fn main() {
  let mut name = String::from("dipesh");
 let fullname = add_surname(&mut name);
println!("fullName is {fullname}");
}

fn add_surname(s : &mut String)-> &mut String{
 s.push_str(" kala");
 s

}
