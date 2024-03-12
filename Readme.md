# Unofficial The City of Kings Mapper

Use the tool [here](https://alexkazik.github.io/mapper/).

## Translation

The english version is OCR'd from the game, with permission from the author.
There may be bugs in it, please report/fix when possible.

The german translation is automatically generated from the english one.

If you'd like to have another language please let me know.

You can contact me via [email](mailto:mapper+6527@tx0.eu)
or [BGG](https://boardgamegeek.com/geekmail/compose?touser=txnull).

## Requirements

- https://rustup.rs/
- `rustup target add wasm32-unknown-unknown`
- https://trunkrs.dev/#install

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```

## Building

```bash
trunk build
```

If the application will not be in the domain root (e.g. `https://example.com/mapper`):

```bash
trunk build --public-url /mapper
```
