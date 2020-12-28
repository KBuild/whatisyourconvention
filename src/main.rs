use anyhow::Result;
use walkdir::WalkDir;
use std::env;

#[derive(Debug)]
enum MyCase {
    Camel,
    Pascal, 
    Kebab,
    Snake,
    Lower,
    Upper,
    Mixed,
    Others
}

fn case_finder(str: Option<&str>) -> MyCase {
    match str {
        Some(inner_data) => {
            if !inner_data.is_ascii() {
                return MyCase::Others
            }

            if inner_data.contains("-") || inner_data.contains("_") {
                if heck::KebabCase::to_kebab_case(inner_data).eq(inner_data) {
                    return MyCase::Kebab
                }

                if heck::SnakeCase::to_snake_case(inner_data).eq(inner_data) {
                    return MyCase::Snake
                }

                return MyCase::Mixed
            }

            if inner_data.to_ascii_uppercase().eq(inner_data) {
                return MyCase::Upper
            }
 
            if inner_data.to_ascii_lowercase().eq(inner_data) {
                return MyCase::Lower
            }

            let capitalized: String = heck::CamelCase::to_camel_case(inner_data);

            if capitalized.eq(inner_data) {
                return MyCase::Pascal
            }
            
            if (capitalized[..1].to_ascii_lowercase() + &capitalized[1..]).eq(inner_data) {
                return  MyCase::Camel
            }

            return MyCase::Others
        },
        None => {
            MyCase::Others
        }
    }
}

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Let's search on directory : {:?}",
        current_dir
    );

    for entry in WalkDir::new(current_dir).into_iter().filter_map(|e| e.ok()) {
        println!(
            "{} : {:?}",
            entry.path().display(),
            case_finder(
                // send filename without extensions
                entry.file_name().to_str().map(|s| s.split(".").nth(0)).unwrap()
            )
        );
    }

    Ok(())
}
