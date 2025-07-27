use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PythonType {
    Int,
    Float,
    String,
    Bool,
    List,
    Dict,
    Tuple,
}

impl fmt::Display for PythonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonType::Int => write!(f, "int"),
            PythonType::Float => write!(f, "float"),
            PythonType::String => write!(f, "str"),
            PythonType::Bool => write!(f, "bool"),
            PythonType::List => write!(f, "list"),
            PythonType::Dict => write!(f, "dict"),
            PythonType::Tuple => write!(f, "tuple"),
        }
    }
}

impl FromStr for PythonType {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(PythonType::Int),
            "float" => Ok(PythonType::Float),
            "str" => Ok(PythonType::String),
            "bool" => Ok(PythonType::Bool),
            "list" => Ok(PythonType::List),
            "dict" => Ok(PythonType::Dict),
            "tuple" => Ok(PythonType::Tuple),
            _ => {
                println!("Unknown type: {s}");
                Err(())
            }
        }
    }
}