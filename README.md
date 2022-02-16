# Celeste

A small and cozy Matrix client inspired by Powercord.

## Requirements

- Node.js 16.x
- Rust 1.58.x
- Yarn 1.x

## Development

1. Follow the docs to install [tauri](https://tauri.studio/docs/getting-started/prerequisites)
2. Install yarn dependencies with `yarn install`
3. Run the app with `yarn tauri dev`

Make sure to use `Ctrl+C` to stop the app instead of the close button.

### Formatting
[editorconfig](https://editorconfig.org/) is used for formatting, so please intall a plugin for whatever IDE you're using.

Please also run `cargo fmt` anytime you change the Rust files, as it does some extra stuff as well.
Same with `yarn lint` for the HTML/CSS/JS.

## License

This is licensed under the [MIT license](LICENSE).
