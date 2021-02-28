use phrases::english::farewell;
use phrases::english::greetings;
fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewell::goodbye());
    println!("Hello in Japan: {}", phrases::japan::greetings::hello());
    println!("Goodbye in Japan: {}", phrases::japan::farewell::goodbye());
}
