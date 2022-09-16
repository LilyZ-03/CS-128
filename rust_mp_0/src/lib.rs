use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo
}

impl Operation {
    // [COMPLETE THIS FUNCTION]
    pub fn from_char(symbol: char) -> Option<Operation> {
        match symbol {
            '+' => Some(Operation::Add),
            '-' => Some(Operation::Subtract),
            '*' | 'x' => Some(Operation::Multiply),
            '/' => Some(Operation::Divide),
            '%' => Some(Operation::Modulo),
            _ => None
        }
    }
}

// [HELPER FUNCTION - DO NOT EDIT]
pub fn get_equation_tuple(line: &String) -> (Option<&str>, Option<&str>) {
    let v: Vec<&str> = line.split(&['-', '+', 'x', '*', '/', '%'][..]).collect();
    let x: Option<&str> = match v.get(0) {
        Some(x) => Some(x),
        None => None
    };
    let y: Option<&str> = match v.get(1) {
        Some(y) => Some(y),
        None => None
    };
    (x, y)
}

// [COMPLETE THIS HELPER FUNCTION]
pub fn parse_equation(line: &String) -> Result<(f32, f32, Operation), ()> {
    let nums = get_equation_tuple(line);
    if nums.0.is_none() || nums.1.is_none() 
        || String::from(nums.0.unwrap()).len() < 1 
        || String::from(nums.1.unwrap()).len() < 1 {
        return Err(())
    }
    /* let v: Vec<&str> = line.split(&['-', '+', 'x', '*', '/', '%'][..]).collect();
    for sec in v {
        match sec
    }*/
    let ch: Operation;
    if line.contains("+") {
        ch = Operation::from_char('+').unwrap();
    } else if line.contains("-") { 
        ch = Operation::from_char('-').unwrap();
    } else if line.contains("*") || line.contains("x") {
        ch = Operation::from_char('*').unwrap();
    } else if line.contains("/") {
        ch = Operation::from_char('/').unwrap();
    } else if line.contains("%") {
        ch = Operation::from_char('%').unwrap();
    } else {
        return Err(())
    }
    let x: f32 = String::from(nums.0.unwrap()).trim().parse::<f32>().unwrap();
    let y: f32 = String::from(nums.1.unwrap()).trim().parse::<f32>().unwrap();
    return Ok((x, y, ch))
}

// [COMPLETE THIS FUNCTION]
pub fn solve(equation: &String) -> Result<f32, ()> {
    if parse_equation(equation).is_err() {
        return Err(());
    }
    let dat: (f32, f32, Operation) = parse_equation(equation).unwrap();
    let x: f32 = dat.0;
    let y: f32 = dat.1; 
    match dat.2 {
        Operation::Add => Ok(x + y),
        Operation::Subtract => Ok(x - y),
        Operation::Multiply => Ok(x * y),
        Operation::Divide => Ok(x / y),
        Operation::Modulo => Ok(x % y),
    }
}

// [COMPLETE THIS FUNCTION]
pub fn solve_all(file_path: &str) -> Result<f32, ()> {
    let mut total: f32 = 0.0;
    let file = match File::open(file_path.clone()) {
        Ok(f) => f, 
        _ => return Err(())
    };
    let contents = BufReader::new(file);
    for line in contents.lines() {
        let mut r: Result<f32, ()> = Err(());
        match line {
            Ok(line_dat) => r = solve(&line_dat), 
            _ => ()
        };
        if !r.is_err() {
            total += r.unwrap();
        }
    }
    return Ok(total);
}

// You can test your code with the test cases we've provided by running `cargo test`
// We also encourage you to add more assert statements to test out your code further!
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_single_number() {
        for i in -20..20 {
            assert!(solve(&i.to_string()).is_err());
        }
    }

    #[test]
    fn test_solve_two_random_numbers() {
        let mut rng = rand::thread_rng();

        for _ in 0..200 {
            let (xi, yi): (i16, i16) = rng.gen();
            let (x, y): (f32, f32) = (xi.abs() as f32, yi.abs() as f32); // avoid int overflow by upcasting to int with more space

            assert_eq!(solve(&format!("{}+{}", 1.0, 2.0)).unwrap(), 1.0 + 2.0);
            assert_eq!(solve(&format!("{}-{}", 4.0, 5.0)).unwrap(), 4.0 - 5.0);
            assert_eq!(solve(&format!("{}x{}", x, y)).unwrap(), x * y);
            assert_eq!(solve(&format!("{}*{}", x, y)).unwrap(), x * y);
            assert_eq!(solve(&format!("{}%{}", x, y)).unwrap(), x % y);
            assert_eq!(solve(&format!("{}/{}", x, y)).unwrap(), x / y);
        }
    }
}