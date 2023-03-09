use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};
use std::process;

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
    flag: bool,
}

#[derive(Debug)]
struct CustomError(String);

async fn get_user(username: &String) -> Result<GithubUser, CustomError> {
    let client = reqwest::Client::builder()
        .user_agent("curl/7.54.1")
        .build()
        .unwrap();

    let url = format!("https://api.github.com/users/{}", username);
    let res = client.get(url).send().await.unwrap();

    let user = res.json::<GithubUser>().await.unwrap();

    return Ok(user);
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let user = get_user(&args.name).await.unwrap();

    println!("name: {}", args.name);
    println!("Found {}, with bio {}", user.name, user.bio);

    return Ok(());
}
