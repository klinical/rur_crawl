use rayon::prelude::*;
use reqwest::StatusCode;
use select::{document::Document, predicate::Name};
use std::io::{self, BufRead, Write};

pub fn run(urls: Vec<String>) {
    let url_iter = urls.par_iter();

    let mut fetched = Vec::new();
    url_iter
        .map(|url| match reqwest::blocking::get(url) {
            Ok(resp) => crawl_page(resp),
            Err(_) => Vec::with_capacity(0),
        })
        .collect_into_vec(&mut fetched);

    for result_list in fetched {
        run(result_list);
    }
}

pub fn crawl_page(resp: reqwest::blocking::Response) -> Vec<String> {
    if let StatusCode::OK = resp.status() {
        let text = resp.text().unwrap();

        Document::from(text.as_str())
            .find(Name("a")) // Find all a nodes
            .filter_map(|n| match n.attr("href") {
                // Filter each node for nodes that have the href attr and map to a new iterator
                Some(link) => {
                    println!("Got link: {}", link);
                    Some(link.to_owned())
                } // for every node that has the attr, allocate the node's value to a String
                None => None, // or if node didnt have the href attr, let it fail the map operation
            })
            .collect()
    } else {
        Vec::with_capacity(0)
    }
}

pub fn read_url() -> Option<String> {
    print!("Enter target URL: ");
    let _ = io::stdout().flush();
    let stdin = io::stdin();

    loop {
        if let Some(Ok(line)) = stdin.lock().lines().next() {
            if line == "" {
                return None;
            } else {
                return Some(line);
            }
        }
    }
}
