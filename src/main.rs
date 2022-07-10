use serde::{Deserialize, Serialize};
use std::io::{self, Read};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Serialize, Deserialize, Debug)]
struct Expression {
    left: Point,
    ope: Operator,
    right: Point,
}

fn main() {
    let mut x = String::new();
    io::stdin()
        .read_to_string(&mut x)
        .expect("failed to read from stdin");
    println!("stdin: {}", x);

    let deserialized = serde_json::from_str::<Expression>(&x);

    match deserialized {
        Ok(deserialized) => {
            let left = deserialized.left;
            let right = deserialized.right;

            let p = match deserialized.ope {
                Operator::Plus => Point {
                    x: left.x + right.x,
                    y: left.y + right.y,
                },
                Operator::Minus => Point {
                    x: left.x - right.x,
                    y: left.y - right.y,
                },
                Operator::Mul => Point {
                    x: left.x * right.x,
                    y: left.y * right.y,
                },
                Operator::Div => Point {
                    x: left.x / right.x,
                    y: left.y / right.y,
                },
            };
            println!("expression result = {:?}", p)
        }
        Err(err) => println!("err is {}", err),
    }
}
