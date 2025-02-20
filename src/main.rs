mod app;
mod wordlist;

use clap::Parser;
use wordlist::Wordlist;
use app::App;

fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(u) => u,
        Err(e) => panic!("{}", e),
    };
    buffer
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let args = App::parse();
    if !args.no_reroll {
        let password = Wordlist::get_password(args.number);
        println!("{}", password);
        return;
    }

    clear_screen();
    println!("\x1b[31mr + enter to reroll\x1b[0m");
    println!("{}", Wordlist::get_password(args.number));
    loop {
        let input = get_input();
        match input.chars().nth(0) {
            Some(input) => match input {
                'r' => {
                    clear_screen();
                    println!("{}", Wordlist::get_password(args.number));
                },
                _ => {
                    clear_screen();
                    break;
                }
            },
            _ => {
                break;
            }
        }
    }
}
