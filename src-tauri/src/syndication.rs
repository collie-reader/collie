use chrono::{DateTime, FixedOffset, Utc};
use std::error::Error;
use syndication::Feed;

pub struct RawItem {
    pub title: String,
    pub author: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<DateTime<FixedOffset>>,
}

pub fn fecth_feed_title(link: &str) -> Result<String, Box<dyn Error>> {
    let content = reqwest::blocking::get(link)?.text()?;
    match content.parse::<Feed>()? {
        Feed::Atom(atom) => Ok(atom.title().to_string()),
        Feed::RSS(rss) => Ok(rss.title().to_string()),
    }
}

pub fn fecth_feed_items(link: &str) -> Result<Vec<RawItem>, Box<dyn Error>> {
    let content = reqwest::blocking::get(link)?.text()?;
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
                    .map(|x| x.value())
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap().to_string()),
                published_at: x
                    .published()
                    .map(|x| {
                        DateTime::parse_from_rfc3339(x)
                            .map(|x| x.with_timezone(&Utc).fixed_offset())
                    })
                    .filter(|x| x.is_ok())
                    .map(|x| x.unwrap()),
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
                link: x.link().map(|x| x.to_string()),
                content: x.description().map(|x| x.to_string()),
                published_at: x
                    .pub_date()
                    .map(|x| {
                        DateTime::parse_from_rfc2822(x)
                            .map(|x| x.with_timezone(&Utc).fixed_offset())
                    })
                    .filter(|x| x.is_ok())
                    .map(|x| x.unwrap()),
            })
            .collect()),
    }
}
