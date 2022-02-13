pub fn is_odd(num: u8)-> u8{
    return num%2;
}

fn main(){
    println!("above function gives {}, below function {}", is_odd(7),is_even(3));
}

pub fn is_even(num: u8)-> bool{
    let digit: u8 = num%2;
    return digit == 0;
}