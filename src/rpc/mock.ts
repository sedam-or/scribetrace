import type { Rpc } from "./rpc.ts";
import {
  type HistoryEvent,
  type Mark,
  type Milestone,
  type MilestoneDiff,
  type ExternalOpenOutcome,
  type PaperItem,
  type PaperLinkRequest,
  type WorkspaceInfo,
  type ZoteroErrorCode,
  ZoteroError,
} from "./types.ts";

export interface MockRpcOptions {
  initialWorkspace?: string | null;
  chosenFolder?: string | null;
  papers?: PaperItem[];
  failZoteroCode?: ZoteroErrorCode | string | null;
  zoteroDetail?: string;
  zoteroSeconds?: number;
  externalOutcome?: ExternalOpenOutcome;
}

/** Standard fixture papers drawn from tests/fixtures/zotero/ */
export const FIXTURE_PAPERS: PaperItem[] = [
  {
    id: "z_6WG6QNHB",
    title: "Attention is all you need",
    authors: "Vaswani et al.",
    year: "2017",
    marksCount: 3,
    doiDisplay: "10.48550/arXiv.1706.03762",
    linkAvailability: { doi: "available", pdf: "available", item: "available" },
  },
  {
    id: "z_KBPLWQT5",
    title: "Deep residual learning for image recognition",
    authors: "He et al.",
    year: "2016",
    marksCount: 2,
    doiDisplay: "10.1109/CVPR.2016.90",
    linkAvailability: { doi: "available", pdf: "available", item: "available" },
  },
  {
    id: "h_HMIBIMH7",
    title: "Language models are few-shot learners",
    authors: "Brown et al.",
    year: "2020",
    marksCount: 0,
    doiDisplay: null,
    linkAvailability: { doi: "no_doi", pdf: "no_zotero_key", item: "no_zotero_key" },
  },
];

/** Backend-only source data used by the mock RPC; never exposed in PaperItem. */
const MOCK_PDF_KEYS: Readonly<Record<string, string>> = {
  "z_6WG6QNHB": "EFGH6789",
  "z_KBPLWQT5": "H3E7BPJW",
  "z_EFGH6789": "EFGH6789",
  "z_H3E7BPJW": "H3E7BPJW",
};

const makeFixtureMark = (index: number): Mark => ({
  key: `mark-${String(index).padStart(3, "0")}`,
  kind: "highlight",
  text: `Reading note ${index}: a stable source helps preserve the argument.`,
  comment: index % 5 === 0 ? `Margin note ${index}.\nThis point is useful context.` : null,
  color: index % 2 === 0 ? "#ffd400" : "#5fbf7a",
  page_index: index % 42,
  added: `2026-10-${String((index % 28) + 1).padStart(2, "0")}T09:00:00Z`,
  modified: null,
});

const BASE_MILESTONE_MARKS = Array.from({ length: 200 }, (_, index) => makeFixtureMark(index + 1));
const editedMark = (index: number, changes: Partial<Mark>) => ({
  ...BASE_MILESTONE_MARKS[index - 1]!,
  ...changes,
});
const replaceFixtureMark = (marks: Mark[], replacement: Mark) =>
  marks.map((mark) => mark.key === replacement.key ? replacement : mark);

const MILESTONE_SNAPSHOTS: Record<string, Mark[]> = {
  "milestone-import": BASE_MILESTONE_MARKS.slice(0, 180),
  "milestone-refresh": BASE_MILESTONE_MARKS.slice(0, 188),
  "milestone-session": [
    ...replaceFixtureMark(
      BASE_MILESTONE_MARKS.slice(0, 188).filter((mark) => mark.key !== "mark-004"),
      editedMark(10, { comment: "Margin note 10 revised.\nThis point is useful context." }),
    ),
    ...BASE_MILESTONE_MARKS.slice(188, 193),
  ],
  "milestone-manual": [
    ...replaceFixtureMark(
      BASE_MILESTONE_MARKS.slice(0, 188).filter((mark) => !["mark-004", "mark-008"].includes(mark.key)),
      editedMark(10, { comment: "Margin note 10 revised.\nThis point is useful context." }),
    ),
    ...BASE_MILESTONE_MARKS.slice(188, 195),
  ],
  "milestone-latest": [
    ...replaceFixtureMark(
      BASE_MILESTONE_MARKS.slice(0, 188).filter((mark) => !["mark-004", "mark-008"].includes(mark.key)),
      editedMark(10, { comment: "Margin note 10 revised for final review.\nThis point is useful context." }),
    ),
    BASE_MILESTONE_MARKS[3]!,
    BASE_MILESTONE_MARKS[7]!,
    ...BASE_MILESTONE_MARKS.slice(188, 200),
  ],
};

