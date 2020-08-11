use reqwest;
use reqwest::Url;
use scraper::Selector;
use scraper::Html;

#[derive(Debug)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

pub type SearchResults = Vec<SearchResult>;

const DJANGO_DOC_SEARCH_URL: &str = "https://docs.djangoproject.com/en/3.0/search/";

async fn do_request(url: &str) -> Result<String, reqwest::Error> {
    Ok(reqwest::
        get(url)
        .await?
        .text()
        .await?)
}

fn parse_django_search(search_results: &Html) -> SearchResults {
    let items_selector = Selector::parse("dl.search-links .result-title > a")
        .expect("it seems django search page change its layout");
    search_results.select(&items_selector).map(|page| {
        let result = SearchResult{
            title: page.text().next().unwrap().to_string(),
            url: page.value().attr("href").unwrap().to_string()
        };
        result
    }).collect()
}

pub async fn search_documentation(terms: &str) -> SearchResults {
    let url = Url::parse_with_params(DJANGO_DOC_SEARCH_URL, &[("q", &terms)]).unwrap();
    //println!("{:?}", url);

    let body = do_request(&url.into_string()).await.unwrap();
    return parse_django_search(&Html::parse_document(&body));
}
