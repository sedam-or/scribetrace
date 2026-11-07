# ScribeTrace: User guide (Linux first)

**Status:** draft v0.1, 2026-11-06 · **Author:** pm · **Applies to:** the 1.0.0 release (not yet published). **Nothing in this guide can be installed yet**: ScribeTrace is pre-alpha ([README](../README.md)). Each section is marked **built** or **planned** by slice number from [BUILD-PLAN](BUILD-PLAN.md); update the marks as slices land. On-screen wording comes from `src/strings/en.json`; if this guide and the app disagree, that file wins and this guide is wrong.
Screenshots are added later from qa's end-to-end run.

*ScribeTrace is an independent project, not affiliated with or endorsed by Zotero or Obsidian.*

## 1. What it does, and what it does not
ScribeTrace remembers how your reading of a paper changed. You keep reading and marking in **Zotero 7** and writing in your notes or **Obsidian**. ScribeTrace copies your marks in on request, saves a **milestone** each time something changed, and shows what was added, removed or edited between two milestones. Everything is stored in plain files you can read without the app.

It does not manage references, sync, read PDFs itself (Stage 1), or keep the history of a changing web page.

## 2. Install (planned: needs the first release)
Releases will be listed at <https://github.com/sedam-or/scribetrace/releases>. Pick one:

| Format | Steps |
|---|---|
| **AppImage** | Download `ScribeTrace_<version>_amd64.AppImage`, make it runnable (`chmod +x ScribeTrace_<version>_amd64.AppImage`), then double-click it or run it from a terminal. |
| **.deb** (planned, not part of 1.0) | Not offered with 1.0. Once it is published, it will be installed with `sudo apt install ./ScribeTrace_<version>_amd64.deb`. Until then, use the AppImage. |

The AppImage needs a 64-bit system and the WebKitGTK 4.1 library (`libwebkit2gtk-4.1`). It uses the library already on your system: on Fedora, `sudo dnf install webkit2gtk4.1`; on Debian and Ubuntu, `sudo apt install libwebkit2gtk-4.1-0`. The AppImage's launcher sets `WEBKIT_DISABLE_DMABUF_RENDERER=1`. If FUSE is not available on your system, run `APPIMAGE_EXTRACT_AND_RUN=1 ./ScribeTrace_<version>_amd64.AppImage` instead. Check the download against the published `SHA256SUMS` before you run it. There is no account and no installer wizard. Flatpak is planned, not promised ([CI-CD](CI-CD.md) §4). **Release builds, file names and the signature steps are not final.** Other distributions (Fedora, Arch and so on) should work with the AppImage but are untested.

## 3. Set up in three steps (slices 4 and 5; planned)
1. **Switch on Zotero access.** In Zotero, open **Settings, then Advanced**, and tick **"Allow other applications on this computer to communicate with Zotero"**. Zotero must be running whenever you import. *(A live Zotero 7 check showed that with this setting off, Zotero refuses with "Local API is not enabled" ([SPIKE-3](spikes/zotero-live-check.md)). That check changed the setting in a config file, so the exact label and menu path are still to be confirmed in the Zotero window.)*
2. **Choose your project folder.** ScribeTrace creates a `.scribetrace` folder inside it. Your papers are not touched.
3. **Press Connect to Zotero.** You should see "Connected, N papers." If not, the message names the setting to fix; press **Check again**. ScribeTrace never retries by itself.

Target, not yet measured: first mark imported within 5 minutes of install.

