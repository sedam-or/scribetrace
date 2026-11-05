# ScribeTrace: Validation protocol

**Date:** 2026-11-06 · **Author:** pm · **Status:** draft v2, challenged by qa on bias and sample size and approved by qa on 2026-11-06 · Related: [GO-TO-MARKET](GO-TO-MARKET.md), [STRESS-TEST](STRESS-TEST.md) ST-01 and ST-03, [REQUIREMENTS](REQUIREMENTS.md) §1 and §4.
Two tests: **(A)** do people look back at their history (the product gate), **(B)** can a new user reach first value unassisted (the 3-3-3-3 journey). Neither test collects paper content. Numbers are decision rules, not proof of a market.

## 1. Who takes part
- **A:** 8 researchers. **B:** 8 different researchers recruited up front; 5 are used, 3 are a reserve for re-runs after a fix (§3). Nobody takes both tests.
- **Screening (all must be true):** uses Zotero 7 weekly and marks PDFs; **is actively writing a paper, chapter or rebuttal this week** and will re-read at least 3 papers for it. For A, at least 5 of 8 in social sciences or humanities (the primary segment) and 2 from other fields to check the segment choice.
- **Recruiting without courtesy bias:** the owner posts an **open sign-up invitation** (message below) in their own channels. The owner does not see who signs up; pm screens and enrols by the criteria above, and sees participants only under codes P1 to P8. Personal asks to friends are not used. Sign-up channel: a plain email address or a form from a third-party service, whichever the owner prefers (§5; no server of ours).
- No payment is assumed. Any thank-you gift is the owner's decision (§5).

**Invitation (copy):**
> I'm looking for researchers who use Zotero and are writing something this week, to try a small free prototype that keeps a history of the highlights and notes you make, so you can see how your reading of a paper changed. It's very early and I'm checking whether it's useful at all, so honest "I didn't need it" answers help most. About 20 minutes to set up, a few minutes a day for five days, then a 15-minute chat. Everything stays on your computer; I only ask for simple counts and times, never paper titles or highlighted text. You can stop at any time. Reply to [address] if interested.

## 2. Test A: the 5-day diary study (product gate)
**Before (day 0, 20 min):** the participant reads and signs the consent (§4), switches on Zotero's local access, and **double-clicks one launcher** (no terminal, no install of other software; we check on day 0 that it starts, and replace the participant if it cannot be made to start in 20 minutes, counting them as a *setup failure* in the report). It reads the marks of 3 papers they choose (non-confidential ones) and saves a local snapshot. It uses no network, writes only to one local folder, can be deleted at once, and its source is readable.
**Days 1 to 5:** they read and mark as usual. Whenever they want, they open the launcher's **report** ("what changed since the snapshot"), which refreshes the snapshot. **The program itself writes an immutable UTC timestamp for every run** to a local log (nothing else). We send no reminders; one neutral technical check on day 3 never mentions the report.
**Day 6 (15 min chat):** the participant sends the timestamp file (times only) and answers: Did you run it? What for? What did you do with what you saw? Would you miss it? Notes are written by hand, no recording.

**What counts (fixed before the study):**
- **Voluntary run:** a timestamp from the program log. Two runs count as *different days* only if they fall on **different calendar days and at least 12 hours apart** (this blocks day-5 cramming).
- **Concrete use:** the participant describes, with a date, **a specific action that followed from the report**: re-read named marks they had changed or dropped, chose which paper to re-read, or pasted a quote or a change into their own draft. "Interesting" or "nice to have" does not count. pm rates it from the notes and writes down the sentence that justifies each rating, before looking at the count.

| Result | Rule |
|---|---|
| **Pass (product gate)** | At least **4 of 8** have voluntary runs on **at least 2 different days**, **and** at least 3 of those 4 have a concrete use |
| **Fail** | 2 or fewer of 8 meet the first condition: drop the history headline, keep Zotero import, quote, Obsidian export and the format ([BUSINESS §6](BUSINESS.md)) |
| **Inconclusive** | Exactly 3 of 8: add 4 new participants and require 6 of 12 in total; no other change of rule |

**What the rule can tell us** (binomial arithmetic, including the 3-of-8 extension branch; computed 2026-11-06): if the true share who would look back were 60%, the rule passes **88%** of the time; at 50%, 71%; at 30%, **21.5%**; at 20%, 6%. A result of 4 of 8 has a 95% confidence interval of roughly 19% to 81%. So this is a go/no-go screen for a cheap next step, **not** evidence of a market, and a pass must not be quoted as one.
**Known biases, reported with the result:** volunteers like tools; a script is not the app; five days cover active writing, not papers re-read months later (not tested); participants know a researcher is watching; the sign-up reaches the owner's circle, not all Zotero users.

