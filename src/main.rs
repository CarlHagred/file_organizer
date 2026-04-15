use std::env::args;
use std::fs::read_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = match args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("A path needs to be provided as the first argument");
            return Ok(());
        }
    };
    let path_content = read_dir(argument)?;

    for content in path_content {
        let entry = content.unwrap();
        let path = entry.path();
        println!("{}", path.display());
    }

    Ok(())
}

