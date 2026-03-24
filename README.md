# 🎵 Jukebox

A self-hosted networked jukebox. Run it on any machine, and anyone on
your local network can queue up songs from **music.local**. Audio plays
out of the host device. 

## Screenshots

![Search page](assets/search.png) | ![Admin page](assets/admin.png)

## Features

- **YouTube search** — Users search for songs right from the web UI.
  Results are capped at 10 minutes.
- **Rate-limited queue** — Prevents spamming. Keep things fair.
- **Admin controls** — Pause, play, and skip from a separate admin page.
- **Zero configuration** — Installs with one script, launches with one
  command, and is reachable at `music.local` via mDNS. No ports to
  remember.
- **Fully self-contained** — On first launch, Jukebox downloads the
  correct `ffmpeg` for your platform. `yt-dlp` is freshly downloaded on
  every launch to stay compatible with YouTube.
- **Matrix-themed UI** — Green-on-black aesthetic.

## How It Works

1. A user visits `music.local` and searches for a song.
2. The top results from YouTube (up to 5, all under 10 minutes) are
   displayed.
3. The user picks one and it's added to the queue.
4. Under the hood, `yt-dlp` downloads the audio and `ffmpeg` converts it
   to MP3.
5. Jukebox plays the queue in order through the host machine's primary
   audio output using [rodio](https://github.com/RuSt-Random-Chat/rodio).

The admin can visit the admin page to pause, play, or skip tracks.

## Requirements

- **macOS** or **Linux**
- A network that supports mDNS (most home/office networks do)
- That's it. Everything else is handled for you.

## Installation

Run the install script:

```bash
curl -fsSL https://raw.githubusercontent.com/dposideon/jukebox/refs/heads/master/install.sh | sh
```

This downloads the correct binary for your platform, places it on your
PATH, and sets up permissions so the server can bind to port 80.

Once installed, just run:

```bash
music
```

Then open **[http://music.local](http://music.local)** on any device on
your network.

If mdns is not available, it can still be used by going to the IP address and port printed on the terminal at startup.

The QR code on the webpage also redirects to the IP and port.

## Usage

### For guests

Visit `music.local` in any browser. Type a song name or artist into the
search box, pick a result, and it gets added to the queue.

> **Note:** Only songs under 10 minutes will appear in search results.
> If your search returns fewer than 5 results (or none at all), it's
> likely because most matches exceed the time limit.

### For the host / admin

The admin page is available at `music.local/admin`. From there you can:

- ⏸️ Pause playback
- ▶️ Resume playback
- ⏭️ Skip the current track

> **Note:** There is currently no authentication on the admin page. It
> is not linked anywhere in the main UI, but anyone who knows the URL
> can access it. Password protection is planned for a future release.

## Uninstalling

Run the uninstall script:

```bash
curl -fsSL https://raw.githubusercontent.com/dposideon/jukebox/refs/heads/master/uninstall.sh | sh
```

Or do it manually:

**Linux:**

```bash
rm $(which music)
rm -rf ~/.local/share/jukebox
```

**macOS:**

```bash
rm $(which music)
rm -rf ~/Library/Application\ Support/jukebox
```

## Tech Stack

| Component     | Technology                          |
| ------------- | ----------------------------------- |
| Server        | Rust                                |
| Audio         | rodio (native audio output)         |
| Frontend      | HTML, CSS, vanilla JavaScript       |
| Song download | yt-dlp                              |
| Conversion    | ffmpeg                              |
| Discovery     | mDNS (`music.local`)                |

## Known Limitations

- No authentication on the admin page (yet).
- Port 80 is not configurable.
- Requires mDNS support on the network to use the `music.local` address.
- Song availability depends on yt-dlp's continued compatibility with
  YouTube.

## Acknowledgments

The Matrix-themed UI (HTML, CSS, and JavaScript) was generated with
[Claude](https://claude.ai). All backend code, architecture, and
infrastructure are my own work.

## License

[MIT](LICENSE)

## Disclaimer

This project uses yt-dlp to download audio from YouTube. This may
conflict with YouTube's Terms of Service. Use at your own discretion and
responsibility. This software is provided as-is with no guarantees.
