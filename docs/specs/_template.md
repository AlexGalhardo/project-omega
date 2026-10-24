# Specification authoring template

Status: APPROVED authoring contract, 2026-10-24. This is the reusable template, not an implementation spec or a blank completed artifact. Create a real spec by filling every field below with concrete values; do not submit headings alone. Use original prose and the scenario/requirement/outcome conventions inspected in E4-14, without installing an agent framework.

## Identity and authority

Assign an unused SP ID and descriptive title. Record lifecycle status, date, exact prerequisite spec/contract IDs, linked GDD requirement/acceptance IDs, relevant Art Bible IDs and ADR links. Distinguish approved rules from proposed changes. Include origin/evidence and known limits. List critical open decisions explicitly; any such decision prevents READY.

## Problem, outcomes and exclusions

Explain the player/engineering problem and observable result in a short paragraph. State requirements using measurable outcomes. Name exclusions and boundary cases; do not use phrases such as "works correctly" or refer to unavailable conversation context. Link canonical formulas/catalogs instead of duplicating them.

## Technical design

Name components, resources, owned data, public interfaces, systems/schedules, events/queues, ordering/flush boundaries and data flow. For pure tools or functions, explicitly state that ECS systems are not applicable and define input/output/error contracts. Identify serialization/version behavior, failure modes, numeric units, timing, determinism assumptions and relevant performance measurement needs. Do not claim unverified upstream APIs.

## Acceptance and validation

Give stable SP-ID-ACn criteria in Given/When/Then form, including normal/rejected/boundary cases. Map each criterion to a named future test or native procedure with expected results. Specify exact package/feature/test commands and setup/fixtures. Commands are planned until actually run; state missing tools/hardware. Include appropriate unit, integration, invariant, headless or native checks rather than blindly requiring every category.

## Ownership and handoff

List allowed and forbidden file paths. Task assignment narrows them to exact files/globs and identifies a serial owner for shared registrations/manifests/contracts. Provide reviewable slices when the feature is larger than one change. Each future task adds its stable roadmap ID, objective, dependency contracts, required tests, acceptance mapping, validation commands, report format and escalation conditions. No task is READY with an unresolved critical choice or conflicting file owner.

## Completion evidence

Apply the canonical Definition of Done and quality gates. Start with a truthful NOT IMPLEMENTED statement. Later record actual commit/environment, exact commands and exit codes, criterion-to-test mapping, native screenshots/logs when needed, limitations, change requests and reviewer/integration outcome. Never populate evidence with expected output or mark DONE merely because prose or code exists.
