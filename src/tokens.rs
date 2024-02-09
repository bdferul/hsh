use std::{
    iter::once,
    ops::{Deref, DerefMut},
};

use tap::Pipe;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Pipe,
    //Semicolon,
    Str(String),
}

impl Token {
    /// panics with Token::Pipe
    pub fn str(&self) -> &str {
        match self {
            Self::Str(s) => s,
            _ => panic!("called Token::str() on {self:?}"),
        }
    }
}

pub trait Tokenize {
    fn tokenize(self) -> Tokens;
}

impl<T: AsRef<str>> Tokenize for T {
    fn tokenize(self) -> Tokens {
        let mut r = Tokens(Vec::new());

        let mut chars = self.as_ref().chars().peekable();
        let mut in_quote = false;
        while let Some(c) = chars.next() {
            match c {
                '|' => r.push(Token::Pipe),
                //';' => r.push(Token::Semicolon),
                '\'' => in_quote = !in_quote,
                _ if c.is_whitespace() && !in_quote => (),
                _ => {
                    if in_quote {
                        chars
                            .by_ref()
                            .take_while(|x| *x != '\'')
                            .pipe(|xs| once(c).chain(xs))
                            .collect::<String>()
                            .pipe(Token::Str)
                            .pipe(|x| r.push(x));
                    } else {
                        chars
                            .by_ref()
                            .take_while(|x| !x.is_whitespace())
                            .pipe(|xs| once(c).chain(xs))
                            .collect::<String>()
                            .pipe(Token::Str)
                            .pipe(|x| r.push(x));
                    }
                }
            }
        }

        r
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tokens(Vec<Token>);

impl Deref for Tokens {
    type Target = Vec<Token>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tokens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn t() {
    use Token::*;
    let cmd = "ls | grep 'kill tyler'";
    let tokens = vec![
        Str("ls".to_string()),
        Pipe,
        Str("grep".to_string()),
        Str("kill tyler".to_string()),
    ]
    .pipe(Tokens);

    assert_eq!(cmd.tokenize(), tokens)
}
