# Track F — Replaceable SVG asset pipeline

Status: Stage 1 research, 2026-10-23. Sources: [ledger](sources.md#track-f). No SVG asset was created, rasterized, displayed or benchmarked. The following is a pipeline recommendation, not the Art Bible or an implementation spec.

## Findings and evidence

**Verified:** resvg supports static SVG rendering through a Rust library and CLI, separates parsing/simplification through usvg, excludes scripting and animation, and documents feature support limits [F01]. **Verified:** Bevy supports image-backed sprites, atlases and an explicit rendered size independent of source image dimensions [C07, C23]. These capabilities allow SVG authoring without requiring runtime vector rendering.

**Verified:** `bevy_svg` uses usvg plus Lyon tessellation into Bevy meshes and describes SVG support as basic [F02]. `bevy_vello` offers a compute-centric vector integration with optional SVG support and documented limitations [F03]. At the revisions consulted, both manifests depend on **Bevy 0.19**, not 0.20 [F04, F05]. **Unknown:** an available, tested 0.20-compatible release or patch. Do not add either to a 0.20 project based only on a README badge. We have not tested their actual rendering correctness or performance.

## Alternatives and trade-offs

All comparative cost/performance statements below are **Inferred**, not measurements of this project.

| Approach | Advantages | Costs and risks | Fit |
| --- | --- | --- | --- |
| Runtime vector tessellation / vector renderer | Zoom and procedural path changes without pre-exporting every scale; authoring maps closely to shapes. | SVG subset differences, renderer/plugin migration, tessellation/compute cost, material/order interactions, GPU/browser constraints. | Justified by an actual dynamic-vector requirement, not merely SVG source files. |
| Offline SVG to PNG, then Bevy sprites | Build-time validation; ordinary Bevy rendering at runtime; direct path to future pixel-art textures. | Export sizes, rebuild step, texture memory, sampling artifacts at excessive zoom. | Best provisional default for a bounded arena camera. |
| Runtime rasterization with cached textures | Can accommodate authored SVG changes or unknown display scale at runtime. | Parser in shipping build, loading latency, cache invalidation, memory limits, font/resource reproducibility. | Consider only when runtime authoring/scaling is required. |
| Hybrid sprites plus vector UI/effects | Restricts vector flexibility to the few places that benefit. | Two rendering paths and more platform/regression tests. | Escalation if offline textures demonstrably fail a requirement. |

**Recommendation:** offline rasterization with a pinned resvg tool, followed by native Bevy images/sprites. Keep the rasterizer in the asset build workflow rather than every game build unless regeneration is needed. Do not require Node bindings, a paid art service, a custom renderer or a Bevy fork. Exact tool release and installation method remain to validate on the chosen developer OS.

## Proposed asset boundary

**Recommendation; origin: original design:** game definitions refer to stable visual IDs, never SVG paths or atlas coordinates. A presentation catalog maps each visual ID to an asset handle, pivot, rendered logical extent, animation clips and orientation data. Collision shapes, attack ranges, movement speeds, pickup radii and simulation timing live in gameplay definitions.

An image's pixel dimensions and SVG `viewBox` must not determine a hurtbox. A larger replacement image changes texture density, not world size. Decorative overshoot can extend beyond the logical body without changing damage. Animation listens to simulation phases; the number of art frames cannot decide attack activation or invulnerability duration. Direction count and mirroring are presentation choices unless a mechanic explicitly requires directional collision.

Proposed transformation chain:

`SVG authoring coordinates -> export pixels -> pivot/extent metadata -> presentation transform -> camera`

`gameplay logical coordinates -> collision and simulation` remain independent. Stage 3 must define actual coordinate direction, anchor convention, camera scale and visual/readability requirements. A single explicit conversion should handle any SVG-down/world-up axis difference; do not spread negative scales through game code.

## Authoring and build recommendations

- Use explicit view boxes, predictable bounds and named pivots; validate empty or inconsistent dimensions.
- Start with simple paths, fills and strokes. Add gradients, masks, clipping or filters only after representative exports pass. Do not assume browser SVG behavior equals the selected exporter.
- Disallow scripts, network references and undeclared local resource dependencies. Pin fonts or convert decorative lettering to paths; use Bevy text for accessible/localizable UI copy rather than embedding it in artwork.
- Record source asset, creator/license, exporter version, export scale/settings and output hash. A tool's reproducibility claim is not proof that unpinned system fonts produce identical project outputs.
- Keep editable source separate from runtime outputs. Decide whether to commit generated textures based on checkout usability and measured CI cost; either way, regeneration must be documented and reproducible.
- Measure atlas benefit before introducing packing complexity. If adopted, define padding/extrusion, alpha handling, sampling, stable metadata and mip/bleed tests. Never identify content by a fragile atlas index.
- Preserve one canonical metadata schema. Avoid duplicate pivot/size settings in source art, gameplay data and renderer code.

## Future pixel-art replacement

**Recommendation:** supply a different visual catalog and raster assets under the same visual IDs. Pixel art may need nearest sampling, camera snapping and altered animation cadence, so replacement is not automatically visually seamless. Those settings belong to presentation. Keep interpolation/camera policy from changing authoritative collision positions. A hybrid transition can replace one character first without re-authoring combat rules.

PixelLab is an optional future source of textures, and ElevenLabs is an optional future audio source. Neither service was invoked or required. Licensing, pricing, output rights, style continuity and acquisition records must be assessed if selected; current rights or costs are **Unknown**. See [IP policy](../legal/ip-policy.md) for provenance rules.

## Feasibility experiment proposed — not executed

**Recommendation:** after authorization for experiments or approved implementation gates, build a minimal comparison using original placeholder art: one character, one projectile, one effect and one HUD icon. Test several display scales, transparency, pivots and the SVG features actually used. Record exporter settings, errors, image dimensions, load behavior and memory.

Then replace the character with a differently sized raster sprite/animation through the catalog alone. Feed identical tick inputs and compare authoritative positions, collision outcomes and event ticks. Acceptance should require identical gameplay outcomes, while visual approval checks silhouette alignment and readability. A broken pivot or changing hitbox disproves the boundary. This experiment is not evidence of performance until it is run.

## Risks and unknowns

**Unknown:** approved aesthetic, resolutions, zoom range, target GPU, animation complexity, atlas need, exporter version and asset-production time. **Inferred risks:** overly elaborate SVGs inflate authoring/export cost; custom vector renderers add upgrade burden; changing art style late still requires art and QA effort even with a correct software boundary. Prefer offline textures unless the selected style needs more.
