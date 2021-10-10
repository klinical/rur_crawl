use shells;

fn main() {
    let mut urls = Vec::<String>::with_capacity(5);
    println!(
        "Enter a series of URLs by pressing ENTER after each. \n\
        To stop submitting new URLs, simply input exclusively ENTER."
    );
    while let Some(url) = shells::read_url() {
        urls.push(url);
    }

    shells::run(urls);
}
