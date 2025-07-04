use r_python::stdlib::io::builtin_open;
use r_python::ir::ast::Expression;
use r_python::interpreter::expression_eval::ExpressionResult;

#[test]
fn open_write_and_read() {
    let path = "test_file_io.txt".to_string();
    let content = "Hello, test!".to_string();
    let args_write = [
        Expression::CString(path.clone()),
        Expression::CString("w".to_string()),
        Expression::CString(content.clone()),
    ];
    let result_write = builtin_open(&args_write);
    assert!(result_write.is_ok());
    let args_read = [
        Expression::CString(path.clone()),
        Expression::CString("r".to_string()),
    ];
    let result_read = builtin_open(&args_read);
    assert!(matches!(result_read, Ok(ExpressionResult::Value(Expression::CString(ref s))) if s == &content));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn open_read_nonexistent_file() {
    let args = [
        Expression::CString("file_that_does_not_exist.txt".to_string()),
        Expression::CString("r".to_string()),
    ];
    let result = builtin_open(&args);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.contains("could not open"));
}

#[test]
fn open_write_and_read_empty_content() {
    let path = "test_file_empty.txt".to_string();
    let content = "".to_string();
    let args_write = [
        Expression::CString(path.clone()),
        Expression::CString("w".to_string()),
        Expression::CString(content.clone()),
    ];
    let result_write = builtin_open(&args_write);
    assert!(result_write.is_ok());
    let args_read = [
        Expression::CString(path.clone()),
        Expression::CString("r".to_string()),
    ];
    let result_read = builtin_open(&args_read);
    assert!(matches!(result_read, Ok(ExpressionResult::Value(Expression::CString(ref s))) if s == &content));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn open_write_missing_content_argument() {
    let args = [
        Expression::CString("file_should_fail.txt".to_string()),
        Expression::CString("w".to_string()),
    ];
    let result = builtin_open(&args);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.contains("third argument"));
}

#[test]
fn open_unsupported_mode() {
    let args = [
        Expression::CString("file.txt".to_string()),
        Expression::CString("a".to_string()),
    ];
    let result = builtin_open(&args);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.contains("unsupported mode"));
}

#[test]
fn open_first_argument_not_string() {
    let args = [
        Expression::CVoid,
        Expression::CString("r".to_string()),
    ];
    let result = builtin_open(&args);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.contains("first argument must be a string"));
}