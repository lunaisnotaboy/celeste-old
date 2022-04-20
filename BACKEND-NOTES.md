# Backend Notes

## Backend

Options are configured using a `config.ini` file, currently in the project root.
I need to figure out a way to get the default config path on Linux, Windows, an MacOS at some point.

Data will _probably_ be stored in a `sled` database, using various keyspaces for different sections.
eg. blurhash cache, message cache, encryption keys, etc.

The database can be managed via tauri states.

Data should be stored in the proper locations, eg `~/.config`,  `~/.local/share` for the databases, etc.
We should make heavy use of cacheing, and store it *properly* in `~/.cache`, unlike Element.

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
