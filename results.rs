#[derive(Debug)]
enum MyError{
    Error1
}

// Err, an enum that contains an error code
//  Ok(value), Awrapper that contains a value

fn divide(dividend: i32, divisor: i32)-> Result<i32, MyError>{
    if dividend % divisor !=0{
        Err(MyError::Error1)
    }else{
        Ok(dividend/divisor)
    }
}

fn main(){
    let divide =  divide(4,2);
    let res = divide.expect("we crashed");
    // match divide{
    //     Ok(v)=> println!("{}", v),
    //     Err(v)=> println!("{:?}", v)
    // }

    // if divide.is_ok(){
    //     println!("{} ok", divide.unwrap());
    // }

    // println!("{} ok direct", divide.unwrap());
    // println!("{} ok or", divide.unwrap_or(100));
    println!("{}", res);
}