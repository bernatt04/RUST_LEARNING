use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

fn main() {
    while true {
        print!("\nChoices:\n");
        println!("1. New note \n 2. Open note \n 3. Delete note \n 4. Exit");

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
        }
        //case 2, open file.
        else if inputnum == 2 {
            let entries = fs::read_dir(".").unwrap();
            let mut num = 1;
            let mut files: Vec<String> = Vec::new();
            print!("\nFILE NAMES:\n");
            for entry in entries {
                let entry = entry.unwrap();
                let name = entry.file_name();
                let name = name.to_string_lossy();
                print!(" {}. {}\n", num, name);
                files.push(name.to_string());
                num = num + 1;
            }

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("error");

            for i in files {
                if i == choice.trim() {
                    let mut file = File::open(i).unwrap();
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();

                    println!("{}", content);
                }
            }
        }
        //case 3, delete file.
        else if inputnum == 3 {
            let entries = fs::read_dir(".").unwrap();
            let mut num = 1;
            let mut files: Vec<String> = Vec::new();
            for entry in entries {
                //show files in folder and save entries in a vector.
                let entry = entry.unwrap();
                let name = entry.file_name();
                let name = name.to_string_lossy();
                print!("{}. {}\n", num, name);
                files.push(name.to_string());
                num = num + 1;
            }

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("error");

            for i in files {
                if i == choice.trim() {
                    fs::remove_file(i).unwrap();
                    print!("file deleted");
                }
            }
        }
        //case 4, exit
        else if inputnum == 4 {
            exit(0);
        }
    }
}
