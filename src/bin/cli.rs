#[macro_use]
extern crate clap;

fn argparser<'a>() -> clap::ArgMatches<'a> {
    clap_app!( myapp =>
        (version: "1.0")
        (author: "obayemi E")
        (about: "searches stuff on the django documentation")
        (@arg terms: +required !multiple <aaa> "terms to search on documentation")
    ).get_matches()
}


#[tokio::main]
async fn main() {
    let matches = argparser();
    let terms = matches.value_of("terms").unwrap();
    let results = djangodocbot::search_documentation(terms).await;
    println!("results for \"{}\"", terms);
    for result in results {
        println!("\n{}\n > {}", result.title, result.url);
    }
}
