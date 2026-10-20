# AI-assisted workflow

The point of this project is twofold: a faithful Rust port, and visible evidence of how to direct AI on a nontrivial rewrite. Both need a repeatable process and a truthful record.

## Roles

| | Does | Does not |
| --- | --- | --- |
| Human (maintainer) | Sets scope, approves ADRs, reviews every PR diff, decides divergences, owns merges and tags | Rubber-stamp green CI |
| AI agent | Reads the Python source and spec, drafts tests then code, runs checks, writes PR bodies and log entries, flags ambiguities | Edit `spec/`, change scope or conformance expectations to make tests pass, merge unreviewed work |

## Per-slice loop

1. **Specify.** Agent states the slice, its spec refs, and its done criteria (3–7 bullets) in the PR description before coding.
2. **Grill** (`grilling`) only when the spec is silent or the Python code contradicts the docs. Outcome goes to `06-spec-gaps.md` or an ADR.
3. **Red.** Failing tests first: conformance case, ported Python test, or property test.
4. **Green + refactor.** Smallest change that passes; then idiomatic cleanup with tests still green.
5. **Verify.** fmt, clippy, tests, schema validation, and the differential scenario for the slice if one exists.
6. **Review.** `code-review` skill on the diff, in a fresh context. Fix or consciously dismiss each finding; record both in the PR.
7. **Record.** PR body (`pr` skill) with the *AI assistance* section; ledger entries; TODO and HANDOFF updated.
8. **Human review and merge.**

## Evidence ledger

[AI_LEDGER.md](../AI_LEDGER.md) is a table of events, not a diary. Log:

- **Catch**: AI produced a defect that a test, the compiler, clippy, the differential job, or the human caught.
- **Override**: human rejected or redirected an AI proposal, with the reason.
- **Win**: AI found something non-obvious (a spec contradiction, a Python quirk) with the evidence.
- **Gap**: spec or tooling insufficiency that cost time.

Each row links a commit or PR. Do not backfill from memory; write the row in the session where it happens. An empty ledger at M9 is itself a finding: it means the log was not kept.

## Metrics worth collecting (cheap, honest)

Per milestone: Python LoC read vs Rust LoC written, tests ported, PRs, review findings by severity, defects caught by layer (compiler / unit / conformance / differential / human), wall-clock time. Put them in the milestone's PROJECT_LOG entry. The case study (M9) reads from these.

## Case study (`docs/CASE_STUDY.md`, written in M9)

Outline fixed now so evidence is gathered toward it: *Why port; how the contract was extracted; verification design; the five best catches (from the ledger); what the AI was bad at; what changed my mind; numbers; what I would do differently.* Write from the ledger, not from impressions.

## Prompting conventions

Prompts live in `prompts/`. A milestone prompt names the milestone file, the Python modules to read, the spec docs, the traps, the exit criteria, and the stop condition ("stop at the PR; do not merge"). Keep prompts short: the milestone files carry the detail.
