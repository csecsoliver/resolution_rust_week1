use std::io::Error;
use std::thread::JoinHandle;

use clap::Parser;
use clap::ValueEnum;
use clap::builder::PossibleValue;
use reqwest::StatusCode;
use reqwest::blocking::Response;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum StoriesOptions {
    #[default]
    Top,
    New,
    Best,
    Ask,
    Show,
}
impl ValueEnum for StoriesOptions {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Top, Self::New, Self::Best, Self::Ask, Self::Show]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Top => PossibleValue::new("top"),
            Self::New => PossibleValue::new("new"),
            Self::Best => PossibleValue::new("best"),
            Self::Ask => PossibleValue::new("ask"),
            Self::Show => PossibleValue::new("show"),
        })
    }
}
impl ToString for StoriesOptions {
    fn to_string(&self) -> String {
        match self {
            Self::Top => ("top").to_string(),
            Self::New => ("new").to_string(),
            Self::Best => ("best").to_string(),
            Self::Ask => ("ask").to_string(),
            Self::Show => ("show").to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The metric to sort the stories by
    #[arg(short, long, default_value_t = StoriesOptions::Top)]
    stories_option: StoriesOptions,
}

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
    descendants: u32,
}

fn main() {
    let client = reqwest::blocking::Client::new();
    let args = Args::parse();
    let stories_option = args.stories_option.to_string();
    println!("The first 10 of Hacker News' {stories_option} stories\n");
    let stories_url = format!("https://hacker-news.firebaseio.com/v0/{stories_option}stories.json");
    let top_ids: Vec<u64> = match (match client.get(stories_url).send() {
        Ok(r) => r.json(),
        Err(e) => Err(e),
    }) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to fetch story list with error: {e}");
            return;
        }
    };

    for (i, id) in top_ids.iter().take(10).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

        let story = client.get(&url).send();

        let response = match story {
            Ok(response) => response.json(),
            Err(e) => Err(e),
        };
        let story: Story = match response {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to fetch story with error: {e}");
                continue;
            }
        };

        let link = story.url.as_deref().unwrap_or("(no url)");
        println!(
            "{}, {} ({} points and {} comments, by {})",
            i + 1,
            story.title,
            story.score,
            story.descendants,
            story.by
        );
        println!("\t{}\n", link);
    }
}