const MILESTONE_NOTES: Record<string, string> = {
  "milestone-import": "Read-through notes: baseline.",
  "milestone-refresh": "Read-through notes: baseline.\nAdded a question about scope.",
  "milestone-session": "Read-through notes: baseline.\nAdded a question about scope.\nSecond reading clarified the central claim.",
  "milestone-manual": "Read-through notes: baseline.\nSecond reading clarified the central claim.\nPrepared rebuttal outline.",
  "milestone-latest": "Read-through notes: baseline.\nSecond reading clarified the central claim.\nPrepared rebuttal outline.\nUpdated citations.",
};

export const FIXTURE_MILESTONES: Milestone[] = [
  { id: "milestone-import", ts: "2026-10-01T09:00:00Z", reason: "import", label: null, upto: { "mock-device": 180 } },
  { id: "milestone-refresh", ts: "2026-10-08T14:30:00Z", reason: "refresh", label: "Second reading", upto: { "mock-device": 189 } },
  { id: "milestone-session", ts: "2026-10-14T16:15:00Z", reason: "session", label: null, upto: { "mock-device": 195 } },
  { id: "milestone-manual", ts: "2026-10-21T11:20:00Z", reason: "manual", label: "Rebuttal notes", upto: { "mock-device": 198 } },
  { id: "milestone-latest", ts: "2026-11-01T10:05:00Z", reason: "refresh", label: null, upto: { "mock-device": 204 } },
];

export const FIXTURE_VISIBLE_HISTORY: HistoryEvent[] = [
  ...BASE_MILESTONE_MARKS.map((mark, index) => ({
    v: 1,
    id: `fixture-mark-${String(index + 1).padStart(3, "0")}`,
    device: "mock-device",
    seq: fixtureMarkSequence(index + 1),
    ts: mark.added ?? "2026-10-01T09:00:00Z",
    author: "Me",
    type: "mark_import" as const,
    payload: mark,
  })),
  ...FIXTURE_MILESTONES.map((milestone) => ({
    v: 1,
    id: milestone.id,
    device: "mock-device",
    seq: (milestone.upto["mock-device"] ?? 0) + 1,
    ts: milestone.ts,
    author: "Me",
    type: "milestone" as const,
    payload: {
      label: milestone.label,
      reason: milestone.reason,
      upto: milestone.upto,
    },
  })),
  {
    v: 1,
    id: "fixture-rename-second-reading",
    device: "mock-device",
    seq: 206,
    ts: "2026-11-02T10:05:00Z",
    author: "Me",
    type: "milestone_rename" as const,
    payload: { milestone: "milestone-refresh", label: "Second reading" },
  },
].sort((left, right) => left.ts.localeCompare(right.ts) || left.seq - right.seq);

function fixtureMarkSequence(markNumber: number): number {
  if (markNumber <= 180) return markNumber;
  if (markNumber <= 188) return markNumber + 1;
  if (markNumber <= 193) return markNumber + 2;
  if (markNumber <= 195) return markNumber + 3;
  return markNumber + 4;
}

function diffMilestoneMarks(fromId: string, toId: string, before: Mark[], after: Mark[]): MilestoneDiff {
  const beforeByKey = new Map(before.map((mark) => [mark.key, mark]));
  const afterByKey = new Map(after.map((mark) => [mark.key, mark]));
  const added: Mark[] = [];
  const removed: Mark[] = [];
  const edited: MilestoneDiff["edited"] = [];

  for (const mark of after) {
    const previous = beforeByKey.get(mark.key);
    if (!previous) {
      added.push(mark);
    } else if (markContentChanged(previous, mark)) {
      const noteDiff = diffNoteLines(previous.comment, mark.comment);
      edited.push({ key: mark.key, before: previous, after: mark, noteDiff });
    }
  }

  for (const mark of before) {
    if (!afterByKey.has(mark.key)) removed.push(mark);
  }

  const sortByKey = (left: Mark, right: Mark) => left.key.localeCompare(right.key);
  added.sort(sortByKey);
  removed.sort(sortByKey);
  edited.sort((left, right) => left.key.localeCompare(right.key));
  return {
    from: fromId,
    to: toId,
    added,
    removed,
    edited,
    noteDiff: diffNoteLines(MILESTONE_NOTES[fromId] ?? "", MILESTONE_NOTES[toId] ?? ""),
  };
}

function markContentChanged(before: Mark, after: Mark): boolean {
  return before.kind !== after.kind
    || before.text !== after.text
    || before.comment !== after.comment
    || before.color !== after.color
    || before.page_index !== after.page_index;
}

