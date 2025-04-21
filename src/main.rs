use clap::{arg, command, Parser};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(
    version,
    about = "CLI that goes though the file and subsequent \\input, \\include and packs all the content to the single output file. "
)]
struct Cli {
    #[arg(value_hint=clap::ValueHint::DirPath)]
    input_file: PathBuf,

    #[arg(value_hint=clap::ValueHint::DirPath)]
    output_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    println!(
        "Starting copying {} to the {}",
        &cli.input_file.display(),
        &cli.output_file.display()
    );
    pack_file(cli.input_file, cli.output_file.clone());
    println!(
        "Finished copying successfully to the {}",
        cli.output_file.display()
    )
}

fn pack_file(input_path: PathBuf, output_path: PathBuf) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .unwrap_or_else(|_| {
            panic!(
                "Couldn't open the file to write to: {}",
                &output_path.display()
            )
        });
    pack_file_aux(&input_path, &mut file);
}

// temp func to know if the line is commented
fn is_line_commented(line: &str) -> bool {
    line.trim_start().contains("%")
}

fn pack_file_aux(input_path: &PathBuf, output_file: &mut std::fs::File) {
    let mut contents: String = String::new();

    let mut r_file = OpenOptions::new()
        .read(true)
        .open(&input_path)
        .unwrap_or_else(|_| panic!("Couldn't open the file: {}", &input_path.display()));
    r_file.read_to_string(&mut contents);
    for line in contents.lines() {
        if is_line_commented(line) {
            output_file.write_fmt(format_args!("{}\n", line));
        } else if line.contains("\\input{") {
            let new_path = extract_path_from_input(line);
            pack_file_aux(&new_path, output_file);
        } else if line.contains("\\include{") {
            let new_path = extract_path_from_include(line);
            pack_file_aux(&new_path, output_file);
        } else {
            output_file.write_fmt(format_args!("{}\n", line));
        }
    }
}

fn extract_path_from_input(line: &str) -> PathBuf {
    if !line.contains("\\input{") {
        panic!("The line doesn't contain an input");
    }
    let start_idx = line.rfind("\\input{").unwrap() + "\\input{".len() - 1;

    let mut line = line.to_string();
    line.drain(0..=start_idx);
    let stop_idx = line.rfind("}").unwrap();
    line.drain(stop_idx..);
    PathBuf::from(line)
}
fn extract_path_from_include(line: &str) -> PathBuf {
    if !line.contains("\\include{") {
        panic!("The line doesn't contain an include");
    }
    let start_idx = line.rfind("\\include{").unwrap() + "\\include{".len() - 1;

    let mut line = line.to_string();
    line.drain(0..=start_idx);
    let stop_idx = line.rfind("}").unwrap();
    line.drain(stop_idx..);
    PathBuf::from(line)
}
