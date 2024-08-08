# YouTube Subtitle Generator

## Required tools

- [Rustup](https://rustup.rs/) with either stable or nightly tool chain installed

## Run

- Flags

```bash
$ cargo run -- --help
Usage: youtube-subtitle-generate [OPTIONS] --input <INPUT> --output-dir <output-dir>

Options:
  -i, --input <INPUT>            Input file containing links of YouTube channel
  -o, --output-dir <output-dir>  Directory to save processed video segment
  -a, --amount <AMOUNT>          Number of videos to process for each channel, process all videos if not specified
  -p, --parallel                 Wether to run script in parallel, the default is false
  -h, --help                     Print help
```

- Examples

```bash
cargo run -- --input input.example.txt --output-dir data --amount 1 --parallel
```
