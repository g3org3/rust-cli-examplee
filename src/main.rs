use clap::Parser;
use std::process;

mod github;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let user_result = github::lib::get_user(&args.name).await;

    match user_result {
        Err(err) => {
            if args.verbose {
                println!("{}\n  > {}", err.reason, err.message,);
            } else {
                println!("{} (try -v to print more details)", err.reason);
            }
            process::exit(1)
        }
        Ok(user) => {
            println!("name: {}", args.name);
            println!("Found {}, with bio {}", user.name, user.gravatar_id);
        }
    }

    Ok(())
}
