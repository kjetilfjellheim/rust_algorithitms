mod args;

use std::fs;

use aes::{AESData, DecryptedState, EncryptedState};
use args::Args;
use clap::Parser;

/**
 * This is a program for testing the AES encryption and decryption.
 * It reads a sequence of bytes from standard input, encrypts them using AES,
 * and then decrypts them back to verify correctness.
 */
fn main() {
    let args = Args::parse();
    if args.input_file.is_empty() || args.output_file.is_empty() {
        eprintln!("Input and output files must be specified.");
        return;
    }
    if args.password.is_empty() {
        eprintln!("Password must be specified.");
        return;
    }
    if args.encrypt {
        encrypt(args.input_file, args.output_file, args.password);
    } else if args.decrypt {
        decrypt(args.input_file, args.output_file, args.password);
    } else {
        eprintln!("Please specify either --encrypt or --decrypt.");
        return;
    }
    println!("Operation completed successfully.");
}

/**
 * Generates a key from the provided password.
 * This is a simple key generation function that pads or truncates the password
 * to ensure it is 16 bytes long (128 bits), which is the required key size
 * for AES encryption.
 */
fn generate_key(password: &[u8]) -> Vec<u8> {
    // Simple key generation: pad or truncate to 16 bytes (128 bits)
    let mut key = vec![0u8; 16];
    for (i, &b) in password.iter().take(16).enumerate() {
        key[i] = b;
    }
    key
}

/**
 *  Encrypts the contents of the input file using AES encryption.
 *  The encrypted data is written to the output file.
 *  The password is used to generate the key for encryption.
 *
 * # Arguments
 * * `input_file`: The path to the file containing the data to encrypt.
 * * `output_file`: The path to the file where the encrypted data will be written.
 * * `password`: The password used to generate the key for encryption.
 *
 */
fn encrypt(input_file: String, output_file: String, password: String) {
    let password = generate_key(password.as_bytes());
    let data = fs::read(&input_file).unwrap_or_else(|_| panic!("Failed to read input file: {input_file}"));
    let roundkeys = AESData::<DecryptedState>::generate_roundkeys(&password);
    let aes_data = AESData::<DecryptedState>::new(data.clone());
    let encrypted = aes_data.encrypt(&roundkeys);
    fs::write(output_file, encrypted.data).unwrap_or_else(|_| panic!("Failed to write to output file"));
}

/**
 *  Decrypts the contents of the input file using AES encryption.
 *  The decrypted data is written to the output file.
 *  The password is used to generate the key for decryption.
 *
 * # Arguments
 * * `input_file`: The path to the file containing the encrypted data.
 * * `output_file`: The path to the file where the decrypted data will be written.
 * * `password`: The password used to generate the key for decryption.
 *
 */
fn decrypt(input_file: String, output_file: String, password: String) {
    let password = generate_key(password.as_bytes());
    let data = fs::read(&input_file).unwrap_or_else(|_| panic!("Failed to read input file: {input_file}"));
    let roundkeys = AESData::<EncryptedState>::generate_roundkeys(&password);
    let aes_data = AESData::<EncryptedState>::new(data.clone());
    let encrypted = aes_data.decrypt(&roundkeys);
    fs::write(output_file, encrypted.data).unwrap_or_else(|_| panic!("Failed to write to output file"));
}
