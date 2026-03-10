//in this file i will be creating different functions and experimenting with recursion too.

fn add(a: f32, b: f32) -> f32 {
    return a + b;
}

fn join_letters(a: String, b: String) -> String {
    return a + &b;
}

//trial of recursion
fn fibo(a: i32) -> i32 {
    if a <= 1 {
        return a;
    }
    return fibo(a - 1) + fibo(a - 2);
}

fn main() {
    let a = "hello ";
    let b = "world";
    print!("{}", fibo(12));
    print!("{}", join_letters(a.to_string(), b.to_string()));
    print!("{}", add(4.0, 5.0));
}
