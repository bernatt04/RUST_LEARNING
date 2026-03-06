//this file will serve for loops and ifs as i dont want to create a full freaking folder just for ifs

fn main() {
    //for loop basic
    for x in 0..20 {
        println!("{}", x);
    }

    //for with ifs
    for x in 0..20 {
        if x % 2 == 0 {
            println!("{}", x);
        }
    }

    //while loop
    let mut y = 0;
    while y < 20 {
        print!("{}", y);
        y += 1;
    }

    //im not doing a while with if, same as with for.

    //for loop array traversal
    let mut arr = [1, 2, 3, 4];
    for value in arr {
        print!("{}", value);
        arr[value - 1] = 0;
    }
    print!("{:?}", arr);
}
