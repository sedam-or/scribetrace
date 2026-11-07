import assert from "node:assert/strict";
import { test } from "node:test";

import { renderToStaticMarkup } from "react-dom/server";

import {
  AppShell,
  Button,
  DiffBadge,
  EmptyState,
  ErrorBanner,
  FolderPicker,
  Input,
  PaperListRow,
  PaperPage,
  DiffLines,
  MilestoneTimelineList,
  TimelineItem,
  ToastNotice,
} from "../src/components/index.ts";
import { FIXTURE_MILESTONES } from "../src/rpc/mock.ts";

test("Button renders variants, sizes and accessibility attributes", () => {
  const primaryHtml = renderToStaticMarkup(
    <Button variant="primary" size="small">
      Save
    </Button>,
  );
  assert.match(primaryHtml, /class="[^"]*st-btn-primary[^"]*"/);
  assert.match(primaryHtml, /class="[^"]*st-btn-sm[^"]*"/);
  assert.match(primaryHtml, /type="button"/);
  assert.match(primaryHtml, />Save<\/button>/);

  const disabledHtml = renderToStaticMarkup(
    <Button variant="destructive" disabled>
      Delete
    </Button>,
  );
  assert.match(disabledHtml, /class="[^"]*st-btn-destructive[^"]*"/);
  assert.match(disabledHtml, /disabled=""/);
  assert.match(disabledHtml, /aria-disabled="true"/);
});

test("Input and FolderPicker render paired labels and picker affordances", () => {
  const inputHtml = renderToStaticMarkup(
    <Input id="test-input" label="Search Papers" value="Quantum" placeholder="Filter..." />,
  );
  assert.match(inputHtml, /<label for="test-input"/);
  assert.match(inputHtml, /value="Quantum"/);
  assert.match(inputHtml, /placeholder="Filter\.\.\."/);

  const pickerHtml = renderToStaticMarkup(
    <FolderPicker
      id="project-dir"
      label="Project Folder"
      value="/home/user/papers"
      browseLabel="Browse..."
      onBrowse={() => {}}
      readOnly
    />,
  );
  assert.match(pickerHtml, /<label for="project-dir"[^>]*>Project Folder<\/label>/);
  assert.match(pickerHtml, /value="\/home\/user\/papers"/);
  assert.match(pickerHtml, /readonly=""/i);
  assert.match(pickerHtml, />Browse\.\.\.<\/button>/);
});

test("PaperListRow renders metadata, marks count, and keyboard attributes", () => {
  const rowHtml = renderToStaticMarkup(
    <PaperListRow
      id="p1"
      title="Attention Is All You Need"
      authors="Vaswani et al."
      year="2017"
      marksCount={8}
      statusText="Refreshed 2h ago"
      active
      onRefresh={() => {}}
    />,
  );
  assert.match(rowHtml, /role="button"/);
  assert.match(rowHtml, /tabindex="0"/);
  assert.match(rowHtml, /aria-selected="true"/);
  assert.match(rowHtml, /class="[^"]*active[^"]*"/);
  assert.match(rowHtml, /Attention Is All You Need/);
  assert.match(rowHtml, /Vaswani et al\. · 2017/);
  assert.match(rowHtml, /8 marks/);
  assert.match(rowHtml, /aria-label="Refresh marks for Attention Is All You Need"/);
});

