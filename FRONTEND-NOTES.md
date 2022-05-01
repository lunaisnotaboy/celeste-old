# Frontend Notes

All user interaction should be done in the frontend, with networking & OS interaction being delegated to the backend.
The frontend code should never have to make network requests or read/write files/databases other than frontend components such as HTML, JS, or CSS.

A splashscreen is displayed while the backend runs it's setup steps.

After that, all info is passed back and forth via tauri commands called from the frontend.

## User Experience
All actions should first be displayed in the frontend, then relayed to the backend, in order to improve latency.

Any permanent actions (deletes, etc.) should have confirmations in order to prevent accidents.

Actions that take a while should be displayed, with a loading bar/spinner.

## Themeing
The chosen theme will be specified in a config file read by the backend, therefore the frontend should ask the backend for that info.

The theme's CSS will be specified in a user-accessible folder (probably something like `~/.local/share/celeste/themes`) which will then be read in and applied.

There should also be a menu for selecting a theme, that will first apply the theme, then relay that info to the backend to be stored.

It'd be nice to bundle Twemoji with the app and use those by default, but also allow the option to use system fonts for that.

By default, we should also give the option to use a font such as [OpenDyslexic](https://github.com/antijingoist/opendyslexic).
