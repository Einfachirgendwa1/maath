use std::{collections::HashMap, error::Error, fmt::Display};

use anyhow::{Context, Result};

macro_rules! f {
    ($($e:expr), *) => {
        Function {
            arguments: vec![$($e)*],
            term: FunctionTerm::Value(Value::Literal(0.)),
        }
    };
}

macro_rules! solve {
    ($e:ident($($args:expr), *)) => {
        $e.solve_args_in_order(vec![$($args)*])
    };
}

struct Function {
    arguments: Vec<char>,
    term: FunctionTerm,
}

impl Function {
    fn solve_for(&self, args: &HashMap<char, f64>) -> Result<f64> {
        self.term.solve(&args)
    }

    fn solve_args_in_order(&self, in_order: Vec<f64>) -> Result<f64> {
        self.solve_for(
            &self
                .arguments
                .iter()
                .zip(in_order)
                .map(|(x, y)| (*x, y))
                .collect(),
        )
    }

    fn variable(&self, name: char) -> Result<Box<FunctionTerm>> {
        if !self.arguments.contains(&name) {
            Err(MyError::NoSuchVariable { variable: name })?
        }
        Ok(FunctionTerm::Variable(name).into())
    }
}

impl From<u32> for Box<FunctionTerm> {
    fn from(value: u32) -> Self {
        FunctionTerm::Value(Value::Literal(value as f64)).into()
    }
}

enum FunctionTerm {
    Variable(char),
    Value(Value),
    Calculation {
        left: Box<FunctionTerm>,
        right: Box<FunctionTerm>,
        operation: Operation,
    },
}

impl FunctionTerm {
    fn solve(&self, args: &HashMap<char, f64>) -> Result<f64> {
        let result = match self {
            Self::Value(x) => x.get()?,
            Self::Variable(x) => args[&x],
            Self::Calculation {
                left,
                right,
                operation,
            } => operation.apply(
                left.solve(args)
                    .context("Failed to solve the left hand side.")?,
                right
                    .solve(args)
                    .context("Failed to solve the right hand side.")?,
            )?,
        };

        Ok(result)
    }
}

enum Value {
    Literal(f64),
    _Calculation {
        left: Box<Value>,
        right: Box<Value>,
        operation: Operation,
    },
}

impl Value {
    fn get(&self) -> Result<f64> {
        Ok(match self {
            Self::Literal(x) => *x,
            Self::_Calculation {
                left,
                right,
                operation,
            } => operation.apply(
                left.get().context("Failed to solve the left hand side.")?,
                right
                    .get()
                    .context("Failed to solve the right hand side.")?,
            )?,
        })
    }
}

enum Operation {
    _Plus,
    _Minus,
    _Multiply,
    _Divide,
    Pow,
}

#[derive(Debug)]
enum MyError {
    DivisionByZero,
    NoSuchVariable { variable: char },
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "Cannot divide by zero!"),
            Self::NoSuchVariable { variable } => write!(
                f,
                "The variable {variable} does not exist for this function."
            ),
        }
    }
}

impl Error for MyError {}

impl Operation {
    fn apply(&self, left: f64, right: f64) -> Result<f64> {
        let result = match self {
            Self::_Plus => left + right,
            Self::_Minus => left - right,
            Self::_Multiply => left * right,
            Self::_Divide => {
                if right == 0. {
                    Err(MyError::DivisionByZero)?
                }
                left / right
            }
            Self::Pow => left.powf(right),
        };

        Ok(result)
    }
}

fn main() {
    // f(x) = x²
    let mut f = f!('x');
    f.term = FunctionTerm::Calculation {
        left: f.variable('x').unwrap(),
        right: 2.into(),
        operation: Operation::Pow,
    };

    for x in (0..=10).map(|x| x as f64) {
        println!("f({x}) = {}", solve!(f(x)).unwrap())
    }
}
