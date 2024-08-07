use rand::prelude::*;
use std::collections::HashMap;

pub struct Wordlist(HashMap<String, String>);

impl Default for Wordlist {
    fn default() -> Self {
        let wordlist:HashMap<String, String> = match serde_json::from_str(include_str!("effwordlist.json")){
            Ok(s)=>s,
            Err(_)=>panic!("Failed to deserialize wordlist")
        };
        Wordlist(wordlist)
    }
}

impl Wordlist {
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

    fn get_word()->String{
        if let Some(result) = Self::default().0.remove(&Self::roll()){
            return result
        } else {
            panic!("Failed to get word from wordlist.")
        }
    }

    pub fn get_password(n:usize)->String{
        let password:String = (1..=n)
            .map(|_| Self::proper_case(Self::get_word()))
            .collect::<Vec<String>>()
            .join("");

        password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_is_5_digits(){
        assert_eq!(Wordlist::roll().len(), 5);
    }

    #[test]
    fn proper_case_capitalizes_first_letter(){
        assert_eq!(Wordlist::proper_case(String::from("foo")), "Foo");
    }

    #[test]
    fn proper_case_has_no_effect_on_all_caps(){
        assert_eq!(Wordlist::proper_case(String::from("BAR")), "BAR");
    }
}