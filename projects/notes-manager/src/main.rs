use std::fs;
use std::fs::File;
use std::io;

fn main() {
    print!("Choices:");
    println!("1. New note \n 2. Open note \n 3. Delete note");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error reading input."); //saving input into input variable

    let inputnum: i32 = input.trim().parse().unwrap(); //convert string into int

    //case 1 create file
    if inputnum == 1 {
        println!("please enter the name of the file");
        let mut filename = String::new();
        io::stdin()
            .read_line(&mut filename)
            .expect("error leyendo el nombre del archivo"); //expect = except in java

        let filename = format!("{}.txt", filename.trim()); // using trim to get rid of the \n that readline automatically adds.
        File::create(filename).unwrap();

        //case 3, Delete file.
    } else if inputnum == 3 {
        let entries = fs::read_dir(".").unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            print!("{:?}", entry);
        }
    }
}
