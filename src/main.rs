use regex::Regex;

//define an enum for expressions
#[derive(Debug)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

//a simple parser that only understands simple arithmetics
fn parse_expression(input: &str) -> Expr {
    let re = Regex::new(r"(\d+|\+|\-|\*|\/)").unwrap();
    let tokens: Vec<&str> = re.find_iter(input).map(|mat| mat.as_str()).collect();

    let mut stack: Vec<Expr> = Vec::new();
    let mut op_stack: Vec<&str> = Vec::new();

    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                while !op_stack.is_empty()
                    && precedence(op_stack.last().unwrap()) >= precedence(token)
                {
                    let op = op_stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(apply_op(op, left, right));
                }
                op_stack.push(token);
            }
            num => {
                stack.push(Expr::Number(num.parse().unwrap()));
            }
        }
    }

    while !op_stack.is_empty() {
        let op = op_stack.pop().unwrap();
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();

        stack.push(apply_op(op, left, right));
    }
    stack.pop().unwrap()
}

fn apply_op(op: &str, left: Expr, right: Expr) -> Expr {
    match op {
        "+" => Expr::Add(Box::new(left), Box::new(right)),
        "-" => Expr::Sub(Box::new(left), Box::new(right)),
        "*" => Expr::Mul(Box::new(left), Box::new(right)),
        "/" => Expr::Div(Box::new(left), Box::new(right)),
        _ => panic!("Unknown operator: {}", op),
    }
}

fn precedence(op: &str) -> usize {
    match op {
        "-" | "+" => 1,
        "/" | "*" => 2,
        _ => 0,
    }
}

fn codegen(exp: &Expr) -> String {
    match exp {
        Expr::Number(n) => n.to_string(),
        Expr::Add(left, right) => format!("{} + {}", codegen(left), codegen(right)),
        Expr::Sub(left, right) => format!("{} - {}", codegen(left), codegen(right)),
        Expr::Mul(left, right) => format!("{} * {}", codegen(left), codegen(right)),
        Expr::Div(left, right) => format!("{} / {}", codegen(left), codegen(right)),
    }
}

fn main() {
    let js_expression = "3 + 4 + 6";
    let parsed_expression = parse_expression(js_expression);
    let output = codegen(&parsed_expression);

    println!("Parsed Expression: {:?}", parsed_expression);
    println!("Output : {}", output);
}
