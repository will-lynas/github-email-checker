use reqwest::{Error, blocking::Client};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    name: String
}

fn main() -> Result<(), Error> {
    let url = "https://api.github.com/users/will-lynas/repos";

    let client = Client::new();
    let response = client.get(url)
        .header(USER_AGENT, "MyRustApp")
        .send()?;

    let repos: Vec<Repo>;
    if response.status().is_success() {
        repos = response.json()?;
    } else {
        return Err(response.error_for_status().unwrap_err());
    }

    repos.into_iter()
    .map(|repo| repo.name)
    .for_each(|name| get_commits(&name).unwrap());

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Commit {
    commit: BabyCommit,
}

#[derive(Serialize, Deserialize, Debug)]
struct BabyCommit {
    author: Author,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    email: String,
}

fn get_commits(repo: &str) -> Result<(), Error> {
    let url = format!("https://api.github.com/repos/will-lynas/{}/commits", repo);

    let client = Client::new();
    let response = client.get(&url)
        .header(USER_AGENT, "MyRustApp")
        .send()?;

    let commits: Vec<Commit>;
    if response.status().is_success() {
        commits = response.json()?;
    } else {
        return Err(response.error_for_status().unwrap_err());
    }

    let emails: Vec<String> = commits.into_iter()
        .map(|commit| commit.commit.author.email)
        .collect();

    println!("Repo: {}", repo);
    println!("Emails: {:?}", emails);

    Ok(())
}
