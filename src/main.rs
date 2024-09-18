use regex::Regex;
use std::collections::HashMap;

//define an enum for expressions
#[derive(Debug)]
enum Expr {
    Number(i64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

//struct for variable declaration
#[derive(Debug)]
struct VariableDeclaration {
    name: String,
    value: Expr,
}

//struct for programs state
#[derive(Debug)]
struct Program {
    variables: HashMap<String, i64>,
    statements: Vec<Stmt>,
}

#[derive(Debug)]
enum Stmt {
    Assignment(String, Expr),
    Declaration(VariableDeclaration),
    Expression(Expr),
}

// a simple parser for variable declation , assignment and expressions
fn parse_program(input: &str) -> Program {
    let mut program = Program {
        variables: HashMap::new(),
        statements: Vec::new(),
    };
    let re_stmt = Regex::new(r"(\w+)\s*=\s*(\d+|[\w\s\+\-\*/\(\)])").unwrap();
    let re_decl = Regex::new(r"var\s+(\w+)\s*=\s*(\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = re_decl.captures(line) {
            let var_name = caps.get(1).unwrap().as_str().to_string();
            let value = caps.get(2).unwrap().as_str().parse().unwrap();
            program.variables.insert(var_name.clone(), value);
            program
                .statements
                .push(Stmt::Declaration(VariableDeclaration {
                    name: var_name,
                    value: Expr::Number(value),
                }));
        } else if let Some(caps) = re_stmt.captures(line) {
            let var_name = caps.get(1).unwrap().as_str().to_string();
            let value_exp = parse_expression(caps.get(2).unwrap().as_str());
            program
                .statements
                .push(Stmt::Assignment(var_name, value_exp));
        } else {
            let exp = parse_expression(line);
            program.statements.push(Stmt::Expression(exp));
        }
    }
    program
}

//a simple parser that only understands simple arithmetics

fn parse_expression(input: &str) -> Expr {
    let re = Regex::new(r"(\d+|\+|\-|\*|\/|\(|\))").unwrap();
    let tokens: Vec<&str> = re.find_iter(input).map(|mat| mat.as_str()).collect();

    let mut output_queue: Vec<Expr> = Vec::new();
    let mut operator_stack: Vec<&str> = Vec::new();

    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                while !operator_stack.is_empty()
                    && operator_stack.last().unwrap() != &"("
                    && precedence(operator_stack.last().unwrap()) >= precedence(token)
                {
                    let op = operator_stack.pop().unwrap();
                    apply_op(op, &mut output_queue);
                }
                operator_stack.push(token);
            }
            "(" => {
                operator_stack.push(token);
            }
            ")" => {
                while !operator_stack.is_empty() && operator_stack.last().unwrap() != &"(" {
                    let op = operator_stack.pop().unwrap();
                    apply_op(op, &mut output_queue);
                }
                if !operator_stack.is_empty() && operator_stack.last().unwrap() == &"(" {
                    operator_stack.pop();
                } else {
                    panic!("Mismatched parentheses");
                }
            }
            num if num.parse::<i64>().is_ok() => {
                output_queue.push(Expr::Number(num.parse().unwrap()));
            }
            var if var.trim().parse::<i64>().is_ok() => {
                output_queue.push(Expr::Number(var.trim().parse().unwrap()));
            }
            var => {
                output_queue.push(Expr::Variable(var.to_string()));
            }
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        if op == "(" {
            panic!("Mismatched parentheses");
        }
        apply_op(op, &mut output_queue);
    }

    if output_queue.len() != 1 {
        panic!("Invalid expression");
    }
    output_queue.pop().unwrap()
}

fn apply_op(op: &str, stack: &mut Vec<Expr>) {
    if stack.len() < 2 {
        panic!("Invalid expression");
    }
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    let result = match op {
        "+" => Expr::Add(Box::new(left), Box::new(right)),
        "-" => Expr::Sub(Box::new(left), Box::new(right)),
        "*" => Expr::Mul(Box::new(left), Box::new(right)),
        "/" => Expr::Div(Box::new(left), Box::new(right)),
        _ => panic!("Unknown operator: {}", op),
    };
    stack.push(result);
}

fn precedence(op: &str) -> usize {
    match op {
        "-" | "+" => 1,
        "/" | "*" => 2,
        _ => 0,
    }
}

fn evaluate_expr(expr: &Expr, vars: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Variable(name) => *vars.get(name).expect("Undefined Variable;"),
        Expr::Add(left, right) => evaluate_expr(left, vars) + evaluate_expr(right, vars),
        Expr::Sub(left, right) => evaluate_expr(left, vars) - evaluate_expr(right, vars),
        Expr::Mul(left, right) => evaluate_expr(left, vars) * evaluate_expr(right, vars),
        Expr::Div(left, right) => evaluate_expr(left, vars) / evaluate_expr(right, vars),
    }
}

fn main() {
    let js_code = r#"3 + (4 * 2) / (1 - 5)"#;
    let mut program = parse_program(js_code);

    println!("Parsed Program: {:?}", program);

    for stmt in &program.statements {
        match stmt {
            Stmt::Assignment(var, expr) => {
                let result = evaluate_expr(expr, &program.variables);
                program.variables.insert(var.clone(), result);
                println!("Assignment: {} = {}", var, result);
            }
            Stmt::Declaration(decl) => {
                println!(
                    "Declaration: {} ={}",
                    decl.name,
                    program.variables.get(&decl.name).unwrap()
                );
                let _ = decl.value;
            }
            Stmt::Expression(expr) => {
                let result = evaluate_expr(expr, &program.variables);
                println!("Expression: {}", result);
            }
        }
    }
}
