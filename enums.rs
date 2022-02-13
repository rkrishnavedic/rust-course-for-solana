fn main(){
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C{x: 10, y:20};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

#[derive(Debug)]
enum MyEnum{
    A, 
    B(i32),
    C {x: i32, y:  i32}
}