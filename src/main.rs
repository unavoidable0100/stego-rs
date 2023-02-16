use clap::{ Parser };

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct StegoArgs {
    #[arg(value_enum)]
    mode: String,
    #[clap(short, long, required = true)]
    file: String,
    #[clap(short, long)]
    message: Option<String>
}

fn main() {
    let args = StegoArgs::parse();

    match args.mode.as_str() {
        "encode" => {
            println!("Encoding");

            image::open(&args.file).expect("Failed to open image");

            args.message.expect("Message argument is required");
        }
        "decode" => {
            println!("Decoding");
        }
        _ => {
            println!("Invalid Mode.")
        }
    }

    // println!("mode: {0} , file: {1} , message: {2:?}", args.mode, args.file, args.message);
}