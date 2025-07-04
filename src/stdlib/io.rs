use crate::ir::ast::{Expression, Function, Type, FormalArgument};
use crate::interpreter::expression_eval::ExpressionResult;
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::File;
use std::io::Read;

pub fn register_builtins(map: &mut HashMap<String, Function>) {
    map.insert("input".to_string(), Function {
        name: "input".to_string(),
        kind: Type::TFunction(Box::new(Some(Type::TString)), vec![Type::TMaybe(Box::new(Type::TString))]),
        params: vec![FormalArgument::new("prompt".to_string(), Type::TMaybe(Box::new(Type::TString)))],
        body: None,
        builtin: Some(builtin_input),
    });
    map.insert("print".to_string(), Function {
        name: "print".to_string(),
        kind: Type::TFunction(Box::new(Some(Type::TVoid)), vec![Type::TAny]),
        params: vec![FormalArgument::new("value".to_string(), Type::TAny)],
        body: None,
        builtin: Some(builtin_print),
    });
    map.insert("open".to_string(), Function {
        name: "open".to_string(),
        kind: Type::TFunction(Box::new(Some(Type::TString)), vec![Type::TString, Type::TMaybe(Box::new(Type::TString))]),
        params: vec![
            FormalArgument::new("path".to_string(), Type::TString),
            FormalArgument::new("mode".to_string(), Type::TMaybe(Box::new(Type::TString)))
        ],
        body: None,
        builtin: Some(builtin_open),
    });

}

pub fn builtin_input(args: &[Expression]) -> Result<ExpressionResult, String> {
    let prompt = if let Some(Expression::CString(msg)) = args.get(0) {
        msg.as_str()
    } else {
        ""
    };
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    let input = input.trim_end_matches(['\n', '\r']).to_string();
    Ok(ExpressionResult::Value(Expression::CString(input)))
}

pub fn builtin_print(args: &[Expression]) -> Result<ExpressionResult, String> {
    let message = args.get(0).cloned().unwrap_or(Expression::CString("".to_string()));
    if let Expression::CString(s) = &message {
        println!("{}", s);
    } else {
        println!("{:?}", message);
    }
    Ok(ExpressionResult::Value(Expression::CVoid))
}

pub fn builtin_open(args: &[Expression]) -> Result<ExpressionResult, String> {
    let path = if let Some(Expression::CString(p)) = args.get(0) {
        p
    } else {
        return Err("open: first argument must be a string with the file path".to_string());
    };
    let mode = match args.get(1) {
        Some(Expression::CString(m)) => m.as_str(),
        _ => "r",
    };
    match mode {
        "r" => {
            let mut file = match File::open(path) {
                Ok(f) => f,
                Err(e) => return Err(format!("open: could not open '{}' for reading: {}", path, e)),
            };
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                return Err(format!("open: error reading '{}': {}", path, e));
            }
            Ok(ExpressionResult::Value(Expression::CString(contents)))
        }
        "w" => {
            let content = if let Some(Expression::CString(c)) = args.get(2) {
                c
            } else {
                return Err("open: when using mode 'w', a third argument with the content to write is required".to_string());
            };
            match std::fs::write(path, content) {
                Ok(_) => Ok(ExpressionResult::Value(Expression::CVoid)),
                Err(e) => Err(format!("open: could not write to '{}': {}", path, e)),
            }
        }
        m => Err(format!("open: unsupported mode '{}'.", m)),
    }
}
