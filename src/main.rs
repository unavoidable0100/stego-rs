mod cli;
mod utils;


use clap::Parser;
use libaes::{ Cipher };
use passwords::PasswordGenerator;
// use std::{io::Cursor, path::PathBuf};
// use image::io::Reader as ImageReader;

// use pqcrypto::kem::kyber1024::{keypair, encapsulate, decapsulate};


use cli::{Cli, Modes};


fn main() {
    let cli = Cli::parse();

    // For debugging
    // println!("input: {:?}", cli.input_file);
    // println!("mode: {:?}", cli.mode.unwrap());

    // Since mode is Option, it returns either Some(mode) or None
    // so we make a match case for these two
     
    match &cli.mode {
        Modes::Encode(args) => { 
            // println!("{:?} \n{:?}", files.input_file, files.output_file);

            encrypt_message(cli.password.to_string());
            println!("{:?}, {:?}", args.input_file, args.output_file);
        }
        Modes::Decode(_) => { todo!() }
    }
}


fn encrypt_message(password: String) {
    let _pg = PasswordGenerator {
        length: 16,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };


    let hashed_key = blake3::hash(password.as_bytes());
    
    let plaintext = b"A plaintext";
    let iv = b"This is 16 bytes";

    let cipher = Cipher::new_256(hashed_key.as_bytes());

    let encrypted = cipher.cbc_encrypt(iv, plaintext);

    println!("{:?}", encrypted);
}

