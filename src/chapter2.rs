use rand::Rng;
#[allow(dead_code)]

pub fn guessing_game() {
    let lower = 0;
    let upper = 100;
    let secret_number = rand::rng().random_range(lower..upper);
    println!("Guess a number between {lower} and {}", upper - 1);
    'game: loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue 'game,
        };
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break 'game;
            }
        }
    }
}
