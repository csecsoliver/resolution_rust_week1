use clap::Parser;
use clap::ValueEnum;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;x0
use std::fs;
use std::process::exit;

// #[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
// enum StoriesOptions {
//     #[default]
//     Top,
//     New,
//     Best,
//     Ask,
//     Show,
// }
// impl ValueEnum for StoriesOptions {
//     fn value_variants<'a>() -> &'a [Self] {
//         &[Self::Top, Self::New, Self::Best, Self::Ask, Self::Show]
//     }
//     fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
//         Some(match self {
//             Self::Top => PossibleValue::new("top"),
//             Self::New => PossibleValue::new("new"),
//             Self::Best => PossibleValue::new("best"),
//             Self::Ask => PossibleValue::new("ask"),
//             Self::Show => PossibleValue::new("show"),
//         })
//     }
// }
// impl ToString for StoriesOptions {
//     fn to_string(&self) -> String {
//         match self {
//             Self::Top => ("top").to_string(),
//             Self::New => ("new").to_string(),
//             Self::Best => ("best").to_string(),
//             Self::Ask => ("ask").to_string(),
//             Self::Show => ("show").to_string(),
//         }
//     }
// }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, ValueEnum)]
enum RepoOptions {
    #[default]
    Repo,
    Branches,
    Readme,
    Languages,
    Issues,
    // Commits,
    // Files,
    // PROpen,
    // PRClosed,
    // PRMerged,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// What repo to list
    #[arg(short, long)]
    repo: String,

    /// What to list related to the repo
    #[arg(short, long)]
    option: RepoOptions,
}

#[derive(Deserialize)]
struct User {
    login: String,
    url: String,
    html_url: String,
}

#[derive(Deserialize)]
struct Repo {
    full_name: String,
    html_url: String,
    owner: User,
    private: bool,
    description: Option<String>,
    fork: bool,
    forks: u64,
    watchers: u64,
    default_branch: String,
    open_issues: u64,
}
#[derive(Deserialize)]
struct Branch {
    name: String,
}

#[derive(Deserialize)]
struct Readme {
    content: String,
}

#[derive(Deserialize)]
struct Issue {
    user: User,
    title: String,
    state: bool,
}
// #[derive(Deserialize)]
// struct Story {
//     title: String,
//     url: Option<String>,
//     score: u32,
//     by: String,
//     descendants: u32,
// }

fn get_data<T: serde::de::DeserializeOwned>(url: &str, client: &Client) -> T {
    let data: T = match (match client.get(url).send() {
        Ok(r) => r.json(),
        Err(e) => Err(e),
    }) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to fetch repo with error: {e}");
            exit(1);
        }
    };
    return data;
}

fn main() {
    let gh_key = fs::read_to_string(".env").unwrap_or(("").to_string());

    let client = reqwest::blocking::Client::new();
    let args = Args::parse();
    let repo_name = args.repo;
    let repo_option = args.option;
    if repo_option == RepoOptions::Repo {
        let repo_data: Repo = get_data(
            &format!("https://api.github.com/repos/{repo_name}"),
            &client,
        );
        let branch_count = get_data::<Vec<Branch>>(&repo_name, &client).iter().count();
        println!("Details of repo: {}", repo_data.full_name);
        println!("Owner: {}", repo_data.owner.login);
        println!("Url: {}", repo_data.html_url);
        println!(
            "Description: {}",
            repo_data
                .description
                .unwrap_or(("no description").to_string())
        );
        println!("Is private: {}", repo_data.private);
        println!("Is fork: {}", repo_data.fork);
        println!("Number of forks: {}", repo_data.forks);
        println!("Watchers: {}", repo_data.watchers);
        println!("Number of open Issues: {}", repo_data.open_issues);
        println!("Default branch: {}", repo_data.default_branch);
        println!("Number of branches: {}", branch_count);
        println!("Number of open issues: {}", repo_data.open_issues);
    } else if repo_option == RepoOptions::Branches {
        let branches: Vec<Branch> = get_data(
            &format!("https://api.github.com/repos/{repo_name}/branches"),
            &client,
        );
        println!("Branches of repository {repo_name}:");
        for branch in branches {
            println!("{}", branch.name);
        }
    } else if repo_option == RepoOptions::Readme {
        let readme: Readme = get_data(
            &format!("https://api.github.com/repos/{repo_name}/readme"),
            &client,
        );
        println!("Readme of repo {repo_name}:");
        println!("{}", readme.content)
    } else if repo_option == RepoOptions::Languages {
        let repo_languages: HashMap<String, u64> = get_data(
            &format!("https://api.github.com/repos/{repo_name}/languages"),
            &client,
        );
        println!("Languages used in repo {repo_name}:");
        for language in repo_languages {
            println!("{} ({})", language.0, language.1)
        }
    } else if repo_option == RepoOptions::Issues {
        let repo_issues: Vec<Issue> = get_data(
            &format!("https://api.github.com/repos/{repo_name}/issues"),
            &client,
        );
        print!(
            "{repo_name} has {} open, {} closed issues",
            repo_issues.iter().filter(|x| x.state).count(),
            repo_issues.iter().filter(|x| !x.state).count()
        );
        println!("First 10 open issues' titles in repo:");
        for issue in repo_issues.iter().filter(|y| y.state).take(10).enumerate() {
            println!("{}. {}", issue.0, (*issue.1).title);
            println!("Author: {}", (*issue.1).user.login);
        }
    }
}
