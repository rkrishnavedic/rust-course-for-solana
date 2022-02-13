fn main(){
    let arr: [u8; 5] = [1,2,3,9,18];
    let arr2: [u8; 5] = [100;5];

    println!("index: {}, length {}", arr[0], arr2.len());

    // structure of the array or any other objects
    println!("{:?}", arr);

    // Slices are subarrays
    let slice = &arr[1..4]; // [2,3] ; slice dont have length property on compile time
    println!("{:?}", slice);
    borrow_slice(arr, slice);
}

fn borrow_slice(arr: [u8;5], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}