## 3. Test B: first-use usability test (needs the working prototype)
**Setup:** a clean computer or account, the installer, and a **test Zotero library of 3 open-access arXiv papers** with marks and short notes that we wrote ourselves in realistic academic style (no real person's reading, no names). One observer, silent, notes only (no screen recording). The tester gets no instructions beyond the task card. **Timing:** the test build logs a timestamp when each task screen is reached and completed, to a local file on the test machine, with no content and no network; the observer's stopwatch is the backup. The release build has no such logging (R-02, no telemetry).
| Task | Starts when | Pass condition (targets from [debate 08](debate/08-links-and-3-step.md)) |
|---|---|---|
| T1 Install and open | tester receives the download link | Usable window within 5 min of the start, no account |
| T2 Connect to Zotero | window open | "Connected, N papers" without a hint |
| T3 Import a paper's marks | connected | **First mark visible within 5 min of the start of T1** |
| T4 Copy one mark as a quote and paste it | T3 done | Quote with citation pasted within 10 min of the start of T1 |
| T5 Change a mark in Zotero, refresh, find what changed | T4 done | Found without a hint (recorded, not part of the pass rule) |
**Rules:** *Unassisted* means no hint. Stuck for 3 minutes: record the step as failed, give one hint, continue. **Pass: 4 of 5 testers complete T1 to T4 unassisted within the times.** If 2 of the first 3 testers fail at the same step, stop and fix; the count restarts with testers from the reserve of 3 (a third failure of the same kind means the fix is not enough: stop and redesign). After each session ask: "What did you expect to happen next?" and "What would you tell a colleague?"

## 4. Consent, data and privacy
**Consent text (shown and signed before day 0):**
> I agree to take part in a short test of a note-history tool. I understand that: (1) the tool runs only on my computer and sends nothing anywhere; (2) I will share only the **times** at which I ran the report and my answers in a chat, never paper titles, highlighted text or notes; (3) I can read the program's source first, stop at any time, and ask for my answers to be deleted; (4) notes are kept under a code (P1 to P8), not my name, and deleted 90 days after the final report (target). I am responsible for not testing on confidential manuscripts.

**Data we keep:** run timestamps, interview notes under a code, usability times. **We never keep:** paper content, titles, highlights, screen recordings, or contact details beyond what is needed to arrange the chat, deleted after the chat. Whether a university requires ethics approval for interviews with its staff or students is **unverified**; ask each participant to check, and do not enrol anyone who must get approval first.

## 5. User experience: the 5 highest-friction moments (expected; Test B confirms or corrects them)
Each fix serves a stated goal from [REQUIREMENTS](REQUIREMENTS.md); no new requirement is added. "Errors" means wrong clicks or dead ends the observer notes; targets are to be measured.
| # | Friction moment | UX fix (one line) | Goal (R-ID) | Measured in Test B |
|---|---|---|---|---|
| 1 | **First run**: blank window, unclear next step | One screen with three numbered steps and the current one highlighted; no menus visible yet | R-05 | Time T1 to first action; 0 unaided "what now?" questions |
| 2 | **Zotero switch-on**: tester must find a hidden setting | Show the exact setting name and a screenshot, a "Check again" button, and the same text on failure | R-04 | Time in T2; 4 of 5 pass T2 unassisted; errors per attempt |
| 3 | **Empty state**: connected but nothing imported | Show the papers list with an "Import" button on the first paper and one sentence on what happens | R-01, R-05 | Time T2 to T3; 0 hints |
| 4 | **First import**: wait with no sign of progress, or a surprising result (0 marks) | Show a progress count; if 0 marks, say so and name the likely cause (paper has no marks in Zotero) | R-01, R-09 | Time T3 ≤ 5 min from T1; errors; tester can state what was imported |
| 5 | **Refresh and "what changed"**: tester cannot tell what is new | After refresh, open directly on the change list with added, removed and edited marks labelled | R-11, R-12 | T5 found without a hint; time to find; the "what did you expect?" answer |
Success for the section: no moment accounts for a failed hint in more than 1 of 5 sessions. A moment that fails in 2 of the first 3 sessions triggers the stop-and-fix rule in §3.

## 6. Decisions only the user can take
1. **Who posts the open invitation** and which address or form receives replies. Recommend: the owner posts in their channels; replies go to an address pm reads, so the owner never sees who joined.
2. **Thank-you gift or none.** Recommend a small gift paid by the owner, stated up front; no payment tied to results.
3. **Sign-off of the pass/fail rule and the "concrete use" definition before any participant starts.** Recommend yes, so the result cannot be argued afterwards.
4. **Reserve cost:** 16 recruited people in all (8 + 8). Recommend accepting it; fewer testers weaken the stop-and-fix rule.

*Length note: about 2 pages against the 1.5-page brief, because the UX section (§5) and the qa changes were added after the brief.*
