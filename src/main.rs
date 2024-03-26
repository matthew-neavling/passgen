use rand::prelude::*;
use std::io;
use std::collections::HashMap;


fn get_wordlist()->HashMap<String, String>{
    let wordlist:HashMap<String, String> = match serde_json::from_str(include_str!("effwordlist.json")){
        Ok(s)=>s,
        Err(_)=>panic!("Couldn't deserialize wordlist")
    };
    wordlist
}

fn roll()->String{
    let roll:[u8;5] = core::array::from_fn(|_| rand::thread_rng().gen_range(1..=6));
    roll.map(|u|u.to_string()).join("")
}

fn proper_case(input:String)->String{
    let mut chars:Vec<char> = input.chars().collect();
    chars[0] = match chars[0].to_uppercase().nth(0){
        Some(c)=>c,
        _=>panic!("Failed to uppercase char")
    };
    let proper: String = chars.iter().collect();
    proper

}

fn get_password(wordlist:HashMap<String, String>, n:i32)->String{
    let password:String = (1..=n)
        .map(|_|proper_case(wordlist[&roll()].clone()))
        .collect::<Vec<String>>()
        .join("");

    password
}

fn main() {
    let args:Vec<_> = std::env::args().collect();

    
    
    // Set default number of words to generate. 
    let mut n:i32 = 3;

    // If an argument was passed, attempt to parse.
    if args.len() > 1 {
        n = match args[1].parse(){
            Ok(i)=>i,
            Err(_)=>{
                println!("Input not an integer. Using defaults.");
                3
            }
        }
    }
    
    println!("{}", get_password(get_wordlist(), n));

    let mut buf = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buf){
        Ok(u)=>u,
        Err(e)=>panic!("{}", e)
    };

    match buf.chars().nth(0).unwrap() {
        'r'=>{println!("{}", get_password(get_wordlist(), n))},
        _=>println!("{}[2J", 27 as char)
    }


}
