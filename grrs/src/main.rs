use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use structopt::StructOpt;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read    
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let f = File::open(&args.path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(&args.pattern) {
            println!("{}", line.trim_end());
        }
        // line.clear() clear the line read previously.
        line.clear();
    }

    Ok(())
}
