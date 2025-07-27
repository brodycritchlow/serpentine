use typechecker_test::{TypeChecker, PythonType, TypeCheckError};
use std::fs;

#[test]
fn test_parse_python_script() {
    let python_source = fs::read_to_string("test_script.py")
        .expect("Failed to read test_script.py");
    
    let mut type_checker = TypeChecker::new();
    type_checker.analyze_source(&python_source)
        .expect("Failed to analyze Python source");
    
    let variables = type_checker.get_all_variables();
    
    assert_eq!(variables.get("a"), Some(&PythonType::Int));
    assert_eq!(variables.get("b"), Some(&PythonType::String));
    assert_eq!(variables.get("c"), Some(&PythonType::Float));
    assert_eq!(variables.get("d"), Some(&PythonType::Bool));
    assert_eq!(variables.get("e"), Some(&PythonType::List));
    assert_eq!(variables.get("f"), Some(&PythonType::Dict));
    assert_eq!(variables.get("g"), Some(&PythonType::Tuple));
    
    assert_eq!(variables.len(), 7, "Expected exactly 7 variables");
}

#[test]
fn test_individual_type_checks() {
    let test_cases = vec![
        ("x: int = 42", "x", PythonType::Int),
        ("y: str = 'hello'", "y", PythonType::String),
        ("z: float = 3.14", "z", PythonType::Float),
        ("w: bool = True", "w", PythonType::Bool),
        ("v: list = []", "v", PythonType::List),
        ("u: dict = {}", "u", PythonType::Dict),
        ("t: tuple = ()", "t", PythonType::Tuple),
    ];
    
    for (code, var_name, expected_type) in test_cases {
        let mut type_checker = TypeChecker::new();
        type_checker.analyze_source(code).expect(&format!("Failed to analyze: {}", code));
        assert_eq!(
            type_checker.get_variable_type(var_name), 
            Some(&expected_type),
            "Failed for code: {}", 
            code
        );
    }
}

#[test]
fn test_unknown_type_error() {
    let mut type_checker = TypeChecker::new();
    let result = type_checker.analyze_source("x: UnknownType = 42");
    
    assert!(result.is_err());
    match result {
        Err(e) => assert!(format!("{}", e).contains("Unknown type: UnknownType")),
        Ok(_) => panic!("Expected error for unknown type"),
    }
}

#[test]
fn test_multiple_assignments() {
    let mut type_checker = TypeChecker::new();
    let code = r#"
a: int = 1
b: str = "hello"
c: float = 3.14
d: bool = True
"#;
    
    type_checker.analyze_source(code).expect("Failed to analyze multiple assignments");
    
    assert_eq!(type_checker.get_variable_type("a"), Some(&PythonType::Int));
    assert_eq!(type_checker.get_variable_type("b"), Some(&PythonType::String));
    assert_eq!(type_checker.get_variable_type("c"), Some(&PythonType::Float));
    assert_eq!(type_checker.get_variable_type("d"), Some(&PythonType::Bool));
}

#[test]
fn test_unsupported_annotation() {
    let mut type_checker = TypeChecker::new();
    // Using a complex annotation that's not just a name
    let result = type_checker.analyze_source("x: List[int] = [1, 2, 3]");
    
    assert!(result.is_err());
    match result {
        Err(e) => assert!(format!("{}", e).contains("Unsupported annotation")),
        Ok(_) => panic!("Expected error for complex annotation"),
    }
}

#[test]
fn test_type_inference() {
    let test_cases = vec![
        ("x = 42", "x", PythonType::Int),
        ("y = 3.14", "y", PythonType::Float),
        ("z = 'hello'", "z", PythonType::String),
        ("w = True", "w", PythonType::Bool),
        ("v = False", "v", PythonType::Bool),
        ("a = [1, 2, 3]", "a", PythonType::List),
        ("b = {'key': 'value'}", "b", PythonType::Dict),
        ("c = (1, 2, 3)", "c", PythonType::Tuple),
    ];
    
    for (code, var_name, expected_type) in test_cases {
        let mut type_checker = TypeChecker::new();
        type_checker.analyze_source(code).expect(&format!("Failed to analyze: {}", code));
        assert_eq!(
            type_checker.get_variable_type(var_name), 
            Some(&expected_type),
            "Failed type inference for code: {}", 
            code
        );
    }
}

#[test]
fn test_mixed_typed_and_inferred() {
    let mut type_checker = TypeChecker::new();
    let code = r#"
a: int = 1
b = "hello"
c: float = 3.14
d = True
e = [1, 2, 3]
"#;
    
    type_checker.analyze_source(code).expect("Failed to analyze mixed assignments");
    
    assert_eq!(type_checker.get_variable_type("a"), Some(&PythonType::Int));
    assert_eq!(type_checker.get_variable_type("b"), Some(&PythonType::String));
    assert_eq!(type_checker.get_variable_type("c"), Some(&PythonType::Float));
    assert_eq!(type_checker.get_variable_type("d"), Some(&PythonType::Bool));
    assert_eq!(type_checker.get_variable_type("e"), Some(&PythonType::List));
}

