use rayon::prelude::*;
use reqwest::StatusCode;
use select::{document::Document, predicate::Name};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn run() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut urls: Vec<String> = Vec::new();
    let mut output_file = File::create("output.txt").unwrap();

    println!(
        "Enter a series of URLs by pressing ENTER after each. \n\
        To stop submitting new URLs, simply input exclusively ENTER."
    );
    while let Some(url) = read_url() {
        urls.push(url);
    }

    println!("CRAWLER STARTING.");

    let _ = std::thread::spawn(move || loop {
        if let Some(input) = read() {
            tx.send(input).unwrap();
        }
    });
    'outer: loop {
        let url_iter = urls.chunks(5);

        let mut fetched = Vec::new();
        for chunk in url_iter {
            if let Ok(command) = rx.try_recv() {
                println!("Got command: {}", command);

                if command == "stop" || command == "quit" {
                    println!("CRAWLER STOPPING.");
                    break 'outer;
                }
            }

            fetched.append(
                &mut chunk
                    .par_iter()
                    .flat_map(|link| {
                        if let Ok(resp) = reqwest::blocking::get(link) {
                            crawl_page(resp)
                        } else {
                            Vec::new()
                        }
                    })
                    .filter(|link| !urls.contains(link) && !fetched.contains(link))
                    .collect::<Vec<String>>(),
            );
        }

        println!("CRAWLER FOUND {} LINKS", fetched.len());

        for link in &fetched {
            let _ = output_file.write(link.as_bytes());
            let _ = output_file.write(b"\n");
        }

        urls = fetched;
    }
}

pub fn crawl_page(resp: reqwest::blocking::Response) -> Vec<String> {
    if let StatusCode::OK = resp.status() {
        let text = resp.text().unwrap();

        Document::from(text.as_str())
            .find(Name("a")) // Find all a nodes
            .filter_map(|n| match n.attr("href") {
                Some(link) => {
                    if link.contains("http://") || link.contains("https://") {
                        Some(link.to_owned())
                    } else {
                        None
                    }
                } // for every node that has the attr, allocate the node's value to a String
                None => None, // or if node didnt have the href attr, let it fail the map operation
            })
            .collect()
    } else {
        Vec::with_capacity(0)
    }
}

fn read() -> Option<String> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next();

    match line {
        Some(Ok(input)) => Some(input),
        _ => None,
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
