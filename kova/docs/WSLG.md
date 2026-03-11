# WSLg Setup for Kova GUI

WSLg (Windows Subsystem for Linux GUI) lets Linux GUI apps like `kova gui` run on Windows with proper display.

## Quick setup

1. **Create or edit `.wslconfig`** in your Windows user profile:
   ```
   C:\Users\<YourUsername>\.wslconfig
   ```

2. **Add:**
   ```ini
   [wsl2]
   guiApplications=true
   ```

3. **Apply:**
   ```powershell
   wsl --shutdown
   ```
   Then reopen your WSL terminal.

## Verify

```bash
wsl --version
# Should show "WSLg version: 1.x.x"

# Test
xclock &
```

## If GUI still fails (EGL/MESA errors)

- Update WSL: `wsl --update`
- Ensure Windows 11 build 22000+ or Windows 10 21H2+
- Install GPU drivers (NVIDIA/AMD/Intel) on Windows
