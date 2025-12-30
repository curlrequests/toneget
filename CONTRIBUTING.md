# Contributing to ToneGet

Thank you for your interest in contributing! This project helps Tonal users access and backup their own workout data.

## Project Structure

```
toneget/
├── sync_workouts.py      # Python CLI tool
├── requirements.txt      # Python dependencies
├── desktop-app/          # Tauri desktop application
│   ├── src/              # React frontend
│   └── src-tauri/        # Rust backend
├── README.md
├── LICENSE
├── SECURITY.md
└── CONTRIBUTING.md
```

## How to Contribute

### Reporting Bugs

1. Check if the issue already exists in [Issues](../../issues)
2. If not, create a new issue with:
   - A clear, descriptive title
   - Which tool you're using (Python script or desktop app)
   - Steps to reproduce the problem
   - Expected vs actual behavior
   - Your Python/Node/Rust version and OS
   - Any error messages (with sensitive info redacted)

### Suggesting Features

Open an issue with the `enhancement` label describing:
- What problem it solves
- How you envision it working
- Any alternatives you've considered

### Submitting Code

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly (both Python script and desktop app if applicable)
5. Commit with clear messages (`git commit -m 'Add amazing feature'`)
6. Push to your fork (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Development Setup

### Python Script

```bash
# Create virtual environment
python -m venv venv
source venv/bin/activate  # or `venv\Scripts\activate` on Windows

# Install dependencies
pip install -r requirements.txt

# Run
python sync_workouts.py
```

### Desktop App

```bash
cd desktop-app

# Install Node dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Code Guidelines

### Python Style
- Follow PEP 8
- Use type hints where helpful
- Use meaningful variable names
- Add docstrings for functions
- Keep functions focused and small

### Rust Style
- Follow standard Rust conventions
- Use `cargo fmt` before committing
- Handle errors properly (no `.unwrap()` in production code paths)

### React/JavaScript Style
- Use functional components with hooks
- Keep components small and focused
- Use Tailwind for styling

### Security (Critical)
- **Never** log or store credentials
- **Never** include API keys or secrets
- Sanitize any user data in error messages
- Use HTTPS for all external requests
- Only connect to `tonal.auth0.com` and `api.tonal.com`

### Privacy
- Only access the authenticated user's own data
- Don't add features that could access other users' data
- Respect user privacy in all additions

## What We're Looking For

### High Priority
- Bug fixes
- Cross-platform compatibility improvements
- Error handling improvements
- Documentation improvements
- Accessibility improvements in the desktop app

### Welcome Additions
- Better progress indicators
- Output format options (CSV, etc.)
- Data validation
- Unit tests
- Localization/i18n

### Out of Scope
- Features that access non-user data
- Downloading Tonal's proprietary content (programs, videos, coaches, etc.)
- Anything that could be used to scrape Tonal's platform at scale
- Features requiring reverse engineering beyond personal data access
- Integration with paid services (keep it free and open)

## Keeping Python and Rust in Sync

The Python script and Rust backend should produce identical output. If you add a feature to one, please consider adding it to the other, or note in your PR that it needs to be ported.

Key areas that must stay in sync:
- Data trimming fields (`SET_FIELDS_TO_REMOVE`, etc.)
- Export JSON schema
- API endpoint usage

## Testing

Before submitting a PR:
1. Test with your own Tonal account (if you have one)
2. Verify exports are valid JSON
3. Check that the desktop app builds on your platform
4. Ensure no credentials are logged or stored

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Keep discussions on-topic

## Questions?

Open an issue with the `question` label or start a discussion.

Thank you for helping make this tool better! 🏋️
