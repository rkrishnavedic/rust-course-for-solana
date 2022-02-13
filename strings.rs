fn main(){
    let str: &str = "here is the string";
    let mut string: String = String::from("the String Class");

    let slice = &string[1..4];
    println!("..\n..{} {}", str,slice);
    slice.len();

    string.push('1');
    string.push_str("! help");
    println!("{}",string);
    string = string.replace("help", "good");
    println!("{}", string);
}