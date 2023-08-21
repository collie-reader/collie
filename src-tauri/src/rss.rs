use std::error::Error;

use rss::Channel;

pub fn fecth_feed_channel(link: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::blocking::get(link)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
