use serpentine::{PythonType, TypeCheckError, TypeChecker};

#[test]
fn test_basic_type_annotation() {
    let mut checker = TypeChecker::new();
    let code = "x: int = 42";
    assert!(checker.analyze_source(code).is_ok());
    assert_eq!(checker.get_variable_type("x"), Some(&PythonType::Int));
}

#[test]
fn test_type_inference() {
    let mut checker = TypeChecker::new();
    let code = "x = 42\ny = 'hello'\nz = 3.14";
    assert!(checker.analyze_source(code).is_ok());
    assert_eq!(checker.get_variable_type("x"), Some(&PythonType::Int));
    assert_eq!(checker.get_variable_type("y"), Some(&PythonType::String));
    assert_eq!(checker.get_variable_type("z"), Some(&PythonType::Float));
}

#[test]
fn test_type_mismatch_error() {
    let mut checker = TypeChecker::new();
    let code = "x: int = 42\nx = 'hello'";
    let result = checker.analyze_source(code);
    assert!(result.is_err());

    match result {
        Err(TypeCheckError::TypeMismatch {
            expected, actual, ..
        }) => {
            assert_eq!(expected, PythonType::Int);
            assert_eq!(actual, PythonType::String);
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_bool_to_int_assignment() {
    let mut checker = TypeChecker::new();
    let code = "x: int = True";
    assert!(checker.analyze_source(code).is_ok());
    assert_eq!(checker.get_variable_type("x"), Some(&PythonType::Int));
}

#[test]
fn test_int_to_bool_assignment_fails() {
    let mut checker = TypeChecker::new();
    let code = "x: bool = 42";
    assert!(checker.analyze_source(code).is_err());
}

#[test]
fn test_implicit_type_changes_allowed() {
    let mut checker = TypeChecker::new();
    let code = "x = 42\nx = 'hello'";
    assert!(checker.analyze_source(code).is_ok());
    assert_eq!(checker.get_variable_type("x"), Some(&PythonType::String));
}

#[test]
fn test_implicit_type_changes_with_flag() {
    let mut checker = TypeChecker::new().with_implicit_checking();
    let code = "x = 42\nx = 'hello'";
    assert!(checker.analyze_source(code).is_err());
}

#[test]
fn test_literal_value_in_error() {
    let mut checker = TypeChecker::new();
    let code = "x: int = 'hello'";

    match checker.analyze_source(code) {
        Err(TypeCheckError::TypeMismatch { literal_value, .. }) => {
            assert_eq!(literal_value, Some("'hello'".to_string()));
        }
        _ => panic!("Expected TypeMismatch error with literal value"),
    }
}

#[test]
fn test_all_basic_types() {
    let mut checker = TypeChecker::new();
    let code = r#"
a: int = 1
b: float = 3.14
c: str = "test"
d: bool = True
e: list = []
f: dict = {}
g: tuple = ()
"#;
    assert!(checker.analyze_source(code).is_ok());
    assert_eq!(checker.get_variable_type("a"), Some(&PythonType::Int));
    assert_eq!(checker.get_variable_type("b"), Some(&PythonType::Float));
    assert_eq!(checker.get_variable_type("c"), Some(&PythonType::String));
    assert_eq!(checker.get_variable_type("d"), Some(&PythonType::Bool));
    assert_eq!(checker.get_variable_type("e"), Some(&PythonType::List));
    assert_eq!(checker.get_variable_type("f"), Some(&PythonType::Dict));
    assert_eq!(checker.get_variable_type("g"), Some(&PythonType::Tuple));
}

#[test]
fn test_unknown_type_error() {
    let mut checker = TypeChecker::new();
    let code = "x: UnknownType = 42";

    match checker.analyze_source(code) {
        Err(TypeCheckError::UnknownType(type_name)) => {
            assert_eq!(type_name, "UnknownType");
        }
        _ => panic!("Expected UnknownType error"),
    }
}

#[test]
fn test_multiple_assignments() {
    let mut checker = TypeChecker::new();
    let code = "x: int = 1\ny: int = 2\nx = 3\ny = 4";
    assert!(checker.analyze_source(code).is_ok());
    assert_eq!(checker.get_variable_type("x"), Some(&PythonType::Int));
    assert_eq!(checker.get_variable_type("y"), Some(&PythonType::Int));
}

#[test]
fn test_literal_integers() {
    let mut checker = TypeChecker::new();
    let code = "x: str = 'test'\nx = 42";

    match checker.analyze_source(code) {
        Err(TypeCheckError::TypeMismatch { literal_value, .. }) => {
            assert_eq!(literal_value, Some("42".to_string()));
        }
        _ => panic!("Expected TypeMismatch error with literal value"),
    }
}

#[test]
fn test_empty_source() {
    let mut checker = TypeChecker::new();
    assert!(checker.analyze_source("").is_ok());
}

#[test]
fn test_parse_error() {
    let mut checker = TypeChecker::new();
    let code = "x: = 42";

    match checker.analyze_source(code) {
        Err(TypeCheckError::ParseError(_)) => {
            // Expected
        }
        _ => panic!("Expected ParseError"),
    }
}
