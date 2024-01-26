use reqwest::{Error, blocking::Client};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::env;

fn main() -> Result<(), Error> {
    let api_key = env::var("GITHUB_API_KEY").unwrap();
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let email = &args[2];
    get_repos(&api_key, &name)?
        .into_iter()
        .for_each(|repo| check_commits(&api_key, &name, &repo, email).unwrap());
    Ok(())
}

fn get_repos(api_key: &str, name: &str) -> Result<Vec<Repo>, Error> {
    let url = format!("https://api.github.com/users/{name}/repos");

    let client = Client::new();
    let response = client.get(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header(USER_AGENT, name)
        .send()?;

    if response.status().is_success() {
        let repos = response.json()?;
        Ok(repos)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

fn check_commits(api_key: &str, name: &str, repo: &Repo, email: &str) -> Result<(), Error> {

    let bad_emails: Vec<String> = get_all_commits(api_key, name, repo)?
        .into_iter()
        .map(|commit| commit.commit.author.email)
        .filter(|s| s == email)
        .collect();

    println!("Repo: {}", repo.name);
    println!("Number of bad emails: {}", bad_emails.len());

    Ok(())
}

fn get_all_commits(api_key: &str, name: &str, repo: &Repo) -> Result<Vec<Commit>, Error> {
    let url = format!("https://api.github.com/repos/{}/{}/commits", name, repo.name);

    let mut all_commits: Vec<Commit> = Vec::new();
    let client = Client::new();
    let mut i = 1;
    loop {
        let response = client.get(&url)
            .header("Authorization", format!("Bearer {api_key}"))
            .header(USER_AGENT, name)
            .query(&[("per_page", "100"), ("page", &i.to_string())]) // 100 is max
            .send()?;

        if response.status().is_success() {
            let commits: Vec<Commit> = response.json()?;
            if commits.len() == 0 { break }
            all_commits.extend(commits);
        } else {
            return Err(response.error_for_status().unwrap_err());
        }
        i += 1;
    }
    Ok(all_commits)
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
