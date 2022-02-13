fn main(){
    // unsigned integer u8,...,u128
    let unsigned: u8 = 9;

    //signed integer i8,...,i128
    let signed: i8 = -2;

    // float for decimals;
    let float: f32 = 0.54;

    println!("lets see {} and {} and also  {}", unsigned, signed, float);

    //char 
    let letter = "c";
    let emoji =  "\u{1f90c}";
    println!("{} {}", letter, emoji);

    let boolvar: bool =  true && false || true;
    println!("boolean: {}", boolvar);

}