fn main() {
    hello_word("Hello, world!");
}

fn hello_word(text: &str) -> () {
    println!("{}", text);
}
