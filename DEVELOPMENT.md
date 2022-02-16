# Development Notes

## Frontend

A splashscreen is displayed while the backend runs it's setup steps.
After that, all info is passed back and forth via tauri commands called from the frontend.

The frontend probably will never directly access databases/config files, but instead call on the backend to do that for it.

`// note: idk how you want to manage themes, but rn i have an option in the config to specify a theme name`

It'd be nice to bundle Twemoji with the app and use those by default, but also allow the option to use system fonts for that.
Also we should by default give the option to use a font such as [OpenDyslexic](https://github.com/antijingoist/opendyslexic).

## Backend

Options are configured using a `config.ini` file, currently in the project root.
I need to figure out a way to get the default config path on Linux, Windows, an MacOS at some point.

Data will *probably* be stored in a `sled` database, using various keyspaces for different sections.
eg. blurhash cache, message cache, encryption keys, etc.

The database can be managed via tauri states.

## Features

### Images

Once we receive a new image, a blurhash should be computed and cached.
It will then be displayed immediately, until the full image has been fetched.
This could also be used for image spoilers.

For compatibility with blurhash, images should be displayed with a preview the same size as the blurhash.

We should also have the ability to attach multiple images, and send messages with the images.
I don't think that's yet a part of the matrix protocol, but we could implement it in the frontend then just send the mesages seperately.

Might be nice to optimize uploaded images by default as well

### Messages

We should support full markdown, including '||' for spoilers.

Custom emoji support would be good to implement too, along with stickers.

### Encryption

At first I think it'd be best to design for future encryption in mind, but not actually implement it until later.

Eventually we should support all encryption features though, including cross-signing.

We should probably support SSO login at some point aswell.
