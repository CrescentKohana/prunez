# ListenBrainz Prune

At the time of writing ListenBrainz doesn't have the option to mass delete listens. This alleviates that. 

## Usage

- Copy and rename `example.env` to `.env` and change LISTENBRAINZ_API_KEY to the one found in: [listenbrainz.org/profile](https://listenbrainz.org/profile)
- Export ListenBrainz listens: [listenbrainz.org/profile/export](https://listenbrainz.org/profile/export)
- Place the exported .json with the executable. Rename it to listens.json or set a different source filename with --filename.
- Run the executable with options below.

```prunez [options]```

- `--filename, -f` sets source filename. Default: **listens.json**
- `--url, -u` sets API URL. Default: **https://api.listenbrainz.org**
- `--artists, -a` list of artists. All listens associated with them will be pruned. Separated by `;;`.
- `--songs, -s` list of song titles. Separated by `;;`.
- `--recids, -r` list of recording msids. Separated by `;`.

## Development

- Format with `cargo fmt`
- Build with `cargo build`
- Run with `cargo run -- [options]`