#[test]
fn test_type_mismatch_typed_assignment() {
    let mut type_checker = TypeChecker::new();
    let code = "x: int = 'hello'";
    
    let result = type_checker.analyze_source(code);
    assert!(result.is_err());
    
    match result {
        Err(TypeCheckError::TypeMismatch { variable, expected, actual, .. }) => {
            assert_eq!(variable, "x");
            assert_eq!(expected, PythonType::Int);
            assert_eq!(actual, PythonType::String);
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_inferred_variable_can_change_type() {
    let mut type_checker = TypeChecker::new();
    let code = r#"
x = 42
x = "hello"
x = 3.14
x = True
"#;
    
    // This should now pass since inferred variables can change type
    type_checker.analyze_source(code).expect("Inferred variables should be able to change type");
    
    // The final type should be Bool
    assert_eq!(type_checker.get_variable_type("x"), Some(&PythonType::Bool));
}

#[test]
fn test_type_mismatch_reassignment_typed() {
    let mut type_checker = TypeChecker::new();
    let code = r#"
x: int = 42
x = "hello"
"#;
    
    let result = type_checker.analyze_source(code);
    assert!(result.is_err());
    
    match result {
        Err(TypeCheckError::TypeMismatch { variable, expected, actual, .. }) => {
            assert_eq!(variable, "x");
            assert_eq!(expected, PythonType::Int);
            assert_eq!(actual, PythonType::String);
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_valid_reassignment_same_type() {
    let mut type_checker = TypeChecker::new();
    let code = r#"
x = 42
x = 100
y: str = "hello"
y = "world"
"#;
    
    type_checker.analyze_source(code).expect("Should allow reassignment with same type");
    assert_eq!(type_checker.get_variable_type("x"), Some(&PythonType::Int));
    assert_eq!(type_checker.get_variable_type("y"), Some(&PythonType::String));
}

#[test]
fn test_mixed_inferred_and_typed_reassignments() {
    let mut type_checker = TypeChecker::new();
    let code = r#"
# Inferred variable can change type
a = 42
a = "hello"

# Explicitly typed variable cannot change type
b: int = 42
b = 100  # OK - same type
"#;
    
    type_checker.analyze_source(code).expect("Should allow inferred variables to change type");
    assert_eq!(type_checker.get_variable_type("a"), Some(&PythonType::String));
    assert_eq!(type_checker.get_variable_type("b"), Some(&PythonType::Int));
}

#[test]
fn test_multiple_type_errors() {
    let mut type_checker = TypeChecker::new();
    let code = r#"
x: int = 42
y: str = "hello"
x = "wrong"
"#;
    
    let result = type_checker.analyze_source(code);
    assert!(result.is_err());
    
    // Should fail on the first error (x = "wrong")
    match result {
        Err(TypeCheckError::TypeMismatch { variable, .. }) => {
            assert_eq!(variable, "x");
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_type_mismatch_with_various_types() {
    let test_cases = vec![
        ("x: int = 3.14", "x", PythonType::Int, PythonType::Float),
        ("x: str = 42", "x", PythonType::String, PythonType::Int),
        ("x: bool = 'true'", "x", PythonType::Bool, PythonType::String),
        ("x: list = {}", "x", PythonType::List, PythonType::Dict),
        ("x: dict = []", "x", PythonType::Dict, PythonType::List),
        ("x: tuple = [1, 2]", "x", PythonType::Tuple, PythonType::List),
    ];
    
    for (code, var_name, expected_type, actual_type) in test_cases {
        let mut type_checker = TypeChecker::new();
        let result = type_checker.analyze_source(code);
        
        assert!(result.is_err(), "Expected error for code: {}", code);
        match result {
            Err(TypeCheckError::TypeMismatch { variable, expected, actual, .. }) => {
                assert_eq!(variable, var_name);
                assert_eq!(expected, expected_type);
                assert_eq!(actual, actual_type);
            }
            _ => panic!("Expected TypeMismatch error for code: {}", code),
        }
    }
}

#[test]
fn test_including_implicit_flag() {
    // Test that with the flag, inferred variables cannot change type
    let code = r#"
x = 42
x = "hello"
"#;
    
    // Without flag - should pass
    let mut checker = TypeChecker::new();
    checker.analyze_source(code).expect("Should pass without including_implicit");
    
    // With flag - should fail
    let mut checker_strict = TypeChecker::new().with_implicit_checking();
    let result = checker_strict.analyze_source(code);
    assert!(result.is_err());
    
    match result {
        Err(TypeCheckError::TypeMismatch { variable, expected, actual, .. }) => {
            assert_eq!(variable, "x");
            assert_eq!(expected, PythonType::Int);
            assert_eq!(actual, PythonType::String);
        }
        _ => panic!("Expected TypeMismatch error with including_implicit flag"),
    }
}

#[test]
fn test_including_implicit_with_mixed_variables() {
    let code = r#"
# Inferred variable
a = 42
a = "hello"

# Explicitly typed variable
b: int = 42
b = "world"
"#;
    
    // With including_implicit flag, should fail on first reassignment (a = "hello")
    let mut checker = TypeChecker::new().with_implicit_checking();
    let result = checker.analyze_source(code);
    assert!(result.is_err());
    
    match result {
        Err(TypeCheckError::TypeMismatch { variable, .. }) => {
            assert_eq!(variable, "a", "Should fail on inferred variable reassignment first");
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}