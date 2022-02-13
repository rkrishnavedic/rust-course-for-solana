fn main(){
    println!("Now we have for loop");
    for i in 0..6{
        println!("{}", i);
        if i==3{
            break;
        }
    }

    println!("Now we have while loop");
    let mut i = 0;
    while i<5{
        println!("{}", i);
        i+=1;
        if i==2{
            println!("exiting");
            break;
        }
    }
}