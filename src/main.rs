use futures::{stream, StreamExt};
use select::document::Document;
use select::predicate::Name;
use select::predicate::Class;
use std::fs::OpenOptions;
use std::io::prelude::*;

async fn httptest(job: i64) -> Vec<String>{
    let url = format!("https://www.beianx.cn/latest/p{}",job);
    let response = reqwest::get(url)
    .await
    .unwrap()
    .text()
    .await.unwrap();
    let document = Document::from(response.as_str());
    let table = document.find(Class("table-sm")).next().unwrap();
    let res: Vec<String> = table.find(Name("div")).map(|x| x.first_child().unwrap().first_child().unwrap().text()).collect();
    res
}

#[tokio::main]
async fn main() {
    let jobs = 1..21;
    let concurrency = 10;
    let results: Vec<Vec<String>> = stream::iter(jobs)
        .map(httptest)
        .buffer_unordered(concurrency)
        .collect()
        .await;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true).create(true)
        .open("1.txt")
        .unwrap();
    for domains in results{
        for domain in domains{
            file.write(format!("{}\r\n",domain).as_bytes()).unwrap();
        }
    }
}
