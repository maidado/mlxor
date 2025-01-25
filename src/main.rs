use rand::{rngs::OsRng, RngCore};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

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

    // getting th efile paths from the arguments
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

    // xoring the data
    let xor_result: Vec<u8> = file_data
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ xor_pad_data[i % xor_pad_data.len()])
        .collect();

    // writing the xor'd data to file
    let output_path = format!("{}_xor", file_path);
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&xor_result)?;

    println!("XOR finished! Output file: {}", output_path);

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