test("PaperPage renders paper metadata and explains unavailable Zotero links", () => {
  const html = renderToStaticMarkup(
    <PaperPage
      paper={{
        id: "paper-1",
        title: "A paper title",
        authors: "A. Author et al.",
        marksCount: 1,
        doiDisplay: null,
        linkAvailability: { doi: "no_doi", pdf: "no_pdf", item: "no_zotero_key" },
      }}
      onOpenExternal={async () => "opened"}
      onShowHistory={() => {}}
    />,
  );

  assert.match(html, /<h1 id="paper-page-title">A paper title<\/h1>/);
  assert.match(html, /A\. Author et al\./);
  assert.match(html, /No DOI in Zotero for this paper\./);
  assert.match(html, /1 mark in history/);
  assert.match(html, /Open DOI page/);
  assert.match(html, /Open PDF in Zotero/);
  assert.match(html, /Show in Zotero/);
  assert.match(html, /disabled=""/);
  assert.match(html, /This paper has no DOI in Zotero/);
  assert.match(html, /Zotero has no PDF attached to this paper/);
  assert.match(html, /Open timeline/);
  assert.doesNotMatch(html, /<dialog/);
  assert.doesNotMatch(html, /https:\/\//);
});

test("DiffLines renders untrusted note text as inert content and preserves context lines", () => {
  const html = renderToStaticMarkup(
    <DiffLines lines={[
      { op: "remove", text: "<script>bad()</script>" },
      { op: "equal", text: "unchanged context" },
      { op: "add", text: "new text" },
    ]} />,
  );

  assert.doesNotMatch(html, /<script>/);
  assert.match(html, /&lt;script&gt;bad\(\)&lt;\/script&gt;/);
  assert.match(html, /unchanged context/);
  assert.match(html, /new text/);
});

test("MilestoneTimelineList renders the timeline and accessible compare picker", () => {
  const html = renderToStaticMarkup(
    <MilestoneTimelineList
      milestones={FIXTURE_MILESTONES}
      activeMilestoneId="milestone-refresh"
      fromId="milestone-import"
      toId="milestone-refresh"
      onOpenCompare={() => {}}
      onFromChange={() => {}}
      onToChange={() => {}}
      onCompare={() => {}}
    />,
  );

  assert.equal((html.match(/class="st-timeline-item/g) ?? []).length, 5);
  assert.match(html, /Imported from Zotero/);
  assert.match(html, /Second reading/);
  assert.match(html, /Saved by you/);
  assert.match(html, /role="group" aria-labelledby="milestone-picker-instructions-milestone-refresh"/);
  assert.match(html, /id="milestone-picker-instructions-milestone-refresh">Choose two milestones to compare\.<\/p>/);
  assert.match(html, /<span>From<\/span>/);
  assert.match(html, /<span>To<\/span>/);
  assert.match(html, /aria-expanded="true"/);
});

test("TimelineItem renders milestones, author origin, and diff badges", () => {
  const itemHtml = renderToStaticMarkup(
    <TimelineItem
      id="m1"
      title="v2 - Rebuttal edits"
      timestamp="Nov 5, 2026, 14:30"
      author="Me"
      device="laptop"
      diff={{ added: 4, edited: 2, removed: 1 }}
      onRestore={() => {}}
    />,
  );
  assert.match(itemHtml, /v2 - Rebuttal edits/);
  assert.match(itemHtml, /Nov 5, 2026, 14:30 · Me · laptop/);
  assert.match(itemHtml, /\+4/);
  assert.match(itemHtml, /~2/);
  assert.match(itemHtml, /-1/);
  assert.match(itemHtml, /Restore this milestone/);
});

test("DiffBadge renders glyphs and accessible labels (WCAG 1.4.1)", () => {
  const addedHtml = renderToStaticMarkup(<DiffBadge type="added" count={5} />);
  assert.match(addedHtml, /aria-label="5 added"/);
  assert.match(addedHtml, />\+5<\/span>/);
  assert.match(addedHtml, /class="[^"]*st-diff-added[^"]*"/);

  const editedHtml = renderToStaticMarkup(<DiffBadge type="edited" count={3} />);
  assert.match(editedHtml, /aria-label="3 edited"/);
  assert.match(editedHtml, />~3<\/span>/);
  assert.match(editedHtml, /class="[^"]*st-diff-edited[^"]*"/);

  const removedHtml = renderToStaticMarkup(<DiffBadge type="removed" count={1} />);
  assert.match(removedHtml, /aria-label="1 removed"/);
  assert.match(removedHtml, />-1<\/span>/);
  assert.match(removedHtml, /class="[^"]*st-diff-removed[^"]*"/);
});

test("EmptyState renders title, body, action, and diagnostic notice", () => {
  const emptyHtml = renderToStaticMarkup(
    <EmptyState
      title="No papers imported yet"
      body="Pick a paper from your Zotero library."
      actionLabel="Import Marks"
      onAction={() => {}}
      diagnostic="No annotations found in Zotero."
    />,
  );
  assert.match(emptyHtml, /No papers imported yet/);
  assert.match(emptyHtml, /Pick a paper from your Zotero library\./);
  assert.match(emptyHtml, /No annotations found in Zotero\./);
  assert.match(emptyHtml, />Import Marks<\/button>/);
});

test("ErrorBanner renders role alert, setting advice, and no-retry notice", () => {
  const bannerHtml = renderToStaticMarkup(
    <ErrorBanner
      title="Zotero cannot be reached"
      body="Open Settings, then Advanced, and tick Allow other applications."
      actionLabel="Check again"
      onAction={() => {}}
      notice="ScribeTrace does not retry on its own."
    />,
  );
  assert.match(bannerHtml, /role="alert"/);
  assert.match(bannerHtml, /Zotero cannot be reached/);
  assert.match(bannerHtml, /Open Settings, then Advanced/);
  assert.match(bannerHtml, /ScribeTrace does not retry on its own\./);
  assert.match(bannerHtml, />Check again<\/button>/);
});

test("ToastNotice renders status role and polite live region", () => {
  const toastHtml = renderToStaticMarkup(
    <ToastNotice
      message="Quote copied with citation."
      visible
      onDismiss={() => {}}
    />,
  );
  assert.match(toastHtml, /role="status"/);
  assert.match(toastHtml, /aria-live="polite"/);
  assert.match(toastHtml, /Quote copied with citation\./);
  assert.match(toastHtml, /aria-label="Dismiss notice"/);

  const hiddenHtml = renderToStaticMarkup(
    <ToastNotice message="Hidden" visible={false} />,
  );
  assert.equal(hiddenHtml, "");
});

test("AppShell renders skip link, brand header, nav pane, and detail pane", () => {
  const shellHtml = renderToStaticMarkup(
    <AppShell navContent={<nav>Navigation List</nav>} defaultTheme="dark">
      <div>Main Content View</div>
    </AppShell>,
  );
  assert.match(shellHtml, /class="st-skip-link"/);
  assert.match(shellHtml, /href="#main-content"/);
  assert.match(shellHtml, /ScribeTrace/);
  assert.match(shellHtml, /Remember how your reading changed/);
  assert.match(shellHtml, /Theme: Dark/);
  assert.match(shellHtml, /aria-label="Papers"/);
  assert.match(shellHtml, /id="main-content"/);
  assert.match(shellHtml, /Navigation List/);
  assert.match(shellHtml, /Main Content View/);
});
