use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use chumsky::{error, prelude::*};
use extra::Err;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Ast {
    Neg(Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Num(i64),
}

fn parser<'a>() -> impl Parser<'a, &'a str, Ast, Err<error::Rich<'a, char>>> {
    recursive(|value| {
        let token = |c| just(c).padded();

        let number = token('-')
            .or_not()
            .then(text::int(10))
            .to_slice()
            .map(|text: &str| Ast::Num(text.parse::<i64>().unwrap()));

        let nested = value.clone().delimited_by(token('('), token(')'));

        let atom = choice((number, nested));

        let unary = token('-')
            .ignore_then(value.clone())
            .map(Box::new)
            .map(Ast::Neg);

        let factor = choice((unary, atom));

        let product = factor.clone().foldl(
            choice((token('*'), token('/')))
                .then(factor.clone())
                .repeated()
                .at_least(1),
            |a, (op, b)| match op {
                '*' => Ast::Mul(Box::new(a), Box::new(b)),
                '/' => Ast::Div(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            },
        );

        let term = choice((product, factor));

        let sum = term.clone().foldl(
            choice((token('+'), token('-')))
                .then(term.clone())
                .repeated()
                .at_least(1),
            |a, (op, b)| match op {
                '+' => Ast::Add(Box::new(a), Box::new(b)),
                '-' => Ast::Sub(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            },
        );

let expression = choice((sum, term));

        expression
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let parser = parser();
        let (output, errors) = parser.parse(input).into_output_errors();
        if let Some(ast) = output {
            println!("{:?}", ast);
        } else {
            for error in errors {
                println!("{}", error);
            }
        }
    }
    Ok(())
}
