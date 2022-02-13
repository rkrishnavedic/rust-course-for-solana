fn main(){
    /*
    this gives error as you cannot re-assign in rust unless specified to be mutable
    let a = 5;
    a = 4;
    println!("{}", a);
    */

    let mut a = 5;
    a = 4;
    println!("{}", a);
}