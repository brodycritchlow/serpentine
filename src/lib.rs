pub mod python_type;

use std::collections::{HashMap, HashSet};
use python_parser::ast::*;
use std::str::FromStr;

pub use crate::python_type::PythonType;

pub type TypeCheckResult<T> = Result<T, TypeCheckError>;

#[derive(Debug)]
pub enum TypeCheckError {
    ParseError(String),
    UnsupportedAnnotation(String),
    UnknownType(String),
    TypeMismatch {
        variable: String,
        expected: PythonType,
        actual: PythonType,
        literal_value: Option<String>,
    },
}

impl std::fmt::Display for TypeCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeCheckError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            TypeCheckError::UnsupportedAnnotation(msg) => write!(f, "Unsupported annotation: {msg}"),
            TypeCheckError::UnknownType(msg) => write!(f, "Unknown type: {msg}"),
            TypeCheckError::TypeMismatch { variable: _, expected, actual, literal_value } => {
                if let Some(literal) = literal_value {
                    write!(f, "Type \"Literal[{literal}]\" is not assignable to declared type \"{expected}\"\n  \"Literal[{literal}]\" is not assignable to \"{expected}\"")
                } else {
                    write!(f, "Type \"{actual}\" is not assignable to declared type \"{expected}\"\n  \"{actual}\" is not assignable to \"{expected}\"")
                }
            },
        }
    }
}

impl std::error::Error for TypeCheckError {}

pub struct TypeChecker {
    assigned_variables: HashMap<String, PythonType>,
    explicitly_typed: HashSet<String>,
    including_implicit: bool,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            assigned_variables: HashMap::new(),
            explicitly_typed: HashSet::new(),
            including_implicit: false,
        }
    }
    
    pub fn with_implicit_checking(mut self) -> Self {
        self.including_implicit = true;
        self
    }

    pub fn analyze_source(&mut self, python_source: &str) -> TypeCheckResult<()> {
        let ast = python_parser::file_input(python_parser::make_strspan(python_source))
            .map_err(|e| TypeCheckError::ParseError(format!("{e:?}")))?
            .1;

        for node in ast {
            self.process_statement(node)?;
        }

        Ok(())
    }

    fn process_statement(&mut self, statement: Statement) -> TypeCheckResult<()> {
        match statement {
            Statement::TypedAssignment(target, annotation, value) => self.handle_typed_assignment(target, annotation, value)?,
            Statement::Assignment(lhs, rhs_list) => {
                self.handle_untyped_assignment(lhs, rhs_list)?
            },
            _ => {},
        }
        Ok(())
    }

    pub fn get_variable_type(&self, name: &str) -> Option<&PythonType> {
        self.assigned_variables.get(name)
    }

    pub fn get_all_variables(&self) -> &HashMap<String, PythonType> {
        &self.assigned_variables
    }
    
    pub fn handle_typed_assignment(
        &mut self,
        target: Vec<Expression>,
        annotation: Expression,
        values: Vec<Expression>,
    ) -> TypeCheckResult<()> {
        let expected_type = self.parse_type_annotation(&annotation)?;
        
        if let Some(value) = values.first()
            && let Some(actual_type) = self.infer_type_from_expression(value)
                && !self.is_assignable(&actual_type, &expected_type)
                    && let Some(Expression::Name(var_name)) = target.first() {
                        let literal_value = self.get_literal_value(value);
                        return Err(TypeCheckError::TypeMismatch {
                            variable: var_name.clone(),
                            expected: expected_type,
                            actual: actual_type,
                            literal_value,
                        });
                    }
        
        for expr in target {
            if let Expression::Name(name) = expr {
                self.assigned_variables.insert(name.clone(), expected_type.clone());
                self.explicitly_typed.insert(name);
            }
        }
        
        Ok(())
    }
    
    fn handle_untyped_assignment(
        &mut self,
        lhs: Vec<Expression>,
        rhs_list: Vec<Vec<Expression>>,
    ) -> TypeCheckResult<()> {
        let value = match rhs_list.last().and_then(|values| values.first()) {
            Some(v) => v,
            None => return Ok(()),
        };
        
        let inferred_type = match self.infer_type_from_expression(value) {
            Some(t) => t,
            None => return Ok(()),
        };
        
        for expr in lhs {
            let name = match expr {
                Expression::Name(n) => n,
                _ => continue,
            };
            
            if let Some(existing_type) = self.get_enforced_type(&name)
                && !self.is_assignable(&inferred_type, existing_type) {
                    return Err(TypeCheckError::TypeMismatch {
                        variable: name.clone(),
                        expected: existing_type.clone(),
                        actual: inferred_type.clone(),
                        literal_value: self.get_literal_value(value),
                    });
                }
            
            self.assigned_variables.insert(name, inferred_type.clone());
        }
        
        Ok(())
    }
    
    fn get_enforced_type(&self, name: &str) -> Option<&PythonType> {
        if self.explicitly_typed.contains(name) || self.including_implicit {
            self.assigned_variables.get(name)
        } else {
            None
        }
    }
    
    fn parse_type_annotation(&self, annotation: &Expression) -> TypeCheckResult<PythonType> {
        match annotation {
            Expression::Name(type_name) => {
                PythonType::from_str(type_name)
                    .map_err(|_| TypeCheckError::UnknownType(type_name.clone()))
            }
            _ => Err(TypeCheckError::UnsupportedAnnotation(format!("{annotation:?}")))
        }
    }
    
    fn infer_type_from_expression(&self, expr: &Expression) -> Option<PythonType> {
        match expr {
            Expression::Int(_) => Some(PythonType::Int),
            Expression::Float(_) => Some(PythonType::Float),
            Expression::String(_) => Some(PythonType::String),
            Expression::True | Expression::False => Some(PythonType::Bool),
            Expression::ListLiteral(_) => Some(PythonType::List),
            Expression::DictLiteral(_) => Some(PythonType::Dict),
            Expression::TupleLiteral(_) => Some(PythonType::Tuple),
            _ => None
        }
    }
    
    fn get_literal_value(&self, expr: &Expression) -> Option<String> {
        match expr {
            Expression::Int(n) => Some(format!("{n}")),
            Expression::String(pystrings) => {
                if !pystrings.is_empty() {
                    let content: String = pystrings.iter()
                        .map(|s| {
                            format!("{:?}", s.content).trim_matches('"').to_string()
                        })
                        .collect();
                    Some(format!("'{content}'"))
                } else {
                    Some("''".to_string())
                }
            },
            _ => None
        }
    }
    
    fn is_assignable(&self, actual: &PythonType, expected: &PythonType) -> bool {
        if actual == expected {
            return true;
        }
        
        matches!((actual, expected), (PythonType::Bool, PythonType::Int))
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
    