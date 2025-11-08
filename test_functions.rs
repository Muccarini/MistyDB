// Test file for function definitions and calls

use misty_db::interpreter::Interpreter;

#[test]
fn test_simple_function_call() {
    let source = r#"
        func add(a, b) {
            a + b
        }
        add(5, 3)
    "#.to_string();

    // Should execute without panic
    let result = Interpreter::execute_full_pipeline(source);
    assert!(result.is_ok());
}

#[test]
fn test_function_with_multiple_statements() {
    let source = r#"
        func calculate(x, y) {
            x + y;
            x * y
        }
        calculate(4, 7)
    "#.to_string();

    let result = Interpreter::execute_full_pipeline(source);
    assert!(result.is_ok());
}

#[test]
fn test_nested_function_calls() {
    let source = r#"
        func multiply(a, b) {
            a * b
        }
        func square(n) {
            multiply(n, n)
        }
        square(5)
    "#.to_string();

    let result = Interpreter::execute_full_pipeline(source);
    assert!(result.is_ok());
}

#[test]
fn test_function_with_comparison() {
    let source = r#"
        func isGreater(a, b) {
            a > b
        }
        isGreater(10, 5)
    "#.to_string();

    let result = Interpreter::execute_full_pipeline(source);
    assert!(result.is_ok());
}

#[test]
fn test_function_wrong_arg_count() {
    let source = r#"
        func add(a, b) {
            a + b
        }
        add(5)
    "#.to_string();

    let result = Interpreter::execute_full_pipeline(source);
    // Should fail because wrong number of arguments
    assert!(result.is_err());
}

#[test]
fn test_undefined_function() {
    let source = r#"
        notDefined(5, 3)
    "#.to_string();

    let result = Interpreter::execute_full_pipeline(source);
    // Should fail because function is not defined
    assert!(result.is_err());
}

#[test]
fn test_arithmetic_in_function() {
    let source = r#"
        func complex(a, b, c) {
            (a + b) * c
        }
        complex(2, 3, 4)
    "#.to_string();

    let result = Interpreter::execute_full_pipeline(source);
    assert!(result.is_ok());
}
