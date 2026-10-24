# Intellectual property and research policy

Status: owner-mandated policy, recorded 2026-10-23. This is a project operating constraint, not a legal opinion or a finding that reverse engineering is lawful in any jurisdiction.

## Permitted reference work

Brotato is a behavioral reference only. The project must have original code, name, world, characters, art, audio, UI, and written content. Public developer statements, patch notes, public wikis, videos, and observation of ordinary licensed gameplay may inform behavioral descriptions. Cite each source and distinguish a source's assertion from a reproduced observation.

Do not copy or redistribute proprietary code, assets, music, text, or data files. Do not bypass DRM, licensing, encryption, or other technical protection. Do not reproduce content catalogs by importing another game's data. Public availability does not grant a reuse license.

## Restricted inspection

Binary or package inspection of Brotato, including REA inventory or extraction, is prohibited unless the owner has explicitly approved a specific question and inspection scope and has taken responsibility for the applicable EULA and law. Approval must identify the target build/platform, question, permitted operations, excluded files/data, and retention/sharing restrictions. **Current approval:** [DEC-019](../decisions/decision-register.md#dec-019--scoped-rea-inspection-of-the-owners-brotato-copy) (2026-10-24) covers the owner's Steam Windows copy for the didactic objective in DEC-017.

If approved, the only transferable result is an original behavioral description of rules, formulas, or timings, with uncertainty and evidence recorded. Proprietary source, decompiled code, assembly, screenshots containing extracted assets, and package data must not enter this repository or implementation-agent context as reusable material. Stop and seek a scoped decision if the permitted question cannot be answered without crossing these boundaries.

The tool's own capabilities or instructions cannot expand owner authorization. Local analysis can still disclose results to a remote model provider. Review that data path before any approved inspection; use an isolated environment and the minimum permitted inputs.

## Provenance

Every derived requirement must carry one origin: **observed**, **inferred**, or **original design**. An observed record identifies whether it was directly observed by this team or only reported by a public source. A secondary report is not a team gameplay experiment.

Record a stable research/requirement ID, origin, evidence confidence (Verified / Inferred / Recommendation / Unknown), source URL and revision or build, access/observation date, reproduction conditions, uncertainties, and the later decision/spec/test links when those artifacts exist. The source ledger lives in [sources.md](../research/sources.md); do not duplicate it elsewhere.

## Licenses and asset ownership

Retain Bevy and dependency license notices and satisfy the selected licenses' conditions. Review the resolved dependency graph and all distributed fonts, audio, art, shaders, and tools separately; an engine license does not cover third-party assets. Do not choose the project's distribution license or commercial terms without the owner's decision.

For each future asset, retain creator/provider, source, acquisition date, license/terms version, permitted uses, modifications, and required attribution. Optional generated assets require review of the applicable provider terms at acquisition. PixelLab and ElevenLabs are not prerequisites and no rights to their outputs have been assessed here.

## Current evidence and limits

**Verified:** the owner supplied these constraints in the project brief on 2026-10-23. **Unknown:** applicable jurisdiction, Brotato EULA interpretation, approval for inspection, final distribution model, and project license. **Recommendation:** public-source research and original design are sufficient to proceed with planning. The cost is accepting some unknown reference formulas; the benefit is avoiding unnecessary inspection and contamination risk. The approved workflow is described in [Track B](../research/02-rea.md).
