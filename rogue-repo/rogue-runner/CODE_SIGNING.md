# Code Signing for Rogue Runner

**Goal:** Avoid "Unknown Publisher" and SmartScreen warnings on Windows.

---

## 1. MSI Installer (WiX)

The MSI installs to `C:\Program Files\Rogue Runner\` and adds the exe there.

**Build on Windows:**
```powershell
cd rogue-repo\rogue-runner\scripts
.\build-msi.ps1
```

**Prerequisites:** WiX Toolset from https://wixtoolset.org/

**With code signing:**
```powershell
.\build-msi.ps1 -Sign -CertThumbprint "YOUR_CERT_THUMBPRINT"
```

---

## 2. Code Signing Options

### Option A: EV Code Signing (recommended)

- **Effect:** Instant SmartScreen reputation, no "Unknown Publisher"
- **Cost:** ~$300–500/year (Sectigo, DigiCert)
- **Steps:**
  1. Purchase EV cert from Sectigo/DigiCert
  2. Complete identity verification
  3. Receive hardware token (USB)
  4. Sign exe and MSI: `signtool sign /sha1 <thumbprint> /tr http://timestamp.digicert.com /td sha256 /fd sha256 <file>`

### Option B: Standard Code Signing

- **Effect:** Reputation builds over time; early downloads may trigger warnings
- **Cost:** Lower than EV

### Option C: Microsoft Trusted Signing (preview)

- **Effect:** Cloud signing, no hardware token
- **Status:** Public preview at https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/code-signing-for-smart-app-control

---

## 3. Signing the MSI

After obtaining a certificate:

```powershell
signtool sign /sha1 <thumbprint> /tr http://timestamp.digicert.com /td sha256 /fd sha256 rogue-runner-windows-x64.msi
```

Or use the build script:
```powershell
.\build-msi.ps1 -Sign -CertThumbprint "YOUR_THUMBPRINT"
```

---

## 4. Timestamp Servers

- DigiCert: `http://timestamp.digicert.com`
- Sectigo: `http://timestamp.sectigo.com`
- RFC 3161 compliant servers required for long-term validity
