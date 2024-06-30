

fn main() {
let mut  user1_data = User {
    name : String::from("Dipesh"),
    sur_name : String::from("kala"),
    age :23,
    active: true
};
user1_data.sur_name = String::from("jain");
println!("{}",user1_data.name);
println!("{}",user1_data.sur_name);
println!("{}",user1_data.age);
println!("{}",user1_data.active);

let user2_data = User {
    sur_name: String::from("kala"),
    ..user1_data
};
println!("user 2 : {}",user2_data.name);
println!("user 2 : {}",user2_data.sur_name);
println!("user 2 : {}",user2_data.age);
println!("user 2 : {}",user2_data.active);

let color_sruct = Color(1,0);
println!("{}",color_sruct.0);
println!("{}",color_sruct.1);
}
struct  User {
    name : String,
    sur_name: String,
    age : i32,
    active : bool 
} 
// tuple struct 
struct Color(i32,i32);