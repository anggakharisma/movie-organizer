# Movie Organizer

***Movie Organizer*** is a cross-platform desktop application built with Tauri, Rust, React, and TypeScript for managing movies.

![Screenshot](ss.png)

### What you need
- OMDB api key

### Features
✅ Auto fetch poster/metadata using OMDB API.

✅ Sanitize and clean our folder name.

✅ TOML-based configuration.


❌ Right click open location with file manager

❌ Play movie within the app

❌ Multi library suport

### Development steps
First install the dependencies
```
{pnpm, yarn, npm} install
```

then run
```
{pnpm, yarn} tauri dev or npm run tauri dev
```

## Troubleshoot
⚠️ Poster doesn't show up? The app will try it's best to sanitize your folder name but, if it doesn't work you may need to manually rename the folder.
