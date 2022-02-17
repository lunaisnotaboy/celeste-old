# Developing Celeste

Hi there! As Celeste is an open-source project, there are certain
processes we follow to make sure that the codebase is consistent
throughout.

Please read this _whole_ document before getting started, as it
contains information that will help you.

## Running the application

Running the application is actually pretty easy. Now, you may be
tempted to just run `yarn dev` and wait, but that's not how the
desktop app works. You'll need to run `yarn tauri dev` and wait
for everything to build. Also, do not close the window to end the
process, as it only ends the Tauri application and not the React
dev server.

## Formatting

For Rust code, you can just format it with Cargo like you usually do.
Just be sure to run it in [`src-tauri`](src-tauri/).

For everything else, just run `yarn lint`. ESLint will take care
of it.

## Other things to note

We use [EditorConfig](https://editorconfig.org) to make sure all
of our files are displayed correctly in whatever you use to code.
If your editor does not have built-in support for EditorConfig,
please follow the instructions on their website to do so.

Also, as Tauri is still unstable software, there might be bugs
that we cannot fix. Please still report these to our
[issue tracker](https://github.com/lunaisnotaboy/celeste/issues),
so we can report it to Tauri ourselves.

All development takes place on the main branch. When it comes time
for a new major release, we will create a new branch. We follow
[Semantic Versioning](https://semver.org) for our releases.
