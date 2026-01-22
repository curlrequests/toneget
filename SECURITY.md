# Security Policy

## How This Tool Handles Your Credentials

ToneGet needs your Tonal login credentials to access your workout data. Here's exactly what happens:

### What We DO:

- ✅ Send your credentials directly to Tonal's Auth0 authentication servers via HTTPS
- ✅ Use the same OAuth2 flow that Tonal's official mobile app uses
- ✅ Discard your password from memory immediately after authentication
- ✅ Use TLS/HTTPS for all network communication

### What We DO NOT Do:

- ❌ Store your password anywhere
- ❌ Log your credentials
- ❌ Send your credentials to any server other than Tonal's
- ❌ Save authentication tokens between sessions
- ❌ Include any analytics, telemetry, or tracking
- ❌ Phone home to any servers

## Verifying This Yourself

The entire codebase is open source. You can verify:

### Python Script

Check `sync_workouts.py` and search for `authenticate`. You'll see credentials go directly to `tonal.auth0.com`:

```python
response = requests.post(
    f"https://{AUTH0_DOMAIN}/oauth/token",
    json={...}
)
```

### Network Monitoring

Use a tool like Wireshark, Charles Proxy, or mitmproxy to verify the only external connections are to Tonal's servers.

## Reporting Security Issues

If you find a security vulnerability:

1. **DO NOT** open a public issue
2. Email the maintainer directly (see profile)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact

We'll respond within 48 hours and work with you on a fix.

## Best Practices for Users

1. **Use a strong, unique password** for your Tonal account
2. **Download only from official sources** - GitHub releases or building from source
3. **Verify checksums** if provided with releases
4. **Review the code** before running if you're security-conscious
5. **Don't share your export files** publicly (they contain personal data)
6. **Store exports securely** - they contain your workout history

## Dependencies

| Package | Purpose | Risk Level |
|---------|---------|------------|
| requests | HTTP client | Low - widely audited |

We intentionally minimize dependencies to reduce attack surface.

