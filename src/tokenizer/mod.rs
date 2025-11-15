pub(crate) mod tokens;

use crate::tokenizer::tokens::*;
use Token::Whitespace;

pub struct Tokenizer {}

impl Tokenizer {
    pub(crate) fn tokenize(input: &str) -> Vec<Token> {
        println!("Tokenizing \"{}\"...", input);

        let mut index = 0;
        let mut tokens: Vec<Token> = vec![];

        // while there is still some file content left to parse
        while index < input.len() {
            let mut has_match = false;

            // for every token we know about
            for (regex, creator) in TOKEN_CONVERTERS.iter() {
                // make each regex the start of the line (only test the start of input)

                // get the first match
                let first_match = regex.find(&input[index..]);

                // if we have a match...
                if let Ok(match_optional) = first_match {
                    if match_optional.is_none() {
                        continue;
                    }

                    has_match = true;

                    // get the token from the match
                    //println!("Testing regex: {}", r.to_string());
                    let match_str = match_optional.unwrap();

                    //println!("Match: {}", match_str.as_str());
                    let token = creator(index as i128, match_str.as_str().parse().unwrap());
                    //println!("{}", regex.replace(&input, format!("[{:?}]", token)));

                    // increase the index by how much we move
                    index += match_str.end() - match_str.start();

                    // we don't care about whitespace or linebreaks
                    if !(matches!(token, Whitespace) || matches!(token, Token::LineBreak { .. })) {
                        // add tokens to the list
                        tokens.push(token);
                    }
                    break;
                } else {
                    // println!("regex not found")
                }
            }

            // if no tokens match, error
            if !has_match {
                println!("No match found for: {}", &input[index..]);
                break;
            }
        }

        println!("Tokenizer finished: {:?}", tokens);

        // add the end of file token
        tokens.push(Token::Eof);
        // return list of tokens
        tokens
    }
}
