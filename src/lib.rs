use std::fmt;

use reqwest::Client;
use reqwest::redirect::Policy;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Article {
    title: String,
    url: String,
    description: String,
    article_type: String,
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "title: {}, url: {}", self.title, self.url)
    }
}

pub fn get_html(url: &str) -> String {
    return url.to_string();
}

async fn get_redirected_url(url: &str) -> String {
    // Create a new reqwest client with custom redirect policy
    let client = Client::builder()
        .redirect(Policy::custom(|attempt| {
            if attempt.previous().len() > 10 {
                attempt.error("too many redirects")
            } else {
                attempt.follow()
            }
        }))
        .build().unwrap();

    // Make a HEAD request to the URL
    let response = client.head(url).send().await.unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        response.url().to_string()
    } else {
        url.to_string()
    }
}

fn read_file(filename: &str) -> String {
    return std::fs::read_to_string(filename).unwrap();
}

pub fn parse_frontend_focus(html: &str) {
    println!("not implemented: {}", html);
}

pub fn parse_golang_weekly(html: &str) -> Vec<Article> {
    let document = Html::parse_document(html);

    let article_selector = Selector::parse("table.el-item").expect("cannot find article");
    let title_selector = Selector::parse(".mainlink").expect("cannot find title");
    let link_selector = Selector::parse("a").expect("cannot find link");

    let mut articles = Vec::new();

    for article_elem in document.select(&article_selector) {
        let title_elem = article_elem.select(&title_selector).next().unwrap();
        let title = title_elem.text().collect::<Vec<_>>().join("");
        let link_elem = article_elem.select(&link_selector).next().unwrap();
        let url = link_elem.value().attr("href").unwrap();

        let article = Article {
            title,
            url: url.to_string(),
            description: "".to_string(),
            article_type: "".to_string(),
        };
        articles.push(article);
        // let text = article.text().collect::<Vec<_>>().join("\n");
        // println!("Article:\n{}", text);
        // println!("-------------------");
    }

    return articles;
}

pub fn parse_javascript_weekly(html: &str) {
    println!("not implemented: {}", html);
}

pub fn parse_kotlin_weekly(html: &str) {
    println!("not implemented: {}", html);
}

pub fn parse_node_weekly(html: &str) {
    println!("not implemented: {}", html);
}

pub fn parse_scala_times(html: &str) {
    println!("not implemented: {}", html);
}

pub fn parse_this_week_in_rust(html: &str) {
    println!("not implemented: {}", html);
}

#[test]
fn test_golang_weekly() {
    let html = read_file("./tests/inputs/golangweekly-475.html");
    let articles = parse_golang_weekly(html.as_str());
    for article in articles {
        println!("- {}", article);
    }
}
