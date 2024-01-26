use reqwest::{Error, blocking::Client};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Error> {
    let name = "will-lynas";
    get_repos(name)?
        .into_iter()
        .for_each(|repo| print_commits(repo).unwrap());
    Ok(())
}

fn get_repos(name: &str) -> Result<Vec<Repo>, Error> {
    let url = format!("https://api.github.com/users/{name}/repos");

    let client = Client::new();
    let response = client.get(url)
        .header(USER_AGENT, "MyRustApp")
        .send()?;

    if response.status().is_success() {
        let repos = response.json()?;
        Ok(repos)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

fn print_commits(repo: Repo) -> Result<(), Error> {
    let url = format!("https://api.github.com/repos/will-lynas/{}/commits", repo.name);

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

    println!("Repo: {}", repo.name);
    // println!("Emails: {:?}", emails);
    println!("Number of commits: {:?}", emails.len());

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Commit {
    commit: InnerCommit,
}

#[derive(Serialize, Deserialize, Debug)]
struct InnerCommit {
    author: Author,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    email: String,
}
