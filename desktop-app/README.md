# ToneGet Desktop App

A simple, beautiful desktop app to export your Tonal workout data.

This is the graphical version of ToneGet. If you prefer the command line, use `sync_workouts.py` in the root directory instead.

## Pre-built Downloads

Download the latest release from the [Releases page](https://github.com/curlrequests/toneget/releases).

### macOS Installation

1. Download the `.dmg` file for your Mac:
   - **Apple Silicon (M1/M2/M3/M4)**: `ToneGet_*_aarch64.dmg`
   - **Intel Macs**: `ToneGet_*_x64.dmg`
2. Open the DMG and drag ToneGet to your Applications folder
3. **First launch** (required because the app isn't signed with an Apple Developer certificate):
   - Double-click ToneGet in Applications
   - You'll see a message: *"Apple could not verify ToneGet is free of malware"*
   - Click **Done** (not "Move to Trash")
   - Open **System Settings → Privacy & Security**
   - Scroll to the bottom—you'll see *"ToneGet was blocked"*
   - Click **Open Anyway**
   - Click **Open** in the confirmation dialog
4. After this one-time setup, ToneGet will open normally

### Windows Installation

1. Download `ToneGet_*_x64-setup.exe`
2. Run the installer
3. Windows SmartScreen may warn about an unknown publisher—click "More info" → "Run anyway"

## Building from Source

### Prerequisites

1. **Node.js** (v18 or later)
   - Download from https://nodejs.org

2. **Rust** (latest stable)
   - Install from https://rustup.rs
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Tauri CLI**
   ```bash
   cargo install tauri-cli
   ```

### Install Dependencies

```bash
cd desktop-app
npm install
```

### Run in Development Mode

```bash
npm run tauri dev
```

This will start the app with hot-reload enabled.

### Build for Production

```bash
npm run tauri build
```

This creates platform-specific installers in `src-tauri/target/release/bundle/`:
- **macOS**: `.dmg` and `.app`
- **Windows**: `.msi` and `.exe`
- **Linux**: `.deb` and `.AppImage`

## Project Structure

```
desktop-app/
├── src/                    # React frontend
│   ├── App.jsx            # Main UI component
│   ├── main.jsx           # React entry point
│   └── index.css          # Tailwind styles
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── main.rs        # Tonal API logic
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── icons/             # App icons
├── package.json           # Node dependencies
└── vite.config.js         # Vite bundler config
```

## Generating App Icons

Before building for production, you need app icons. You can generate them from a single 1024x1024 PNG:

```bash
npm run tauri icon path/to/icon.png
```

This creates all required sizes in `src-tauri/icons/`.

## How It Works

1. User enters their Tonal email/password
2. App authenticates directly with Tonal's Auth0 (same as official app)
3. Downloads all workout history using Tonal's API
4. Saves to a JSON file the user chooses
5. Credentials are never stored - only used for the single session

## Security Notes

- Credentials go directly to `tonal.auth0.com` over HTTPS
- No data is sent anywhere except Tonal's servers
- The app has no analytics, telemetry, or phone-home features
- All code is open source and auditable
