fn main(){
    let tuple: (u64, i8, bool, f32) = (2,-2,false, 7.8);
    let tuple2 = (4.7, false, 2);

    //print the structure
    println!("first {}, third {}", tuple.0, tuple.2);
    println!("{:?}", tuple2);

    // destructuring
    let (a,b,c,d) = tuple;

    println!("fourth{}, second{}", d,b);

}