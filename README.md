# Bypass Tiktok Ban

![tiktok-banner-black-3](https://github.com/user-attachments/assets/4e780bcd-894d-4895-9088-541c74b70043)

Fuck the US government, this server allows you to bypass the tiktok ban. You can run it yourself or use our site.

https://x.com/BypassTikTok

## Features

- IP rotation for proxying requests
- Optional authentication using tokens
- Configurable logging
- Docker support

## Setup

1. **Configuration**

   Edit `config/config.toml` with your settings:
   - Server address and port
   - Target URL (e.g., TikTok)
   - Proxy IPs
   - Authentication tokens
   - Logging preferences

2. **Build the Application**

   ```bash
   cargo build --release
