use crate::wordlist::Wordlist;

pub struct App {}

impl App {
    fn get_arg() -> usize {
        let args: Vec<String> = std::env::args().collect();

        // Set default number of words to generate.
        let mut length: usize = 3;

        if args.len() > 1 {
            length = match args[1].parse() {
                Ok(i) => i,
                Err(_) => {
                    println!("Input not an integer. Using defaults.");
                    length
                }
            }
        }

        if length > 5 || length < 2 {
            println!("Password length must be 2 <= length <=5 ");
            length = 3;
        }

        length
    }

    fn get_input() -> String {
        let mut buf = String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buf) {
            Ok(u) => u,
            Err(e) => panic!("{}", e),
        };
        buf
    }

    fn clear_screen() {
        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn run() {
        let password_length = Self::get_arg();

        println!("r + enter to reroll");
        println!("{}", Wordlist::get_password(password_length));
        loop {
            // Get input
            let input = Self::get_input();

            match input.chars().nth(0) {
                Some(input) => match input {
                    'r' => println!("{}", Wordlist::get_password(password_length)),
                    _ => {
                        Self::clear_screen();
                        break;
                    }
                },
                _ => {
                    Self::clear_screen();
                    break;
                }
            }
        }
    }
}