## 4. Everyday use (slices 5 to 8; planned)
1. **Import.** Pick a paper and press **Import marks**. Papers with no marks in Zotero are listed as "No marks in Zotero yet".
2. **Refresh.** After you mark more in Zotero, press **Refresh**. If something changed, a new milestone is saved; if not, you are told nothing changed. You can rename any milestone ("first skim", "rebuttal").
3. **Look back.** Open the timeline, choose two milestones, and read what was added, removed or edited, plus any change in your notes. Imported marks are read-only copies: edit them in Zotero.
4. **Copy a quote.** Press **Copy as quote** on a mark. You get the text with a citation key (or author, year and title if there is no key) and a link back to the page in Zotero. ScribeTrace asks before opening any link.
5. **Open a paper (ticket #30; planned).** The paper page shows the title, authors, DOI and the number of marks in its history. It has three buttons:
   - **Open DOI page** opens the paper's DOI address in your web browser.
   - **Open PDF in Zotero** opens the PDF inside Zotero. Zotero must be running.
   - **Show in Zotero** selects the paper in Zotero. Zotero must be running.

   Before any link opens, ScribeTrace shows the full address (a web address or a `zotero://` address) and asks **Open** or **Cancel**. The link then opens in the program your computer uses for it: your web browser, or Zotero. ScribeTrace never opens a web page inside itself, and it never opens a link by itself. If a paper has no DOI, or did not come from Zotero, its button is disabled and the page says why.

## 5. Notes (slice 9; planned)
Each paper's notes are a plain Markdown file (`note.md`) with every saved version kept. If you edit the file in another program, the next time you open the paper ScribeTrace keeps that text as a new version; both are in the history. Notes are limited to 512 KB. Markdown is shown as plain, safe text: scripts and unsafe links are not run.

## 6. Go back to an earlier milestone (slice 10; planned)
Restoring adds a new entry; the earlier history stays as it was. Zotero is not changed.

## 7. Forget a paper's history (slice 11; planned)
Use this for confidential material, such as a manuscript you reviewed.
1. Open the paper and choose **Forget this paper's history**, then confirm.
2. ScribeTrace deletes that paper's history files on this computer and records a "forgotten" event. Your other computers show a banner offering to delete their hidden files.

**Limits, plainly:** copies in a sync service (Syncthing, Dropbox and similar) or in Git history **may remain**. An offline computer keeps its files until it opens the project and you delete them. Forget is not secure erasure. Your papers and Zotero are not changed.

## 8. Export to Obsidian (slice 12; planned)
Choose **Export to Obsidian** and a folder in your vault. ScribeTrace writes only inside a `ScribeTrace` subfolder there and overwrites only files it made that you have not edited; files you edited are skipped and reported. Export is **one way**: changes you make in Obsidian do not come back. It will not write to locations outside the folder you chose.

## 9. Your files
History lives in `<project>/.scribetrace/papers/<paper>/<device>.jsonl`, one file per computer, and notes in `<project>/.scribetrace/notes/`. A command-line tool, `scribetrace-cli`, reads them with the same code as the app. `scribetrace-cli check <folder>` counts damaged lines for each paper and gives the reason; its exit code is 0 when all lines are fine, 1 when damaged lines were found, and 2 for a usage or read error. `scribetrace-cli export <folder> [paper]` prints a paper's history as Markdown to the screen and writes no files. A `timeline` command follows with slice 7. Files are never changed by these commands. The format is **unstable v0** and may change before 1.0. Back up or sync the folder with your own tools; keep it out of public Git repositories if it holds confidential work.

## 10. Privacy
No account, no telemetry, no background process. The only network contact is a request to Zotero **on this computer** (127.0.0.1) when you click Import, Refresh or Connect. **Check for updates** only opens the releases page in your browser. Full policy: [PRIVACY](../PRIVACY.md). Marks and notes are shown as text only, never run. The author label in files is "Me"; your user name is not written.

## 11. Limits (honest list)
- Needs Zotero 7 running with local access on. A live check on Linux with Zotero 7 and three open-access papers showed that the local interface returns highlights, notes, colours, dates and page positions ([SPIKE-3](spikes/zotero-live-check.md)). It was a small test; other mark types and large libraries are not yet checked.
- No history of a changing web page. Save links with Zotero's browser button, then import.
- PDF and Markdown only. Linux is the first supported system; Windows and macOS follow the same build, untested for users.
- No own reader in Stage 1.
- Numbers such as installer size or start time are targets until measured.

## 12. Troubleshooting
| Message | What to do |
|---|---|
| "Zotero cannot be reached" | Start Zotero and keep it open; press Check again. |
| "Zotero said no" | Zotero is running but its access setting is off: tick it (step 1 above); press Check again. |
| "Zotero did not answer" | Close any open Zotero dialog, then press Check again. |
| "N damaged lines were skipped" | Your files were not changed. Run `scribetrace-cli check <folder>` to see where. |
| The AppImage does not start, and mentions WebKitGTK or `libwebkit2gtk` | Install the WebKitGTK 4.1 runtime (section 2), then start it again. |
| The AppImage does not start, and mentions FUSE | Run `APPIMAGE_EXTRACT_AND_RUN=1 ./ScribeTrace_<version>_amd64.AppImage`. |

Report problems at <https://github.com/sedam-or/scribetrace/issues>. Do not paste paper text or notes you want to keep private.
