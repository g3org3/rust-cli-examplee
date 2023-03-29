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

    if res.status().as_u16() != 200 {
        return Err(CustomError {
            reason: format!(
                "The request was not 200, instead it was {}, '{}'",
                res.status().as_u16(),
                res.text().await.map_err(|err| CustomError {
                    reason: String::from("we could not get the string"),
                    message: err.to_string(),
                })?
            ),
            message: format!("response"),
        });
    }

    res.json::<GithubUser>().await.map_err(|err| CustomError {
        reason: String::from("There is a problem to json parse and map the response."),
        message: err.to_string(),
    })
}
