use operation::{Builder, Operation};
mod operation;
mod operator;

fn main() {
    println!("Enter Expression: ");

    // Read inpet from console
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let tokens = tokenize(&input);

    let op = parse(&tokens, None);

    let res = op.evaluate();

    println!("Result: {}", res);
}

type Token = String;
type Tokens = Vec<Token>;
type TokensSlice = [Token];

fn parse(tokens: &TokensSlice, builder: Option<Builder>) -> Box<dyn Operation> {
    if builder.is_none() {
        let builder = Builder::new();
        return parse(tokens, Some(builder));
    }

    let mut builder = builder.unwrap();

    if tokens.is_empty() {
        return builder.build();
    }

    let mut iter = tokens.iter().enumerate();

    while let Some((idx, token)) = iter.next() {
        if *token == '('.to_string() {
            let mut depth = 1;
            while let Some((inner, token)) = iter.next() {
                if *token == '('.to_string() {
                    depth += 1;
                }
                if *token == ')'.to_string() {
                    depth -= 1;
                }
                if depth == 0 {
                    let inner = &tokens[idx + 1..idx + inner];
                    let operation = parse(inner, None);
                    builder.next(operation);
                    break;
                }
            }
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
                tokens.push(token.clone());
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
            tokens.push(c.to_string());
            push_token!();
            continue;
        }

        if operator::Operator::is_operator(&c) {
            push_token!();
            tokens.push(c.to_string());
            push_token!();
            continue;
        }
    }

    tokens
}
