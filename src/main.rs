use operation::{Builder, Operation};
use operator::Operator;
mod operation;
mod operator;
mod token;
use token::Token;

fn main() {
    println!("Enter Expression: ");

    // Read inpet from console
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = tokenize(&input);

    let mut iter = tokens.iter_mut();

    let op = parse(&mut iter, None);

    let res = op.evaluate();

    println!("Result: {}", res);
}

type Tokens = Vec<Token>;
type TokensSlice = [Token];

fn parse<'a, T>(tokens: &mut T, builder: Option<Builder>) -> Box<dyn Operation>
where
    T: Iterator<Item = &'a mut Token>,
{
    if builder.is_none() {
        let builder = Builder::new();
        return parse(tokens, Some(builder));
    }

    let mut builder = builder.unwrap();

    while let Some(token) = tokens.next() {
        if token.is_open_parenthesis() {
            let mut depth = 1;
            while let Some(token) = tokens.next() {
                if token.is_open_parenthesis() {
                    depth += 1;
                }
                if token.is_close_parentesis() {
                    depth -= 1;
                }
                if depth == 0 {
                    let operation = parse(tokens, None);
                    builder.next(operation);
                    break;
                }
            }
        }

        if let Token::Operator(op) = token {
            builder.operator(*op);
            continue;
        }
    }

    builder.build()
}

fn tokenize(input: &str) -> Tokens {
    let mut tokens = Vec::new();

    let mut token = String::new();

    macro_rules! push_token {
        () => {
            if !token.is_empty() {
                let num = token.parse::<f64>().unwrap();
                let boxed = Box::new(num);
                tokens.push(Token::Operation(boxed));
                token.clear();
            }
        };
    }

    for c in input.chars() {
        if c.is_ascii_digit() {
            token.push(c);
            continue;
        }

        if c.is_whitespace() {
            push_token!();
            continue;
        }

        if c == '(' || c == ')' {
            push_token!();
            match c {
                '(' => tokens.push(Token::OpenParenthhesis),
                ')' => tokens.push(Token::CloseParenthesis),
                _ => {}
            }
            continue;
        }

        if let Ok(op) = operator::Operator::from_str(&c.to_string()) {
            push_token!();
            tokens.push(Token::Operator(op));
        }
    }

    tokens
}
