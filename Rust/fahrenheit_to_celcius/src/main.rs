use std::io;

fn main() {
    let fahrenheit = ask_user_input();

    let celcius = convert_f_to_c(fahrenheit);

    println!("{} degrees in Celcius is {}", fahrenheit, celcius)
}

fn ask_user_input() -> f64 {

    let mut fahrenheit = String::new();

    println!("Enter temperature in Fahrenheit");

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: u32 = fahrenheit.trim().parse()
        .expect("Please type a number!");

    f64::from(fahrenheit)
}

fn convert_f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.800
}
