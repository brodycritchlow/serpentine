use typechecker_test::TypeChecker;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }
    
    if args[1] == "--help" || args[1] == "-h" {
        print_help(&args[0]);
        process::exit(0);
    }
    
    let filename = &args[1];
    
    if !std::path::Path::new(filename).exists() {
        eprintln!("Error: File '{}' not found", filename);
        process::exit(1);
    }
    
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };
    
    let verbose = args.iter().any(|arg| arg == "--verbose");
    let show_types_on_error = args.iter().any(|arg| arg == "--show-types-on-error");
    let including_implicit = args.iter().any(|arg| arg == "--including-implicit");
    
    let mut checker = if including_implicit {
        TypeChecker::new().with_implicit_checking()
    } else {
        TypeChecker::new()
    };
    
    match checker.analyze_source(&source) {
        Ok(_) => {
            println!("✅ Type check passed for '{}'", filename);
            
            if verbose {
                let vars = checker.get_all_variables();
                if !vars.is_empty() {
                    println!("\nVariable types:");
                    let mut sorted_vars: Vec<_> = vars.iter().collect();
                    sorted_vars.sort_by_key(|(name, _)| *name);
                    
                    for (name, ty) in sorted_vars {
                        println!("  {} : {:?}", name, ty);
                    }
                }
            }
            
            process::exit(0);
        }
        Err(e) => {
            eprintln!("❌ Type check failed for '{}'", filename);
            eprintln!("Error: {}", e);
            
            if show_types_on_error || verbose {
                let vars = checker.get_all_variables();
                if !vars.is_empty() {
                    eprintln!("\nVariables typed before error:");
                    let mut sorted_vars: Vec<_> = vars.iter().collect();
                    sorted_vars.sort_by_key(|(name, _)| *name);
                    
                    for (name, ty) in sorted_vars {
                        eprintln!("  {} : {:?}", name, ty);
                    }
                }
            }
            
            process::exit(1);
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <filename.py> [OPTIONS]", program_name);
    eprintln!("Try '{} --help' for more information.", program_name);
}

fn print_help(program_name: &str) {
    println!("checkrs - A Python type checker written in Rust");
    println!();
    println!("Usage: {} <filename.py> [OPTIONS]", program_name);
    println!();
    println!("Arguments:");
    println!("  <filename.py>    Python file to type check");
    println!();
    println!("Options:");
    println!("  --verbose             Show all variable types (on success or error)");
    println!("  --show-types-on-error Show typed variables when type check fails");
    println!("  --including-implicit  Check types for all variables (not just explicitly typed)");
    println!("  -h, --help            Show this help message");
    println!();
    println!("Examples:");
    println!("  {} script.py", program_name);
    println!("  {} script.py --verbose", program_name);
    println!("  {} script.py --including-implicit", program_name);
    println!();
    println!("Exit codes:");
    println!("  0 - Type check passed");
    println!("  1 - Type check failed or error occurred");
}