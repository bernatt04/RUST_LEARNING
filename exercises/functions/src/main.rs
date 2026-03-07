//in this file i will be creating different functions and experimenting with recursion too.

fn add(a: f32, b: f32) -> f32 {
    return a + b;
}

fn join_letters(a: String, b: String) -> String {
    return a + &b;
}

fn main() {
    let a = "hello ";
    let b = "world";
    print!("{}", join_letters(a.to_string(), b.to_string()));
    print!("{}", add(4.0, 5.0));
}
