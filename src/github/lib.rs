pub use crate::github::types::GithubUser;
use reqwest;

#[derive(Debug)]
pub struct CustomError {
    pub reason: String,
    pub message: String,
}

pub async fn get_user(username: &String) -> Result<GithubUser, CustomError> {
    let client = reqwest::Client::builder()
        .user_agent("curl/7.54.1")
        .build()
        .map_err(|err| CustomError {
            reason: String::from("There is a problem with the reqwest client builder."),
            message: err.to_string(),
        })?;

    let url = format!("https://api.github.com/users/{}", username);
    let res = client.get(url).send().await.map_err(|err| CustomError {
        reason: String::from("There is a problem with the request."),
        message: err.to_string(),
    })?;

    let status_code = res.status().as_u16();
    if !res.status().is_success() {
        return Err(CustomError {
            reason: format!(
                "The request to github's api was not successful ({}).",
                status_code
            ),
            message: res.text().await.map_err(|err| CustomError {
                reason: String::from("we could not get the string"),
                message: err.to_string(),
            })?,
        });
    }

    res.json::<GithubUser>().await.map_err(|err| CustomError {
        reason: String::from("There is a problem to json parse and map the response."),
        message: err.to_string(),
    })
}
