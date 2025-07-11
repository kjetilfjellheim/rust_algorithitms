use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to encrypt or decrypt
    #[arg(short, long)]
    pub input_file: String,

    /// Output file after encrypt or decrypt
    #[arg(short, long)]
    pub output_file: String,

    /// Password
    #[arg(short, long)]
    pub password: String,

    /// Encrypt
    #[arg(short, long, conflicts_with = "decrypt")]
    pub encrypt: bool,

    /// Decrypt
    #[arg(short, long, conflicts_with = "encrypt")]
    pub decrypt: bool,
}
