use chrono::{DateTime, FixedOffset, Utc};
use scraper::{Html, Selector};
use std::str::FromStr;

use crate::error::{Error, Result};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct RawItem {
    pub title: String,
    pub author: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<DateTime<FixedOffset>>,
}

pub fn find_feed_link(html_content: &str) -> Result<Option<String>> {
    let document = Html::parse_document(html_content);
    let selector =
        Selector::parse("link[type='application/rss+xml'], link[type='application/atom+xml']")
            .unwrap();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            return Ok(Some(href.to_string()));
        }
    }

    Ok(None)
}

pub fn fetch_feed_title(link: &str, proxy: Option<&str>) -> Result<String> {
    let content = fetch_content(link, proxy)?;
    match content.parse::<Feed>()? {
        Feed::Atom(atom) => Ok(atom.title().to_string()),
        Feed::RSS(rss) => Ok(rss.title().to_string()),
    }
}

pub fn fetch_feed_items(link: &str, proxy: Option<&str>) -> Result<Vec<RawItem>> {
    let content = fetch_content(link, proxy)?;
    match content.parse::<Feed>()? {
        Feed::Atom(atom) => Ok(atom
            .entries()
            .iter()
            .map(|x| RawItem {
                title: x.title().to_string(),
                author: Some(
                    x.authors()
                        .iter()
                        .map(|x| x.name().trim())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                link: x.links().first().map(|x| x.href().to_string()),
                content: x
                    .content()
                    .map(atom_syndication::Content::value)
                    .filter(std::option::Option::is_some)
                    .map(|x| x.unwrap().to_string()),
                published_at: x.published().or(Some(x.updated())).map(|x| x.with_timezone(&Utc).fixed_offset()),
            })
            .collect()),
        Feed::RSS(rss) => Ok(rss
            .items()
            .iter()
            .map(|x| RawItem {
                title: x.title().unwrap_or("Untitled").trim().to_string(),
                author: x
                    .author()
                    .map(|x| x.trim().to_string())
                    .or(x.dublin_core_ext().map(|x| x.creators().join(","))),
                link: x.link().map(std::string::ToString::to_string),
                content: x.description().map(std::string::ToString::to_string),
                published_at: x
                    .pub_date()
                    .map(|x| {
                        DateTime::parse_from_rfc2822(x)
                            .map(|x| x.with_timezone(&Utc).fixed_offset())
                    })
                    .filter(std::result::Result::is_ok)
                    .map(std::result::Result::unwrap),
            })
            .collect()),
    }
}

#[cfg(test)]
pub fn fetch_content(link: &str, _proxy: Option<&str>) -> Result<String> {
    use std::fs;
    Ok(fs::read_to_string(link)?)
}

#[cfg(not(test))]
pub fn fetch_content(link: &str, proxy: Option<&str>) -> Result<String> {
    let client = if let Some(proxy_url) = proxy {
        match reqwest::Proxy::all(proxy_url) {
            Ok(p) => reqwest::blocking::Client::builder().proxy(p).build()?,
            Err(_) => reqwest::blocking::Client::new(),
        }
    } else {
        reqwest::blocking::Client::new()
    };
    Ok(client
        .get(link)
        .header("User-Agent", "Mozilla/5.0")
        .send()?
        .text()?)
}

// borrowed from https://github.com/rust-syndication/syndication

#[derive(Clone)]
pub enum Feed {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

impl FromStr for Feed {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match atom_syndication::Feed::from_str(s) {
            Ok(feed) => Ok(Self::Atom(feed)),
            Err(_) => match rss::Channel::from_str(s) {
                Ok(channel) => Ok(Self::RSS(channel)),
                Err(_) => Err(Error::SyndicationParsingFailure),
            },
        }
    }
}
