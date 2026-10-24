# Change request authoring contract

Use an unused CR ID and descriptive title; this template is not a filed request. Record date, author/task/spec, status (Proposed / Approved / Rejected / Superseded) and blocking scope.

State the observed defect or missing decision with reproducible evidence: commit, environment, exact command/input, expected versus actual result, or conflicting document passages. Link the affected approved decision, GDD requirement, ADR, spec, public contract and task. Do not present a preference as an objective defect.

List the smallest viable alternatives, including preserving the existing contract if possible, their costs/risks and effects on content, tests, dependencies, scope and schedule. Recommend one and identify who must approve it. New owner-level product choices need owner authorization; routine fixes within an approved contract can be handled by the assigned task owner.

Specify affected/forbidden files, migration/rollback if needed, revised acceptance cases and the validation needed before dependent work resumes. Preserve the original evidence and decision history. On resolution, record actual approval, implementer/integrator and linked changes; never silently rewrite an approved rule to hide the conflict.
