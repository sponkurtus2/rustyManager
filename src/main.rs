use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, json, to_writer, to_writer_pretty, Value};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;
use tabled::settings::object::Rows;
use tabled::settings::themes::Colorization;
use tabled::{
    settings::{
        style::{BorderColor, Style},
        Color,
    },
    Table, Tabled,
};

#[derive(Parser)]
#[command(name = "rustyManager")]
#[command(author = "Sponkurtus2")]
#[command(version = "1.0")]
#[command(about = "A simple cli to keep your passwords secure 🛡️", long_about = None)]
#[command(help_template = "\
{about-section}
{usage-heading} {usage}

{all-args}{after-help}

EXAMPLES:
    {bin} --print-dataa -p          # Print all stored passwords
    {bin} --new-pass -p PASS -c CONFIRM  # Add new password
    {bin} --delete-pass PASS        # Delete a password
    {bin} --find-pass PASS         # Find a specific password
")]
struct Cli {
    #[arg(short, long, help = "Display all current passwords")]
    print_data: bool,

    #[arg(short, long, help = "Add a new password")]
    new_pass: bool,

    #[arg(short, long, help = "Delete a password")]
    delete_pass: Option<String>,

    #[arg(short, long, help = "Find a password")]
    find_pass: Option<String>,

    #[arg(requires = "new_pass")]
    pass: Option<String>,

    #[arg(requires = "new_pass")]
    pass_c: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Tabled, Clone)]
struct MyData {
    pass: String,
    pass_c: String,
}

fn create_json_file(name: String) -> File {
    let current_dir: &Path = Path::new(".");
    let file: PathBuf = current_dir.join("data.json");

    if file.exists() {
        let json_result: Result<File, std::io::Error> = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .append(true)
            .open("data.json");
        let json_file = match json_result {
            Ok(file) => file,
            Err(error) => panic!("Error: {}", error),
        };
        json_file
    } else {
        let json_file: File = File::create(&name).unwrap();
        json_file
    }
}

fn save_to_json(pass: String, pass_c: String) -> std::io::Result<()> {
    let data: Value = json!({"pass": pass, "pass_c": pass_c});

    let m: MyData = serde_json::from_value(data)?;

    let file_result: File = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")?;

    // let file: File = match file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Error opening file: {}", error),
    // };

    let reader: BufReader<&File> = BufReader::new(&file_result);
    let mut passwords: Vec<MyData> = match from_reader(reader) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    passwords.push(m);

    let file_result: File = File::create("data.json")?;

    to_writer_pretty(file_result, &passwords)?;

    Ok(())
}

fn read_json() -> std::io::Result<()> {
    let file_result: File = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")?;

    let reader: BufReader<&File> = BufReader::new(&file_result);
    let passwords: Vec<MyData> = match from_reader(reader) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    let green_data = Color::new("\u{1b}[48;2;133;159;61m", "\u{1b}[49m");

    let color_head_text: Color = Color::BG_GREEN | Color::FG_BLACK;
    let color_col1 = green_data | Color::FG_BLACK;

    let mut table: Table = Table::new(&passwords);
    table
        .with(Style::rounded())
        .with(BorderColor::new().bottom(Color::FG_GREEN))
        .with(BorderColor::new().top(Color::FG_GREEN))
        .with(BorderColor::new().left(Color::FG_GREEN))
        .with(BorderColor::new().corner_bottom_left(Color::FG_GREEN))
        .with(BorderColor::new().corner_bottom_right(Color::FG_GREEN))
        .with(BorderColor::new().corner_top_left(Color::FG_GREEN))
        .with(BorderColor::new().corner_top_right(Color::FG_GREEN))
        .with(BorderColor::new().right(Color::FG_GREEN))
        .with(Colorization::columns([color_col1]))
        .with(Colorization::exact([color_head_text], Rows::first()));

    println!("{}", table);

    Ok(())
}

fn delete_pass_json(pass_to_delete: &str) -> std::io::Result<()> {
    let file_result: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")?;

    let reader: BufReader<&File> = BufReader::new(&file_result);
    let mut passwords: Vec<MyData> = match from_reader(reader) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    let initial_len: usize = passwords.len();
    passwords.retain(|entry| entry.pass != pass_to_delete);

    let file_result: File = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")?;

    let writer: BufWriter<File> = BufWriter::new(file_result);
    to_writer(writer, &passwords)?;

    if passwords.len() < initial_len {
        println!("✅ Deleted password for : '{}'", pass_to_delete);
    } else {
        println!("❌ Couldn't find any password for:  '{}'", pass_to_delete);
    }
    Ok(())
}

fn find_pass_json(pass_to_find: &str) -> std::io::Result<()> {
    let file: File = OpenOptions::new().read(true).open("data.json")?;

    let reader: BufReader<&File> = BufReader::new(&file);
    let passwords: Vec<MyData> = match from_reader(reader) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    let mut passwords_iter: IntoIter<MyData> = passwords.into_iter();
    let password_to_find = passwords_iter.find(|d| d.pass == pass_to_find);

    let green_data = Color::new("\u{1b}[48;2;133;159;61m", "\u{1b}[49m");

    let color_head_text: Color = Color::BG_GREEN | Color::FG_BLACK;
    let color_col1 = green_data | Color::FG_BLACK;

    let mut table: Table = Table::new(password_to_find); 
    table
        .with(Style::rounded())
        .with(BorderColor::new().bottom(Color::FG_GREEN))
        .with(BorderColor::new().top(Color::FG_GREEN))
        .with(BorderColor::new().left(Color::FG_GREEN))
        .with(BorderColor::new().corner_bottom_left(Color::FG_GREEN))
        .with(BorderColor::new().corner_bottom_right(Color::FG_GREEN))
        .with(BorderColor::new().corner_top_left(Color::FG_GREEN))
        .with(BorderColor::new().corner_top_right(Color::FG_GREEN))
        .with(BorderColor::new().right(Color::FG_GREEN))
        .with(Colorization::columns([color_col1]))
        .with(Colorization::exact([color_head_text], Rows::first()));


    println!("{}", table);

    Ok(())
}

fn main() -> std::io::Result<()> {
    create_json_file("data.json".to_string());

    let cli: Cli = Cli::parse();

    if cli.print_data {
        let _ = read_json();
    } else if cli.new_pass {
        match (cli.pass, cli.pass_c) {
            (Some(pass), Some(pass_c)) => {
                save_to_json(pass, pass_c)?;
            }
            _ => {
                println!("❌ Error: When adding a password, you must provide name, username and password");
                println!("Example: rustyManager -a \"MyApp\" \"myuser\" \"mypassword\"");
            }
        }
    } else if let Some(delete_pass) = cli.delete_pass {
        delete_pass_json(&delete_pass)?;
    } else if let Some(find_pass) = cli.find_pass {
        find_pass_json(&find_pass)?;
    } else {
        println!("No action specified. Use --help to see available commands.");
    }

    Ok(())
}
