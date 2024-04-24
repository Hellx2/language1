#[forbid(clippy::style, clippy::needless_collect)]
pub mod expr;

use expr::*;
use std::io::{stdin, stdout, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lines = vec![];
    let mut vars = vec![];
    if args.len() < 2 {
        loop {
            print!(" >>> ");
            stdout().flush().unwrap();
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            lines.push(line);
            exec(&lines, &mut vars);
        }
    } else {
        for i in args[1..].iter() {
            if i.starts_with('-') {}
        }
    }
}

fn exec(lines: &[String], vars: &mut Vec<(String, Value)>) {
    let line = lines.last().unwrap().trim();
    //dbg!(line);
    if line.starts_with("exit") {
        std::process::exit(0);
    }
    // TODO: Remove previous value upon redefinition
    if line.starts_with("let ") {
        let statement = line
            .split_once("let ")
            .unwrap()
            .1
            .trim()
            .split_once('=')
            .unwrap_or(("", ""));
        exec0(statement, vars);
        /*let name = statement.0.trim().to_string();
        let def = statement.1.trim().to_string();
        let x: Vec<&str> = name.split(",").collect();
        if x.len() > 1 {
            let y: Vec<&str> = def.split(",").collect();
            if y.len() == 1 {
                for i in x {
                    vars.push((i.trim().to_string(), Value::from(def.clone())));
                    println!("{} = {:?}", vars.last().unwrap().0, vars.last().unwrap().1);
                }
            } else if y.len() == x.len() {
                for i in 0..y.len() {
                    vars.push((x[i].trim().to_string(), Value::from(y[i].trim().to_string())));
                    println!("{} = {:?}", vars.last().unwrap().0, vars.last().unwrap().1);
                }
            }
        } else {
            vars.push((name, Value::from(def.clone())));
            println!("{} = {:?}", vars.last().unwrap().0, vars.last().unwrap().1);
        }*/
    } else if line.contains('=') {
        let statement = line.split_once('=').unwrap_or(("", ""));
        exec0(statement, vars);
    } else {
        let f: Vec<&(String, Value)> = vars.iter().filter(|x| x.0 == line.trim()).collect();

        if !f.is_empty() {
            println!("{:?}", f.last().unwrap().1)
        }
    }
}

fn exec0(statement: (&str, &str), vars: &mut Vec<(String, Value)>) {
    let name = statement.0.trim().to_string();
    let def = statement.1.trim().to_string();
    let x: Vec<&str> = name.split(',').collect();
    if x.len() > 1 {
        let y: Vec<&str> = def.split(',').collect();
        if y.len() == 1 {
            for i in x {
                vars.push((i.trim().to_string(), Value::from(def.clone())));
                println!("{} = {:?}", vars.last().unwrap().0, vars.last().unwrap().1);
            }
        } else if y.len() == x.len() {
            for i in 0..y.len() {
                vars.push((
                    x[i].trim().to_string(),
                    Value::from(y[i].trim().to_string()),
                ));
                println!("{} = {:?}", vars.last().unwrap().0, vars.last().unwrap().1);
            }
        }
    } else {
        vars.push((name, Value::from(def.clone())));
        println!("{} = {:?}", vars.last().unwrap().0, vars.last().unwrap().1);
    }
}
