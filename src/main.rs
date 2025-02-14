use indicatif::{ProgressBar, ProgressStyle};
use rand::{rngs::OsRng, RngCore};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn update_progress(progress: i64, maxsize: i64) {
    let progress_percentage = progress as f64 / maxsize as f64;
    let bar_width = 20;

    let num_hashes = (progress_percentage * bar_width as f64).round() as usize;
    let num_dashes = bar_width - num_hashes;

    let progress_bar = format!("{}{}", "#".repeat(num_hashes), "-".repeat(num_dashes));

    println!(
        "{:x} of {:x} complete. [{}]",
        progress, maxsize, progress_bar
    );
    print!("\x1B[2A");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!(
        r#"
mlXOR - File XOR obfuscator/deobfuscator
Copyright (c) 2025 Mai Lawton
This program is licensed under the MIT License
"#
    );

    // check if program started with --genxorpad argument. alternatively with -g
    if args.len() == 4 && args[1] == "--genxorpad" || args.len() == 4 && args[1] == "-g" {
        let size: usize = args[2]
            .parse()
            .expect("Size invalid! Size needs to be a valid number.");
        let filename = &args[3];
        generate_xorpad(size, filename)?;
        println!("Generated XOR pad of size {} bytes at: {}", size, filename);
        return Ok(());
    }

    // check if it has enough arguments
    // if it does itll continue if not it warns the user and exits
    if args.len() != 3 {
        eprintln!("Insufficient arguments! Usage:");
        eprintln!(
            "  To XOR a file: mlxor <filePath> <xorFilePath>\n  To generate XOR pad: --genxorpad <size> <filename>"
        );
        std::process::exit(1);
    }

    // getting the file paths from the arguments
    let file_path = &args[1];
    let xor_pad_file_path = &args[2];

    // open the file and xorpad file
    let mut file = File::open(file_path)?;
    let mut xor_pad = File::open(xor_pad_file_path)?;

    // reading them into memory
    let mut file_data = Vec::new();
    let mut xor_pad_data = Vec::new();
    file.read_to_end(&mut file_data)?;
    xor_pad.read_to_end(&mut xor_pad_data)?;

    if xor_pad_data.is_empty() {
        eprintln!("The XOR pad file is empty! Exiting...");
        std::process::exit(1);
    }

    let progress_bar = ProgressBar::new(file_data.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{bar:40} {percent}%\n[{elapsed_precise}] {bytes}/{total_bytes} (ETA: {eta})")
            .expect("Something went wrong..."),
    );

    // xoring the data
    let mut progressbar_cooldown = 0;

    let xor_result: Vec<u8> = file_data
        .iter()
        .enumerate()
        .map(|(i, &byte)| {
            progressbar_cooldown += 1;
            if progressbar_cooldown % 1000 == 0 {
                progress_bar.inc(1000);
            }
            byte ^ xor_pad_data[i % xor_pad_data.len()]
        })
        .collect();

    progress_bar.finish_with_message("XOR operation completed!");

    // writing the xor'd data to file
    let output_path = format!("{}_xor", file_path);
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&xor_result)?;

    println!("\n\nXOR finished! Output file: {}\n", output_path);

    Ok(())
}

// generate a random xorpad file
fn generate_xorpad(size: usize, filename: &str) -> io::Result<()> {
    let mut xor_pad = vec![0u8; size];
    OsRng.fill_bytes(&mut xor_pad);

    let mut file = File::create(filename)?;
    file.write_all(&xor_pad)?;

    Ok(())
}
