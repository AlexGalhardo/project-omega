# Definition of Done

Status: APPROVED by owner direction, 2026-10-24. This contract distinguishes document approval from verified implementation.

An implementation task is DONE only when its approved spec requirements and assigned scope are implemented, all applicable acceptance checks have passed on the reported commit, required native checks have actual evidence, integration/review has completed, confirmed bugs have regression coverage, provenance/dependency obligations are met and no unresolved blocking change request remains. The task and spec evidence must link the exact results; a passing unrelated test command is insufficient.

Required report fields: task ID, spec/criterion IDs, commit, changed files and reason, environment/tool versions, exact commands and exit status, expected versus actual results, manual/native evidence, limits, blockers, reviewer outcome and next handoff. Report skipped/unavailable checks as incomplete. Approval, IMPLEMENTED and VERIFIED are distinct lifecycle events; self-review is not an independent model review.

For planning documents, completion means requested content exists, dependencies/references/statuses are consistent, evidence labels/provenance are honest and documentary checks pass. It does not mean a planned command was executed or a game is playable. Stage 4 owner approval can accept contracts while implementation remains absent. Stage 5 must assign tasks and verify dependency/requirement coverage before implementation begins.

Do not expand a task to fix unrelated defects. File a change request when a required contract cannot be met. Preserve other workers' changes and stage only assigned files. No secret, proprietary source asset or unsupported license assumption may be added to a release artifact. The final 1.0 gate remains separate from RC qualification.