function diffNoteLines(before: string | null, after: string | null): MilestoneDiff["edited"][number]["noteDiff"] {
  const oldLines = before?.split(/\r?\n/) ?? [];
  const newLines = after?.split(/\r?\n/) ?? [];
  const lcs: number[][] = Array.from({ length: oldLines.length + 1 }, () => Array(newLines.length + 1).fill(0));

  for (let i = oldLines.length - 1; i >= 0; i--) {
    for (let j = newLines.length - 1; j >= 0; j--) {
      lcs[i]![j] = oldLines[i] === newLines[j]
        ? (lcs[i + 1]?.[j + 1] ?? 0) + 1
        : Math.max(lcs[i + 1]?.[j] ?? 0, lcs[i]?.[j + 1] ?? 0);
    }
  }

  const lines: MilestoneDiff["edited"][number]["noteDiff"] = [];
  let i = 0;
  let j = 0;
  while (i < oldLines.length && j < newLines.length) {
    if (oldLines[i] === newLines[j]) {
      lines.push({ op: "equal", text: oldLines[i]! });
      i++;
      j++;
    } else if ((lcs[i + 1]?.[j] ?? 0) >= (lcs[i]?.[j + 1] ?? 0)) {
      lines.push({ op: "remove", text: oldLines[i]! });
      i++;
    } else {
      lines.push({ op: "add", text: newLines[j]! });
      j++;
    }
  }
  while (i < oldLines.length) lines.push({ op: "remove", text: oldLines[i++]! });
  while (j < newLines.length) lines.push({ op: "add", text: newLines[j++]! });
  return lines;
}

/** In-memory Rpc for tests and for running the UI without the shell. */
export function createMockRpc(
  marks: Mark[] = [],
  options: MockRpcOptions = {},
): Rpc {
  const history: HistoryEvent[] = [];
  let currentWorkspace: string | null = options.initialWorkspace !== undefined ? options.initialWorkspace : "/mock/workspace";
  const papers = options.papers ?? FIXTURE_PAPERS;

  return {
    chooseFolder: () => {
      const selected = options.chosenFolder !== undefined ? options.chosenFolder : "/home/user/research";
      currentWorkspace = selected;
      return Promise.resolve(selected ? { path: selected } : null);
    },
    getWorkspace: () => {
      return Promise.resolve(currentWorkspace ? { path: currentWorkspace } : null);
    },
    connectZotero: () => {
      if (options.failZoteroCode) {
        return Promise.reject(
          new ZoteroError(
            options.failZoteroCode,
            options.zoteroDetail,
            options.zoteroSeconds,
          ),
        );
      }
      return Promise.resolve([...papers]);
    },
    importMarks: (paperId) => {
      if (!currentWorkspace) {
        return Promise.reject(new Error("No project folder is open."));
      }
      if (!MOCK_PDF_KEYS[paperId]) {
        return Promise.reject(new ZoteroError("attachment_unresolved"));
      }
      for (const mark of marks) {
        history.push({
          v: 1,
          id: `mock-${history.length + 1}`,
          device: "local-1",
          seq: history.length + 1,
          ts: "2026-11-06T10:00:00Z",
          author: "Me",
          type: "mark_import",
          payload: mark,
        });
      }
      return Promise.resolve({ imported: marks.length, milestone: marks.length ? "mock-milestone" : null });
    },
    readHistory: (paperId) => Promise.resolve(
      paperId === FIXTURE_PAPERS[0]?.id
        ? [...FIXTURE_VISIBLE_HISTORY, ...history].sort((left, right) => left.ts.localeCompare(right.ts) || left.seq - right.seq)
        : [...history],
    ),
    list_milestones: () => Promise.resolve(FIXTURE_MILESTONES.map((milestone) => ({ ...milestone, upto: { ...milestone.upto } }))),
    milestone_diff: (_paperId, fromId, toId) => {
      const before = MILESTONE_SNAPSHOTS[fromId];
      const after = MILESTONE_SNAPSHOTS[toId];
      if (!before || !after) return Promise.reject(new ZoteroError("milestone_missing"));
      return Promise.resolve(diffMilestoneMarks(fromId, toId, before, after));
    },
    open_external: ({ paperId, target }: PaperLinkRequest) => {
      const paper = papers.find((item) => item.id === paperId);
      if (!paper) return Promise.reject(new ZoteroError("attachment_unresolved"));
      const availabilityCode = paper.linkAvailability[target === "doi" ? "doi" : target === "pdf" ? "pdf" : "item"];
      if (availabilityCode !== "available") return Promise.reject(new ZoteroError(availabilityCode));
      return Promise.resolve(options.externalOutcome ?? "opened");
    },
  };
}
