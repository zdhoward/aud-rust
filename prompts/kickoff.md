# Kickoff prompt (paste into a fresh session in this repo)

```
You are executing the aud-rust port. Read AGENTS.md, then HANDOFF.md, then docs/plan/README.md and the active milestone file named in HANDOFF.md.

Execute the active milestone:
- Follow TODO.md top to bottom for that milestone, one vertical slice per branch and PR.
- Read the Python modules and spec docs the milestone lists before writing code. Read docs/plan/05-porting-traps.md for the traps it cites.
- Test first. Run fmt, clippy -D warnings and tests before every commit.
- Record surprises in docs/plan/06-spec-gaps.md (spec issues) and docs/AI_LEDGER.md (AI misses, overrides, wins) as they happen.
- Open PRs with the AI-assistance section filled in. Stop at "PR open and CI green"; do not merge unless I say so.
- Ask me only for decisions the plan marks open, anything that changes scope, or anything outward-facing that the plan does not already authorize.

When the milestone's exit criteria are all checked, update TODO.md, ROADMAP.md, CHANGELOG.md, PROJECT_LOG.md and HANDOFF.md, then tell me what is done, what is open, and propose the next milestone prompt.
```

## Variants

- **Single slice:** replace "Execute the active milestone" with "Execute only task M2-3 from TODO.md".
- **Wrap up:** "Wrap up the session." (global wrap-up procedure applies; HANDOFF.md is the handoff).
- **Review only:** "Run code-review on the open PR for the active milestone and report findings. Do not change code."
- **Spec question:** "The spec is unclear about X. Read the Python source, report the exact behavior with file and line, and propose an SG entry."
