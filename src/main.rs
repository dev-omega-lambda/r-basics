fn main() {
    let heigth: f64 = 1.7;
    let age: i32 = 26;
    let is_happy: bool = true;

    println!(
        "Hi, my name's Bruno...\nI've height {} and {} years old I'm {} very happy",
        heigth, age, is_happy
    );

    let mut count = 0;
    println!("Inital value of count {}", count);
    count += 1;
    println!("Update value of count {}", count);

    let x = 5;
    println!("First value for x {}", x);
    {
        let x = x + 1;
        println!("Second value for x {} inside the scope", x);
    }
    let x = x + 4;
    println!("Third value for x {} outside the scope", x);

    let msg = "Rust is amazing!";
    let transformed_sentence: String = msg.chars().map(|c| c.to_ascii_uppercase()).collect();
    println!("Transformed: {}", transformed_sentence);
}
