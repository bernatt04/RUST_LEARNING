fn main() {
    //day 1
    let x = 1;
    print!("{}", x);

    //day 2
    let mut y = 2; //mut simply means the variable will be able to be changed.
    y = 5;
    print!("{}", y);

    let tuple = (1, 2, "hola"); // creation of a tuple, don't really use it in other languages but will see if this one is different

    let mut arr = [1, 2, 3, 4, 5]; //creation of array, normal.

    println!("{:?}", arr);

    arr[2] = 5;

    println!("{:?}", arr); // have to use :? as an array does not implement display it implements debug. If instead i would have done a print of a string or int i could have used just {}
}
