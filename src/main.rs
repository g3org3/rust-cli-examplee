use std::process;

use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubUser {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    name: String,
    blog: String,
    location: String,
    bio: String,
    public_repos: i64,
    public_gists: i64,
    followers: i64,
    following: i64,
    created_at: String,
    updated_at: String,
}

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

#[derive(Debug)]
struct CustomError(String);

async fn get_user(username: &String) -> Result<GithubUser, CustomError> {
    let client = reqwest::Client::builder()
        .user_agent("curl/7.54.1")
        .build()
        .map_err(|err| CustomError(err.to_string()))?;

    let url = format!("https://api.gith0ub.com/users/{}", username);
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|err| CustomError(err.to_string()))?;

    res.json::<GithubUser>()
        .await
        .map_err(|err| CustomError(err.to_string()))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let user_result = get_user(&args.name).await;

    match user_result {
        Err(err) => {
            if args.verbose {
                println!("Something wrong happend\n  {:?}", err)
            } else {
                println!("Something wrong happend! (try -v to print more details)")
            }
            process::exit(1)
        }
        Ok(user) => {
            println!("name: {}", args.name);
            println!("Found {}, with bio {}", user.name, user.bio);
        }
    }

    Ok(())
}
