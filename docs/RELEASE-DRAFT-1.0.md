# ScribeTrace 1.0 (AppImage): release notes and README draft

**Status: draft, not released.** Nothing here is published until the owner has checked the release candidate and says so (decision n). Owner: pm. Devops commits this file. Version number and file names come from the release pipeline ([CI-CD](CI-CD.md)); the README gets section B only when the release exists.

---

## A. Release notes (text for the GitHub release page)

**ScribeTrace 1.0 for Linux (AppImage)**

ScribeTrace remembers how your reading of a paper changed. It copies your marks from Zotero 7 on request, saves a milestone each time something changed, and lets you compare any two milestones. You keep reading in Zotero.

**What is in this release**
- Set up in three steps: switch on Zotero's local access, choose a project folder, press Connect.
- Import marks from a paper in Zotero, and refresh them later. A changed set is saved as a new milestone; an unchanged one is reported as unchanged.
- Name milestones and compare two of them: what was added, removed or edited.
- Copy a mark as a quote with its citation key (or author, year and title) and a link back to the page in Zotero. ScribeTrace asks before it opens any link.
- Open a paper's DOI page, its PDF in Zotero, or show it in Zotero. Every link asks first.
- Plain files you own: one history file per computer in `.scribetrace/`, readable without the app.
- `scribetrace-cli check` counts damaged lines in those files and gives the reason for each; `scribetrace-cli export` prints a paper's history as Markdown.
- Forget a paper's history on this computer, with a plain warning that copies in sync services or Git may remain.

**Known limits**
- Needs Zotero 7 running, with its local access switched on, whenever you import or refresh.
- Highlights and notes are read from Zotero, with their dates and page positions. Other mark types (underline, image, ink) are not yet verified on a live Zotero.
- No history of a changing web page.
- PDF and Markdown only. Linux is the only system this release is built and tested on; Windows and macOS are not offered yet.
- The file format is **unstable v0** and may change before 1.0 of the format.
- Sizes, memory use and start time are targets. This release does not publish measured numbers.

**Install (AppImage)**
1. Download `ScribeTrace_<version>_amd64.AppImage` and `SHA256SUMS`.
2. Check the file: `sha256sum -c SHA256SUMS --ignore-missing`. It must print `OK`.
3. Make it runnable and start it: `chmod +x ScribeTrace_<version>_amd64.AppImage`, then double-click it or run `./ScribeTrace_<version>_amd64.AppImage`.

The AppImage needs the host WebKitGTK 4.1 runtime. Fedora: `sudo dnf install webkit2gtk4.1`. Debian and Ubuntu: `sudo apt install libwebkit2gtk-4.1-0`. The AppImage's desktop launcher sets `WEBKIT_DISABLE_DMABUF_RENDERER=1`. If FUSE is not available, run it with `APPIMAGE_EXTRACT_AND_RUN=1`. A `.deb` package is planned, not part of 1.0 ([user guide](USER-GUIDE.md) §2).

Report problems at <https://github.com/sedam-or/scribetrace/issues>. Do not paste paper text or notes you want to keep private.

---

## B. README section (paste on release day; replaces "Quick start (planned)" install step)

Replace the status line with the release status, and the **Install** step with:

```markdown
## Download and run (Linux, AppImage)

1. Download `ScribeTrace_<version>_amd64.AppImage` and `SHA256SUMS` from the [releases page](https://github.com/sedam-or/scribetrace/releases).
2. Check the download: `sha256sum -c SHA256SUMS --ignore-missing` must print `OK`.
3. Make it runnable and start it: `chmod +x ScribeTrace_<version>_amd64.AppImage`, then double-click it or run `./ScribeTrace_<version>_amd64.AppImage`.

Needs a 64-bit system and the host WebKitGTK 4.1 runtime: `sudo dnf install webkit2gtk4.1` on Fedora, `sudo apt install libwebkit2gtk-4.1-0` on Debian and Ubuntu. The launcher sets `WEBKIT_DISABLE_DMABUF_RENDERER=1`. If FUSE is not available, run with `APPIMAGE_EXTRACT_AND_RUN=1`. No account. Full steps: [user guide](docs/USER-GUIDE.md).
```

---

## C. Open points before release (pm to check, not for publication)

1. **Version number.** Resolved: 1.0.0 (CEO order "v1.0 release for .appimage"). CHANGELOG, USER-GUIDE and this file all say 1.0.0; 0.1.0 is dropped. No tag until the owner says go.
2. **Slices shipped.** The "What is in this release" list must match the slices that really passed their acceptance tests. Remove any bullet for a slice that did not.
3. **Zotero setting label.** The setup text names the Settings > Advanced label. It is not yet confirmed in a live Zotero window (SPIKE-3). Confirm before release.
4. **Signature and checksum steps.** Confirm with devops that `SHA256SUMS` is published next to the file and what the name is.
5. **Link confirmation.** Confirm the "Open DOI page", "Open PDF in Zotero" and "Show in Zotero" buttons are in the release (ticket #30) before listing them.
