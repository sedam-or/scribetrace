# ADR 0012: Distribution, updates and the network rule

**Status:** Accepted, **assumption pending user decision 2** (changes the earlier "notify-only check")  
**Date:** 2026-11-05 (rewritten in Round 8)

## Context
Readers of untrusted files still need patches. A silent updater is both a network connection and a way to replace the program on every machine. Every network feature we add is code to secure and test, and weakens the privacy sentence "the app only talks to Zotero on your computer".

## Options
A. Signed in-place updater. B. In-app "check for updates" request to a version file. C. Installers and package managers; "Check for updates" **opens the release page in the user's browser**; the app makes no update request.

## Decision: C
- Installers (AppImage or Flatpak, Windows installer, macOS disk image) on the release page with published checksums and build provenance; package managers where free (Flathub, Homebrew, winget, Scoop), which update users automatically.
- The menu item shows the installed version and opens the fixed release URL with the system browser. The app has **no update code**: no manifest format, no TLS or timeout handling, no downgrade logic, no stand-in server.
- **Network rule:** the app makes no network request except Zotero on this computer ([ADR 0009](0009-zotero-integration.md)). External links open in the system browser on click.
- **Later:** an in-app notify-only check, added only if users are measured to miss updates (trigger: more than a quarter of reported issues come from outdated versions). An in-place updater only after a signing routine exists.
- Signing (devops, **not verified**): Apple developer programme about $99/year for notarization; Windows via a free open-source signing programme (to confirm); Linux via Flathub or GPG.

## Consequences
+ Smallest attack surface; one network destination; ~60 lines of spec and tests removed. − Direct-download users update by hand and must compare versions themselves.

Security: SEC-1 reduces to a single exception (Zotero loopback); SEC-28 shrinks to "link is fixed, opened in the system browser" (security to reword). Supply chain items SEC-25, 26, 27, 29 apply unchanged.
