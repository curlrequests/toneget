# ToneGet

Export your personal Tonal workout data to JSON for backup, analysis, or use with third-party tools.

> ⚠️ **Disclaimer**: This is an unofficial, community-built tool. It is not affiliated with, endorsed by, or connected to Tonal Systems, Inc. in any way. Use at your own risk.

## What This Does

ToneGet allows you to download **your own** workout history from Tonal's servers using your personal login credentials. The data belongs to you—you created it through your workouts.

**What you get:**
- Complete workout history (sets, reps, weights, volume, duration)
- Strength Score history and current muscle-by-muscle breakdown
- Personal records and progression data
- Custom workout templates you've created
- All Tonal-specific metrics (ROM, power, tempo, etc.)

## Installation

```bash
# Clone the repository
git clone https://github.com/curlrequests/toneget.git
cd toneget

# Install dependencies
python3 -m pip install -r requirements.txt

# Run the script
python3 sync_workouts.py
```

**Options:**

```bash
python sync_workouts.py              # Standard export (trimmed, compressed)
python sync_workouts.py --full       # Include all raw API fields
python sync_workouts.py --no-gzip    # Skip compression (JSON only)
```

## Usage

1. Enter your Tonal email and password
2. Wait for the download to complete (usually under a minute)
3. Your data is saved to `tonal_workouts_YYYYMMDD_HHMMSS.json.gz`

Your credentials are only used to authenticate directly with Tonal's servers—they are never stored or transmitted anywhere else.

## Output Format

The export includes:

```json
{
  "version": "3.0",
  "exportedAt": "2025-01-15T10:30:00Z",
  "user": { "firstName": "...", "lastName": "..." },
  "profile": { "totalWorkouts": 150, "totalVolume": 500000 },
  "workouts": [
    {
      "id": "...",
      "beginTime": "2025-01-15T08:00:00Z",
      "workoutType": "PROGRAM",
      "totalVolume": 5000,
      "totalReps": 100,
      "workoutSetActivity": [
        {
          "movementId": "...",
          "weight": 85,
          "repCount": 10,
          "oneRepMax": 113,
          "rangeOfMotion": 0.94
        }
      ]
    }
  ],
  "customWorkouts": { ... },
  "strengthScoreHistory": [ ... ],
  "currentStrengthScores": {
    "parsed": {
      "regions": { "Overall": 487, "Upper": 512, "Lower": 445, "Core": 398 },
      "muscles": { "Chest": { "score": 523 }, "Back": { "score": 498 }, ... }
    }
  }
}
```

## Using Your Data

Your export is a standard JSON file containing your complete workout history. Back it up, analyze it with your own tools, or use it with third-party services that work with structured fitness data. It's your data—do what you want with it.

## Legal Notice

### Your Data Rights

You have a right to access your own personal data. This tool helps you exercise that right by downloading data you created through your own physical activity on equipment you own or lease.

### Important Distinction: Your Data vs. Service Content

1. **Your workout data** - The records you created through your physical activity: timestamps, reps, sets, weights, volume, personal records. This is *your* data.

2. **Service content** - Tonal's proprietary materials like workout programs, instructional videos, coach content, and the movement library. This tool does **not** download or redistribute any of this.

### Terms of Service Considerations

Tonal's Terms of Service contain provisions about automated access. These provisions target scraping of proprietary content—not users accessing their own workout history. However:

- **This tool is provided "as is"** with no warranty
- **You assume all risk** associated with using this tool
- We make no claims about the legality of this tool in your jurisdiction

### What This Tool Does NOT Do

- ❌ Access other users' data
- ❌ Store or transmit your credentials anywhere except Tonal
- ❌ Download Tonal's workout programs or instructional content
- ❌ Bypass any security measures
- ❌ Redistribute any of Tonal's proprietary content

## Privacy & Security

- Credentials are sent directly to Tonal's Auth0 servers over HTTPS
- Credentials are never logged, stored, or sent anywhere except Tonal
- Uses the same OAuth2 authentication as Tonal's official app
- All code is open source and auditable

## Contributing

Contributions are welcome!

- ⭐ Star the repository if you find it useful
- 🐛 Report bugs or issues
- 💡 Suggest features
- 🔀 Submit pull requests

## FAQ

**Q: Is this safe to use?**  
A: The tool only accesses your own data using your Tonal credentials. However, automated access may conflict with Tonal's Terms of Service. Use at your own discretion.

**Q: Will Tonal ban my account?**  
A: We don't know. The tool makes a relatively small number of API requests (similar to normal app usage), but there's always some risk with unofficial tools.

**Q: Why not just use the Tonal app?**  
A: The Tonal app doesn't provide data export functionality. This tool lets you backup your data, perform custom analysis, or use it with other fitness tools.

**Q: Why is the file so small after compression?**  
A: Workout data compresses extremely well (often 85-90% smaller) because it has lots of repeated field names and similar values.

**Q: Can I automate this to run regularly?**  
A: Yes! The script can be run via cron or Task Scheduler. Just store your credentials securely (environment variables, not in the script).

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Remember:** This is your data. You created it. You have a right to it.
