# Collie

![](https://user-images.githubusercontent.com/6410412/263000117-70fdfe41-18cf-472a-a038-59d39d8b6e20.png)

Collie is a minimal RSS feed reader application. With Collie, you can:

- subscribe to multiple RSS/Atom feeds to organize your own news feed.
- receive a real-time notification when a new item is added to the subscribed feed. (By default, it is checked every two minutes.)
- and save the items to read again or later.

All you need is a local machine and the Internet. No virtual machine, no cloud infrastructures, no always-on database, and no account registration with privacy information required.

Collie is heavily inspired by [Miniflux](https://miniflux.app/) and [RSS app for Slack](https://gdgkr.slack.com/apps/A0F81R7U7-rss).

## Installation

Download the latest release for your system from [release page](https://github.com/parksb/collie/releases). Collie provides `.exe`/`.msi` files for Windows, `.app`/`.dmg` files for macOS, and `.deb`/`.AppImage` files for Linux.

On macOS, you can also install Collie via Homebrew:

```
$ brew install parksb/x/collie
```

## Build

If you want to build Collie from source, you should get code by forking and cloning the git repository or downloading a zip file. After placing the source in your local environment, go to the project directory, and install front-end dependencies using pnpm. (If pnpm is not installed, [install pnpm](https://pnpm.io/installation) first.)

```
$ pnpm install
```

Then, run the following command to build.

```
$ pnpm tauri build
```

This command builds and installs your own Collie based on the cloned source. To develop and modify the application, learn more about [Tauri](https://tauri.app/).

## Screenshots

![](https://user-images.githubusercontent.com/6410412/262967600-4273a958-cb92-427f-9ddc-19446c1b9889.png)

![](https://user-images.githubusercontent.com/6410412/262967611-1edb6675-b56c-4f28-a505-8689d1d7ede6.png)

![](https://user-images.githubusercontent.com/6410412/262967608-063e2cfd-bc82-4aa4-a159-bacda268397d.png)

## Background

I've been getting tech news from HackerNews, Lobsters, etc. on Twitter (It's X now, but I'll keep calling it Twitter anyway), but many of them have been terminated due to changes in Twitter's API policy. I went from place to place: Bluesky, Mastodon, Slack, and newsletter. However, I couldn't settle anywhere. The social media services such as Bluesky and Mastodon had too many unnecessary features as news feed. Slack RSS was good to get the news in real-time, but the notifications mixed with other workspaces overwhelmed me. The newsletters gave me a lot of high-quality information, but not in real-time.

Then, I remembered Miniflux, the "minimalist and opinionated feed reader" that I had used past. This is the best option for my goal, but I had to pay for the hosted version or keep running docker machine on my local computer which did not have enough resources. Additionally, I didn't need a system that maintains multi-user sessions. Eventually, I had no choice but to create my own application, and that's why I made Collie, the minimal RSS reader just for me.

## License

Collie is distributed under the terms of the [GNU General Public License v3.0](LICENSE).
