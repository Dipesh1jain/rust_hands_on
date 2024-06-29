

fn main() {
let full_string =String::from(" he l l l o");
let first_word = first_word(&full_string);
println!("{first_word}")
}

fn first_word(s: &String)-> String {
let  mut first_word =String::new();
// let bytes = s.as_bytes();
for i in s.chars() {
    if i == ' '{
        break;
    }
    else{
        first_word.push(i)}

};
first_word
}
