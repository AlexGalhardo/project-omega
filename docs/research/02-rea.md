# Track B — REA capabilities and permitted role

Status: Stage 1 research, 2026-10-23. Governing constraint: [IP policy](../legal/ip-policy.md). Source revisions and URLs: [ledger](sources.md#track-b). No REA command, installation, target inspection, or provider readiness test was performed.

## Findings and evidence

**Verified (documentation):** REA exposes CLI and MCP workflows for native binaries and other application artifacts. Its README describes native pseudocode/disassembly, strings, references and call analysis; static JavaScript/Electron and .NET workflows; browser/runtime observation; and specialized archive/metadata inspection, including Godot PCK [B01]. These advertised capabilities are not permission to inspect Brotato. A package inventory also counts as restricted inspection under our policy.

**Verified:** repository main resolved to `33d00a06010e0d890bb66fc68b91e2653f3c0390` during this research; its package manifest declared `rea-agents` 6.7.0 and Node `^22.23.4 || ^24.22.1 || >=26.12.1` [B05, B06]. The docs distinguish main-branch features from released npm functionality [B02]. **Unknown:** current npm dist-tag and the locally installed providers. Version 6.7.0 here is a manifest observation, not an installation recommendation or a claim that every main feature is released.

| Backend / route | Verified documented boundary | Limitation / trade-off |
| --- | --- | --- |
| Ghidra | Existing Ghidra 12.1.x; documentation reports checks with 12.1.4 and JDK 21; Linux/macOS x64 and arm64, plus experimental Windows x64 [B02]. | Host-matching native decompiler and full JDK required. REA does not install Ghidra or Java. Support depends on format and host. |
| Windows Ghidra P0 | Windows 10+ x64, fixed local NTFS volumes, restricted native PE targets and explicit raw 6502; 26 read-only operations [B03]. | Not general provider parity. GUI controls, annotation, and other format/architecture combinations are not implied. A Brotato-specific result has not been demonstrated. |
| Hopper | Existing commercial application; optional installer on documented macOS/Linux hosts [B02]. | No Windows installer; vendor license/demo restrictions apply. UI/session lifecycle and launch dialogs may obstruct unattended work. |
| IDA | Adapts an existing upstream MCP integration; attached and headless modes have separate contracts [B04]. | License, tool availability and live capabilities matter; identical version strings can expose different tools. No universal OS or decompiler parity is established. |
| Static JS/.NET and metadata routes | Some inspections need no native decompiler [B01]. | This does not make native game logic recoverable as readable source. Target-specific capability must be checked. |
| Runtime observation | Documented workflows execute/interact with targets using user permissions [B01]. | Side effects and captured sensitive data differ from static inspection. No runtime analysis is authorized here. |

## MCP setup, if a future approved question requires it

**Verified:** documented setup registers a local MCP server and matching workflow instructions, previews configuration changes, and pins package-runner registrations. Manual registration invokes the REA entry point with `mcp`, plus provider-specific environment settings [B02]. Ghidra uses `GHIDRA_INSTALL_DIR`, an appropriate `JAVA_HOME`, and provider selection; IDA uses `REA_IDA_MCP_CONFIG` [B02–B04]. Windows launch syntax differs from POSIX syntax.

**Recommendation:** after the policy gate, choose a released version, read its matching docs, use a project-scoped registration outside committed credentials/configuration, and run the documented scoped readiness check against an authorized synthetic fixture first. Record tool, backend, JDK and host versions. Do not install automatically as part of game bootstrap. The actual config remains deferred until a concrete host and question exist.

## What inspection can and cannot establish

**Inferred from documented outputs:** static inspection can distinguish hidden branches, constants, intermediate rounding and execution paths that ordinary play may not separate. It can suggest where to design a discriminating gameplay experiment. However, decompiler output is a reconstruction: optimization, dynamic dispatch, scripts, stripped symbols and unavailable runtime inputs can leave behavior unresolved. Native engine code may not contain the game-specific rule being sought.

It cannot establish legal permission, original source fidelity, intended design, fun, balance quality, cross-version equivalence, complete runtime reachability, or that an apparent constant affects the observed mode. A call graph alone does not prove a path executes. No output is automatically a game requirement.

## Evidence workflow

**Recommendation; origin: original design:**

1. **Question:** identify a decision that changes our design; record why the answer matters.
2. **Public sources:** cite versioned developer material first; retain conflicts and gaps.
3. **Gameplay observation:** record ordinary-play conditions, build, repetitions and uncertainty.
4. **Hypothesis:** write competing behavioral explanations and a discriminating test.
5. **Approved REA check:** only after the owner records EULA/law review and explicit question/operation scope. Stop if protection bypass or out-of-scope data is needed.
6. **Cross-check:** compare the proposed behavior with independent observations; disagreement remains unresolved evidence.
7. **Original requirement:** describe desired behavior in original words; mark origin and confidence. A finding may be deliberately rejected.
8. **Spec:** link the approved decision and requirement to precise inputs, units, boundaries and failure behavior.
9. **Test:** use original fixtures with expected results derived from the approved rule, not extracted game data.

Retain only policy-compliant behavioral summaries in the project. A finding record should include question ID, target identity/build, tool versions, authorization reference, method, result, limits, cross-check and downstream requirement links. Restricted intermediate material must not be fed to implementation agents as reusable code.

## Security, alternatives, recommendation

**Verified:** local analysis results are returned to the agent, whose model provider has separate data policies [B01]. The IDA guide also warns about a native bridge release binding beyond loopback [B04]. **Recommendation:** use isolation, least-privilege filesystem/network access, pinned tools, no project secrets, and explicit retention boundaries if inspection is approved. Untrusted binary strings and generated tool text never become instructions.

Public sources plus ordinary play are the default: less setup and contamination risk, with less certainty about hidden formulas. Scoped static inspection can resolve specific ambiguities, with legal, tooling and interpretation costs. Designing an original, transparent rule often eliminates the need to reproduce an unknown formula at all.

**Recommendation:** defer REA indefinitely unless an important approved question survives cheaper evidence gathering. It is neither an engine dependency nor a 1.0 prerequisite.

## Risks and unknowns

**Unknown:** owner's legal review, applicable EULA/law, authorization, selected Brotato build, supported provider/target combination, release availability of main features, and actual local readiness. **Inferred risk:** tool exploration can become an unrelated project and can reveal material that is prohibited from reuse. No pending architecture decision currently requires binary inspection.
