# Simple Video Server (Rust)

A lightweight HLS video streaming server built with Rust, featuring automatic video ID generation and web-based streaming.

## Features

- **HLS Video Streaming**: Serves `.m3u8` playlists and `.ts` video segments
- **Clean URLs**: Watch videos at `/watch?v=VIDEO_ID`
- **Random Video IDs**: Automatically generates 11-character video IDs
- **FFmpeg Integration**: Convert any video format to HLS using the built-in converter
- **High Performance**: Built with Rocket web framework for fast HTTP serving

## Project Structure

```
simple-video-server-rs/
├── Cargo.toml           # Workspace configuration
├── server/              # HTTP video server
│   ├── Cargo.toml
│   └── src/main.rs
├── cm3u8/               # Video-to-HLS converter
│   ├── Cargo.toml
│   └── src/main.rs
└── videos/              # Video storage
    └── <VIDEO_ID>/      # Video ID folders
        ├── <VIDEO_ID>.m3u8    # HLS playlist
        ├── <VIDEO_ID>0.ts     # Video segments
        ├── <VIDEO_ID>1.ts
        └── ...
```

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (1.85.0)
- [FFmpeg](https://ffmpeg.org/) (for video conversion)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/ywjno/simple-video-server-rs
cd simple-video-server-rs
```

2. Build the project:

```bash
cargo build --release
```

### Usage

#### 1. Convert Videos to HLS

Use the `cm3u8` tool to convert your video files:

```bash
# Convert a video - automatically generates video ID
cargo run --release -p cm3u8 -- -i input_video.mp4

# Example output:
# Video ID: Xe5NCiPrw5s
# Playlist: ./Xe5NCiPrw5s/Xe5NCiPrw5s.m3u8
```

This automatically:

- Generates a random 11-character video ID
- Creates a directory with the video ID
- Converts the video to HLS format with 10-second segments

#### 2. Move Videos to Server Directory

```bash
# Move the generated video folder to the videos directory
mkdir -p videos
mv Xe5NCiPrw5s videos/
```

#### 3. Start the Server

```bash
# Start with default port (8080)
cargo run --release -p simple-video-server

# Or set a custom port
PORT=3000 cargo run --release -p simple-video-server
```

The server will start at `http://localhost:8080`.

#### 4. Watch Videos

Open your browser or media player and navigate to:

```
http://localhost:8080/watch?v=Xe5NCiPrw5s
```

**VLC Media Player:**

1. Open VLC
2. Go to "Media" → "Open Network Stream"
3. Enter: `http://localhost:8080/api/watch?v=Xe5NCiPrw5s`
4. Click "Play"

## API Reference

### GET /watch?v=&lt;video_id&gt;

Serves the HLS playlist for the specified video.

**Parameters:**

- `v`: Video ID (11-character string)

**Example:**

```
GET /watch?v=Xe5NCiPrw5s
→ videos/Xe5NCiPrw5s/Xe5NCiPrw5s.m3u8
```

### GET /&lt;video_segment&gt;

Serves video segments (.ts files) referenced by the playlist.

**Parameters:**

- `video_segment`: Video segment filename (must end with `.ts`)

**Example:**

```
GET /Xe5NCiPrw5s0.ts
→ videos/Xe5NCiPrw5s/Xe5NCiPrw5s0.ts
```

**Security:** Only `.ts` files are served through this endpoint.

## Configuration

### Environment Variables

- `PORT`: Server port (default: 8080)

### Server Settings

The server is configured with:

- Bind address: `0.0.0.0` (all interfaces)
- Default port: `8080`
- Log level: Normal (debug) / Critical (release)
- Route prefix: `/`

## Video ID Generation

Video IDs are generated using:

- **Length**: 11 characters
- **Character set**: `A-Z`, `a-z`, `2-9` (excluding confusing characters like `0`, `O`, `I`, `l`, `1`)
- **Example**: `dQw4w9WgXcQ`

This ensures unique, readable video identifiers.

## FFmpeg Conversion Settings

The converter uses these FFmpeg settings:

- **Segment duration**: 10 seconds
- **Video codec**: H.264 (libx264)
- **Audio codec**: AAC
- **Hardware acceleration**: Auto-detected
- **Playlist type**: HLS with unlimited segments

## Development

### Build Commands

```bash
# Build all components
cargo build

# Build server only
cargo build -p simple-video-server

# Build converter only
cargo build -p cm3u8

# Run in development mode
cargo run -p simple-video-server
```

### Dependencies

**Server:**

- `rocket` - Web framework

**Converter:**

- `clap` - Command-line argument parsing
- `rand` - Random video ID generation

## File Organization

Videos are organized as follows:

```
videos/
├── Xe5NCiPrw5s/
│   ├── Xe5NCiPrw5s.m3u8     # Playlist
│   ├── Xe5NCiPrw5s0.ts      # Segment 0
│   ├── Xe5NCiPrw5s1.ts      # Segment 1
│   └── ...
└── ...
```

## License

This project is dual-licensed under either of:

- MIT License
- Apache License 2.0

You may choose either license for your use.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
