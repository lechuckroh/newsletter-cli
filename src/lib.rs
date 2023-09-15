use scraper::{Html, Selector};

pub fn get_html(url: &str) -> String {
    return url.to_string();
}

fn read_file(filename: &str) -> String {
    return std::fs::read_to_string(filename).unwrap();
}

pub fn parse_frontend_focus(html: &str) {
    println!("not implemented: {}", html);
}

pub fn parse_golang_weekly(html: &str) {
    let document = Html::parse_document(html);

    let selector = Selector::parse("table.el-item").expect("failed to find ");

    for article in document.select(&selector) {
        let text = article.text().collect::<Vec<_>>().join("\n");
        println!("Article:\n{}", text);
        println!("-------------------");
    }
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
    parse_golang_weekly(html.as_str());
}
