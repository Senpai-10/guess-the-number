use rand::Rng;
use std::io::Write;

fn main() {
    /*
    TODO: maybe take min, max number and max_attempts from cli
        EXAMPLE:    -m,--min,
                    -M, --max
                    -a, --attempts
    */
    let settings = Settings {
        min: 1,
        max: 100,
        max_attempts: 10,
    };
    let mut current_attempts_count: u16 = 0;
    let random_number: u16 = rand::thread_rng().gen_range(settings.min..settings.max + 1);

    println!(
        "Guess a number between {} and {} in {} attempts",
        settings.min, settings.max, settings.max_attempts
    );

    loop {
        let input = prompt("guess: ");

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        if is_string_numeric(&input) {
            current_attempts_count += 1;
            let left_attempts: u16 = settings.max_attempts - current_attempts_count;

            let guess: u16 = match input.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if left_attempts == 0 {
                println!("You lost, the random number is {}", random_number);
                break;
            }

            if guess == random_number {
                println!("You win in {} attempt", current_attempts_count);
                break;
            } else if guess > random_number {
                println!("The number is lower");
            } else if guess < random_number {
                println!("The number is higher");
            }

            println!("{} attempts are left", left_attempts);
        }
    }
}
struct Settings {
    min: u16,
    max: u16,
    max_attempts: u16,
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn is_string_numeric(string: &String) -> bool {
    for c in string.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
