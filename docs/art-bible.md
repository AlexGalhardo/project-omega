# Project Omega — Art Bible

Status: **APPROVED explicitly by the owner on 2026-10-24.** Revision: 1. This document describes production and presentation requirements; no assets, exports, fonts, audio recordings or renderer experiments have been produced by writing it. Documentary checks are recorded in GDD-020.

## 1. Authority, evidence and visual purpose — ART-001

**Verified owner direction:** scrap robots in an industrial arena; stylized vector art with clear silhouettes and contrasting industrial colors. Source: [decision register](decisions/decision-register.md), consolidated in [revision 6396617](https://github.com/AlexGalhardo/project-omega/tree/6396617), accessed locally 2026-10-23; Stage 2 approved by the owner on 2026-10-23. The [GDD](gdd.md) is canonical for gameplay geometry, timing, input and UI behavior. This Art Bible must never silently change it.

**Recommendation / original design:** all palettes, frame extents, production rules and animation/audio choices below are proposed original direction for approval with Stage 3. Numbers marked P are provisional production/readability values, not measured limits. **Unknown:** actual SVG-export fidelity, memory cost, production throughput and human readability. Research comparisons and their source revisions/access dates are in [Track F](research/05-asset-pipeline.md) and the [source ledger](research/sources.md#track-f). No new Bevy/vector API or service-license claim is made here.

Visual pillars:

- **Built from found parts:** asymmetric plates, bolts, exposed joints and a clearly visible reactor; readable at play scale.
- **Useful color and shape:** faction, danger, currency and power allocation can be understood without relying only on hue.
- **Mechanical motion with restraint:** short recoil, hinge turns and rotation communicate action without obscuring threats.
- **Replaceable presentation:** stable visual IDs and logical extents make SVG-derived textures one presentation option.

Avoid close imitation of Brotato characters, UI arrangement, icons, text or assets. Use original silhouettes and names under the [IP policy](legal/ip-policy.md). The theme does not require crafting, destructible scenery or complex articulated rigs.

## 2. Palette, materials and shape language — ART-002

This palette is an original recommendation P. It supplies semantic tokens; exact hues may change after readability review while token meaning remains stable. Do not tint all enemy/projectile silhouettes to the same unoutlined mass.

| Token | sRGB hex | Role | Shape/contrast reinforcement |
| --- | --- | --- | --- |
| `floor_deep` | `#151D27` | Arena floor | Low-detail large panels; no fine grids. |
| `panel` | `#202A36` | HUD/shop panels | Opaque backing behind text. |
| `metal` | `#657382` | Neutral structural metal | Broad planar facets and seams. |
| `text_primary` | `#F2F5F7` | Main text and player body outline | Stable high-contrast edge. |
| `text_secondary` | `#A8B6C6` | Supporting text | Never communicate disabled/locked state by opacity alone. |
| `player_energy` | `#62DFD0` | Reactor, friendly shots, allocation | Circles/rounded pips and round projectile cores. |
| `hostile` | `#FF6B5E` | Enemy attack cores and damaging projectiles | Diamond or pointed silhouettes with dark edge and bright inner mark. |
| `warning` | `#FFB454` | Pending attacks and warnings | Dashed/hatched outline, inward progress marks. |
| `scrap` | `#E8CC73` | Currency | Hexagonal nut silhouette with a hole. |
| `focus` | `#9EC5FF` | Keyboard/controller focus | Double outline plus explicit focus marker. |

Target normal UI text contrast of at least 4.5:1 against its actual backing as an internal design criterion, not a claim of complete accessibility compliance. Use primary/secondary text on opaque panel backgrounds; never place important text directly over changing combat. Selection, locked, purchased and unaffordable states also have labels/icons. Red/green comparison arrows include signed numeric changes and directional symbols.

Metal uses two flat tones plus one edge highlight. Rust is a sparse orange/brown accent, not a high-frequency texture over every surface. Use matte floors and stronger actor outlines; limit decorative hazard stripes to arena boundaries and clearly noninteractive fixtures. Avoid decorative floor markings that resemble live attack warnings. No essential information uses shader bloom, transparency alone or a rapidly flashing emissive layer.

## 3. Character, enemy and weapon silhouettes — ART-003

The GDD's [character catalog](gdd.md#91-character-catalog), [weapon catalog](gdd.md#92-weapon-catalog) and [enemy catalog](gdd.md#101-common-enemy-catalog) own identities and mechanics. The following is a visual assignment, not additional content.

| Character | Visual silhouette | Identifying feature |
| --- | --- | --- |
| Patch | Compact square chassis with one clipped corner | Prominent repair plate and round reactor. |
| Coil | Round chassis with two unequal side coils | Paired circular housings and thin connector band. |
| Rivet | Broad trapezoid chassis | Heavy front plate and triangular bolt arrangement. |
| Skid | Narrow kite-like chassis | Two long side runners and offset antenna. |

All four have identical gameplay hurtboxes from GDD-004. Main body silhouettes sit around that circle; broad appearance cannot imply a different collision size. A continuous, readable player rim and reactor marker remain visible under weapon decorations. Avoid outlining a false square collision boundary for Patch; rounded body corners and the body rim communicate the shared footprint.

| Enemy | Main shape | Attack-readable accent |
| --- | --- | --- |
| Scrapper | Small toothed disk | Forward eye/notch shows pursuit. |
| Wire Rat | Thin arrow wedge | Long rear wires suggest speed; wires are decorative. |
| Plate Hauler | Wide block with rounded plate rim | Stacked armor slabs indicate durability. |
| Rivet Guard | Short barrel on octagonal body | Barrel turns with locked windup direction. |
| Ram Cart | Rectangular front plow | Paired rails align with charge lane. |
| Spark Hub | Stationary ring with eight spokes | Spokes align with radial emission directions. |
| Survey Eye | Tripod-like body with single long lens | Narrow lens and lane cue differ from a projectile gun. |
| Shard Hopper | Fan-shaped front | Three visible ports match spread shot. |
| Clamp Walker | Wide U-shaped frame | Paired hammer arms cue the ground strike. |
| Coil Skater | Crescent chassis | Offset ring indicates clockwise circling. |

The Press uses a broad piston frame with a central stamp; The Foreman uses a circular distributor chassis with radial ports and an asymmetric supervisory mast. Both must read as larger than common enemies without depending on red color. They use precisely the GDD body radii; no decorative plate is a second damage source.

Weapon identity uses a distinct small silhouette: Bolt Driver short single barrel; Slug Press thick square muzzle; Needle Spitter narrow multi-fin barrel; Chain Saw toothed rail; Drop Hammer square head; Rivet Fan three-port spread; Arc Rail paired long prongs; Nail Lance spear barrel; Scrap Rocket canister tube; Pulse Drum concentric disk; Slag Lobber bent pipe; Rotor Disk circular cutter. Their silhouettes, shop icons and projectile cues must agree, but slots are decorative mounts: authoritative attacks originate at player center as specified in GDD-009.

Passive items use the forty GDD names as icon briefs. Reuse a consistent frame/style, but do not reuse identical pictures for different active shop offers. A 64-pixel source icon must still distinguish basic category (plate, gear, optics, repair, collector) at the smallest approved shop size. Tooltip text carries the exact effect; icons do not encode hidden stats.

## 4. Coordinates, size and pivot contract — ART-004

Gameplay space: LU, +X right, +Y up, fixed arena and camera in [GDD-004](gdd.md#5-space-movement-and-input--gdd-004). SVG space: +X right, +Y down. Author all frames facing right (+X). A single presentation transform converts SVG-down to world-up; gameplay code must not infer direction from texture orientation.

Author the pivot at the logical body center, not at the lowest visible pixel. Metadata records pivot in normalized SVG coordinates from the top-left: `(0.5,0.5)` for symmetrical framing. Pixel-to-world mapping for source point `(x,y)` in view box `(vx,vy,vw,vh)` is:

`local_x = ((x-vx)/vw - pivot_x) * logical_width_lu`;

`local_y = (pivot_y - (y-vy)/vh) * logical_height_lu`.

Apply simulation facing rotation and authoritative position after this mapping. Sprite trimming or atlas packing must preserve the original untrimmed coordinate frame/pivot in catalog metadata. Automatic trimming is disabled initially. Horizontal flipping must not move the pivot or alter gameplay facing. UI icons use a separate UI layout and do not set world extents.

| Asset family | SVG viewBox P | Logical rendered extent P | Required alignment |
| --- | --- | --- | --- |
| Player body | `0 0 64 64` | 40 x 40 LU | Central core diameter 28 LU matches the GDD hurtbox diameter; antennas/wires may occupy the remaining 6 LU per side. |
| Common enemy body | `0 0 64 64` | Square side `body diameter + 12 LU`, authored into metadata per ID | Central combat body rim follows the GDD radius; spare border is decorative. |
| Boss body | `0 0 128 128` | Square side `body diameter + 24 LU`, authored per ID | Peripheral pipework is decorative; main damaging body remains recognizable. |
| Weapon decoration | `0 0 64 64` | 24 x 24 LU | Mount is cosmetic, never moves the authoritative attack origin. |
| Shop/item icon | `0 0 64 64` | UI layout determines size | No world-space use of UI pixel size. |
| Projectile | `0 0 32 32` | Authored to show the GDD collision core at exact logical diameter | Decorative tail may extend behind the core; tail never indicates collision. |
| Ground warning / attack shape | Geometry supplied from simulation | Exactly the authored area/lane | Draw procedural rings/lanes from authoritative coordinates; do not scale a generic texture by guessed sprite bounds. |

These body-to-art relationships are **authoring checks**. The runtime simulation never calculates its radius from these metadata values. For example, the player 64-unit SVG maps to 40 LU, so its 28-LU body core spans 44.8 SVG units, centered at `(32,32)`; swapping a 128-pixel texture into the same logical extent leaves the hurtbox unchanged. Use a development overlay to inspect body-core alignment, not a user-facing collision debug display.

Weapon decorations occupy four consistent diagonal mounts around the chassis. Decorative mounts may be hidden/reduced when crowded. A short energy trail can connect the mounted weapon to its true emission origin; projectile spawn, explosion and beam/capsule edges must match simulation even if a barrel drawing suggests another origin.

## 5. Animation and gameplay timing — ART-005

Author compact separated layers (body, moving tool, reactor, optional tread/rotor) only when they reduce asset work. No skeletal rig, custom animation engine or direction-specific sprite set is required by the initial style. Top-down bodies rotate smoothly in presentation toward current movement or aim; actual aim comes from simulation. At zero movement retain last facing; do not animate uncommanded collision rotation.

Suggested cosmetic cycles P: idle reactor pulse 1.2 s; locomotion tread cycle 0.4 s scaled by movement speed; recoil settle 0.12 s; accepted-hit body tint 0.08 s; defeated-enemy fade/debris 0.25 s. These cosmetic durations never create hit windows, invulnerability or collision. Reduced-flash mode replaces the tint with a brief static outline rather than a full-body flash. Death removes the authoritative entity immediately; a noninteractive presentation remnant may finish its fade.

Authoritative attack timings are read from [GDD-005](gdd.md#6-simulation-and-attack-semantics--gdd-005), [weapon data](gdd.md#92-weapon-catalog), [enemy attack data](gdd.md#101-common-enemy-catalog) and [boss schedules](gdd.md#102-boss-catalog-and-attack-schedules). Do not copy their numeric timings into an independently editable animation timeline. A phase can use normalized progress `elapsed_ticks / phase_ticks`; completion occurs because the simulation changes phase, not because the animation ends.

| Simulation fact | Presentation obligation |
| --- | --- |
| Attack windup begins | Show the correct area/lane or a muzzle-direction cue and progress; follow the locked/following origin policy from the attack. |
| Windup progresses | Animate mechanical compression/charge and progress marks; never prematurely show an active damaging core. |
| Attack emits | Draw hit area/projectile at authoritative origin and orientation on that tick; play emission cue if enabled. |
| Recovery | Release the compressed pose; no new damage marker unless a new attack emits. |
| Attack cancelled by death | Remove its windup cue immediately; emitted projectiles remain as directed by the simulation. |
| Incoming damage accepted | Update HP and local hit feedback once; rejected invulnerable overlaps may show a muted contact cue but no false HP loss. |
| Wave ends / pauses | Clear combat remnants on wave exit; freeze gameplay-linked pose/progress while paused. UI focus can still animate. |

The rate formula includes windup plus recovery minimums, so art must not lengthen those phases. A sprite clip with fewer frames may hold or interpolate; a clip with too many may skip decorative frames. Neither can delay the next legal attack. A reduced-motion setting may simplify cosmetic animation but retains directional and timing information.

## 6. Telegraphs, VFX and readability — ART-006

Pending danger uses dashed/hatched amber boundaries; active hostile projectiles use solid pointed cores with bright inner marks and dark outlines. Friendly projectiles use round/rounded cyan cores; scrap uses a gold hexagon. Shapes remain distinguishable in grayscale. Every live hostile projectile's visible core covers its full collision circle, with decorative tail outside it. Foreground effects must not conceal the player rim or hostile cores.

Ground circles show the actual damaging radius, not a larger decorative ring with an ambiguous inner boundary. Capsules show the exact damaging lane width and rounded ends. The player's hurtbox extends into these shapes during collision, as defined by the GDD; the UI must not imply safety merely because the player center lies outside the ring. Spawn warnings identify the pending enemy silhouette and the arrival progress. Postponed spawn warnings stay active with the original center and a visible hold indicator; they never show a completed countdown while invisible damage can occur.

The GDD distinguishes a ground strike with a locked world center from a beam whose origin follows its displaced attacker. Telegraphs consume those facts directly. Charge cues extend along the current predicted path to its time/distance limit or wall, using the moving body's width; they are cues for a swept body, not independent beam damage. Once charging starts the body and brief directional trail identify the active danger. Decorative sparks cannot be mistaken for additional projectiles.

Draw order, from back to front:

1. Arena floor and low-contrast decorations.
2. Ground warnings and area boundaries, with an additional top-layer boundary if actors would obscure them.
3. Scrap pickups and shadows.
4. Actor bodies and weapon decorations, sorted consistently by logical Y then stable visual-instance ID where needed.
5. Live projectile cores and essential hit/charge indicators.
6. Minimal damage text, player identifying rim and boss targeting indicators.
7. HUD, menus and opaque modal backings.

Outline sizes are evaluated in screen space at approved display scales. Start with 2 physical pixels for critical projectile edges at 1280x720 (P visual target); if zoom/scale changes later, re-evaluate rather than silently shrinking them. VFX quantities are authored for clarity; particle budgets and batching techniques follow measured profiling, not this document.

Reduced effects removes trailing sparks, decorative fragments, floor flashes and excessive damage text while retaining warnings, attack cores, HP changes and spawn cues. Screen shake defaults off under GDD-015; an optional enabled shake is presentation-only and must preserve aiming conversion relative to the actually rendered camera. Do not move the authoritative world or change camera coverage to create shake. For initial implementation, keeping shake absent is acceptable; any later introduction needs aim/readability validation.

## 7. Camera, HUD, UI and typography — ART-007

The camera extent and aspect policy are canonical in [GDD-004](gdd.md#5-space-movement-and-input--gdd-004). The floor fills the arena rectangle; wall decoration can sit outside it only if it does not change what is visible inside the playable area. Letterbox bars are noninteractive for aiming. Camera treatment does not imply movement beyond a wall or offscreen spawns.

Baseline UI design canvas P: 1280x720 logical screen pixels. Use scalable panels, anchors and minimum readable text rather than fixed placements that overflow at 150% UI scale. Arena and UI scales are independent. The GDD owns exactly which information/actions must be available. Proposed arrangement:

- Combat: HP upper left; wave/timer centered above; scrap upper right; four weapon/energy cells along the bottom; boss bar below wave/timer when present. Keep the player's complete arena visible behind the HUD and keep opaque panels compact.
- Shop: title and scrap balance at top, four offers in a two-by-two grid, inventory/selected comparison adjacent, four-slot allocation strip below, Refresh and Start as separated bottom actions. At large UI scale, comparison/detail panels may scroll or open as a focused modal without hiding costs or the Back action.
- Confirmation: name the old and new weapon/tier, lost investment, price and resulting energy; Cancel receives safe initial focus. Do not put a destructive replacement behind the same unreviewed click that selected the offer.
- Settings: grouped readable controls for input, display, audio and effects; Reset bindings and Reset profile are distinct actions. Error/unsaved status has text, icon and an action.

Typography recommendation: one bundled legible sans-serif font with audited distribution rights and a full Latin character set. Exact font file/license is not selected or claimed. Do not depend on the user's installed system font. P baseline sizes at 100%: body 20 logical pixels, supporting labels 18, major values 24 and screen titles 32. Do not shrink text below 18 to fit; wrap, widen or scroll. Use tabular numerals if supported, otherwise reserve stable numeric widths. Avoid all-caps paragraphs and decorative stencil fonts for gameplay values.

UI text remains live text, not rasterized SVG lettering. Font files and notices go through the same provenance record as other assets. A temporary font used for a prototype must not be mistaken for cleared production typography. Locale-specific font expansion is deferred with localization scope.

## 8. SVG authoring and offline pipeline — ART-008

**Recommendation:** author SVG and rasterize offline using a pinned resvg exporter, then use ordinary raster presentation. This follows [Track F](research/05-asset-pipeline.md), whose findings are versioned in the ledger; exact exporter version and Windows invocation are still unverified and must be chosen in Stage 4/Foundation. Do not invent a working export command or add an unverified runtime vector dependency here.

### 8.1 Source rules

- UTF-8 SVG with explicit viewBox and width/height matching its aspect ratio. Use paths, basic shapes, solid fills and strokes initially. Expand strokes that produce inconsistent raster bounds during a verified export test.
- Use the specified source frames and right-facing orientation. Frame IDs/pivots must be stable across animation frames. Keep the same viewBox for every frame of one clip.
- No scripts, foreign objects, embedded HTML, network references or undeclared external images/fonts. Reject external resource references before invoking the exporter. Avoid filters, masks, gradients and clipping until a representative checked export proves a concrete need and acceptable output.
- No live text inside gameplay sprites/icons. Decorative insignia may be original paths, but UI labels remain text. Source filenames and layer names are English.
- Keep source shapes inside declared bounds including strokes, or explicitly enlarge the frame and update presentation metadata. No accidental negative crop or undocumented pivot correction.
- Record creator, origin and rights before accepting an asset. Placeholder status does not waive provenance. No proprietary reference-game asset enters the asset tree.

### 8.2 Planned directories and naming

These paths are a production proposal to be finalized with Stage 4 file ownership. No directories or assets need to be created merely to satisfy this document.

| Path | Purpose |
| --- | --- |
| `art/source/svg/<category>/<asset_id>/<clip>_<frame>.svg` | Editable original sources, e.g. `characters/char_patch/idle_00.svg`. |
| `art/source/audio/` | Original/editable audio projects and source records, excluding unlicensed material. |
| `assets/generated/sprites/` | Runtime PNG outputs, grouped by visual ID. |
| `assets/generated/audio/` | Runtime audio files after the chosen encoding/conversion workflow. |
| `assets/fonts/` | Selected bundled font and notice files. |
| `assets/presentation/` | Canonical visual/audio catalogs and export manifest, format chosen in Stage 4. |
| `docs/legal/asset-provenance/` | Acquisition/license evidence indexed by asset ID. |

Asset IDs are lowercase snake_case and stable. A gameplay ID such as `char_patch` maps to `visual_char_patch`; `wpn_bolt` to `visual_wpn_bolt`; an item icon uses `visual_item_plating`. Shared projectile/VFX assets have separate IDs such as `visual_projectile_friendly_round` or `visual_warning_circle`; sharing an image never merges gameplay identities. Files use zero-padded frame numbers; frame order is explicit metadata, not filesystem enumeration.

### 8.3 Export process and manifest

1. Validate source SVG allowlist, bounds, IDs, resources and provenance. A failed source produces a readable error and does not overwrite the last valid output.
2. Choose the asset's logical extent from the visual catalog and the approved export density. Initial candidate P: four texels per LU for world sprites; icons export at the largest required UI size for the initial layouts (192x192 pixels for 64-unit sources is a candidate). Texture resolution is not collision size or a proven memory budget.
3. Rasterize with the pinned exporter, explicit dimensions and transparent background; write into a temporary output location. Do not use machine-local font lookup or network data.
4. Validate PNG dimensions, alpha, expected frame count, manifest mapping and representative appearance, then replace outputs as one successful build step. Preserve SVG source and hashes.
5. Record source hash, output hash, exporter version/binary identity, settings, logical extent, pivot, sampling and generation revision in the export manifest. Changes must be reviewable; a successful process exit alone does not establish visual correctness.

For world sprites, compute export width/height as `ceil(logical_extent_lu * texels_per_lu)` independently of the SVG source frame dimension. Example: the 40-LU player extent at four texels/LU exports to 160x160, while the logical core remains 28 LU. The build validates equal aspect ratio between viewBox and catalog extent unless a deliberate nonuniform scaling rule is declared; baseline rejects unintended distortion.

Recommendation: commit initial generated runtime assets alongside sources so a normal checkout can load placeholder/production content without installing an exporter. Record outputs as generated and regenerate them from sources, never hand-edit them. Revisit generated-file storage if measured repository growth or CI cost justifies another approach. Git LFS or an external asset service is not a prerequisite and has not been selected.

Default vector-derived sprites use linear sampling and transparent edges. Alpha conventions must be verified in the actual Bevy pipeline before conversion policy is pinned; do not silently premultiply twice. No mipmaps or atlases are required initially. If profiling or packaging evidence justifies atlases, the proposed baseline is 2-pixel edge extrusion plus 2 additional transparent padding pixels at the chosen export density, no rotations and preserved untrimmed pivots; verify bleed at all supported scales before adopting. Packing metadata is generated; stable visual IDs must never expose raw atlas indices to gameplay.

## 9. Presentation catalog and validation — ART-009

The catalog is the canonical source for presentation extent/pivot/sampling/frames, independent from gameplay stats. Its serialization and Bevy handle representation are Stage 4 decisions. Avoid duplicating extent in code, SVG loader options and content data.

| Record / field | Meaning and validation |
| --- | --- |
| `visual_id` | Globally unique stable ID referenced by gameplay, UI or shared effects; no path-derived identity. |
| `source_asset_id`, `provenance_id` | Resolve to source and rights record; generated assets retain their parent source relationship. |
| `logical_extent_lu` | Two positive finite world dimensions for world assets; UI records use an explicit UI layout mode instead. |
| `pivot_normalized` | Two finite values in 0–1, measured from source top-left; validates against untrimmed frame coordinates. |
| `orientation` | Baseline right-facing frame, rotation policy and optional mirroring; no gameplay direction inferred from pixels. |
| `sampling` | Explicit linear or nearest; consistent through source family and chosen skin. |
| `frames` / `clips` | Named frame references and presentation-only cosmetic cadence or simulation-phase binding. Every frame exists; no empty clip. |
| `draw_layer` | Approved semantic layer from section 6, not arbitrary content-controlled renderer priority. |
| `texture_reference` | Runtime file or generated atlas lookup; must resolve and have valid bounds. |
| `fallback_visual_id` | Optional visible placeholder for missing cosmetic data, with no cycles. Missing mandatory catalog schema prevents content acceptance. |

Fallbacks use simple distinguishable silhouettes for player/enemy/projectile/item categories. A cosmetic fallback cannot change collision, invisibly remove a hostile projectile, or conceal a warning. If an essential danger cue cannot be rendered at all, pause and display an actionable content error instead of continuing an unreadable fight. Gameplay-content failures remain the GDD Boot error behavior.

Validation checks references, safe source dependencies, finite positive dimensions, coherent pivots, missing frames, atlas bounds if used, and consistent metadata across skins. Representative visual review checks alignment with development hitbox/attack overlays, scale, occlusion and label readability. This is proposed validation work; no exporter or rendering validation has been executed yet.

## 10. Future pixel-art substitution — ART-010

A replacement skin supplies the same visual IDs with new raster files, metadata and presentation-only sampling/animation choices. Do not change character stats, collision geometry, projectile range, attack ticks or RNG behavior. The GDD content continues to refer to visual IDs rather than source paths. This isolates gameplay rules; it does not eliminate the effort of producing/reviewing new art.

Migration procedure:

1. Replace one character with a differently sized raster source while preserving its logical extent and pivot meaning.
2. Use nearest sampling if appropriate for that skin; choose presentation-only camera/sprite snapping, leaving authoritative LU positions unchanged. A fixed full-arena camera limits integer scaling choices, so letterboxing/display behavior must be reviewed rather than promised as automatically pixel-perfect.
3. Map fewer/more animation frames onto the existing simulation phases; retain the telegraph and damaging-core requirements.
4. Compare identical same-build seed/input sequences with both catalogs. Position, HP, currency and attack tick logs must match; compare visuals separately for alignment and readability.
5. Expand to the remaining catalog only after the single-character proof passes. Skin-wide animation or color changes require art review but do not justify rewriting combat.

PixelLab remains an optional future source of assets. It is not installed, invoked, budgeted or required. Evaluate actual output rights, export workflow and provenance when considering it; no current pricing or terms are asserted by this document.

## 11. Audio direction and optional generation — ART-011

Direction: compact mechanical impacts, rattles, electrical pulses and restrained industrial rhythm. Audio reinforces events already visible; it never supplies the only warning. Avoid constant metal noise that masks critical incoming cues. No voice acting or narration is required for 1.0. All game text and any future spoken content must be original English.

| Event family | Initial production scope P | Required behavior |
| --- | --- | --- |
| Weapon emissions | Six shared categories: light shot, heavy shot, rapid shot, saw/melee, electrical, explosive | Per-weapon pitch/level variations may use the same cleared source; emission follows authoritative attack, not rendered frame count. |
| Hits/deaths | Metal impact, armor impact, player hit, common enemy death, boss death | Player hit distinct from enemy damage; accepted events only for HP-loss cue. |
| Telegraphs | Soft charge, heavy ground-strike warning, boss volley warning | Visual equivalent always present; cancel un-emitted charge cues on attack cancellation. |
| Pickups/shop | Scrap pickup, purchase, denied action, refresh, allocation tick | Group rapid pickups in presentation to avoid a wall of sound; currency calculation remains per event. |
| Interface/results | Focus/confirm/back, unlock, wave start/end, victory/defeat | No duplicate result sound on repeated transition input. |
| Music/ambience | One combat loop, one quieter menu/shop loop, one low arena ambience | Crossfade with state changes; no adaptive score system required. Silence is an acceptable early prototype placeholder. |

Initial sources may be self-created synthetic/recorded sounds or properly licensed third-party placeholders. No chosen clip, license or tool output is claimed here. Record provenance before import; distinguish approved production audio from temporary placeholders. Keep editable sources and runtime outputs separate. Exact codecs, sample rates, voice count, streaming choices and latency constraints require Stage 4 audio compatibility review and profiling; do not invent hardware budgets.

Master/music/SFX settings and mute behavior follow GDD-015. Proposed mix defaults P: master 80%, music 40%, SFX 80%, with 0–100% sliders; master multiplies channel gain. These are preferences, not calibrated loudness targets. Test clipping and critical-cue audibility during dense combat. Looping sounds stop/fade on state exit; pause freezes gameplay-triggered one-shots/loops consistently and may keep low menu ambience. A disconnected output device must not halt simulation or erase settings.

Optional ElevenLabs workflow, only if later selected: define an original sound brief without proprietary samples or an imitation request; inspect the actual service plan/terms and permitted commercial use at acquisition time; record the date, plan, terms reference, prompt, output identifier and subsequent edits; audition and edit the result; import only after rights/provenance review. Do not use voice cloning or upload third-party recordings without specific authorization and rights. Current pricing, ownership terms and eligibility are **Unknown**. The service is neither a requirement nor authorization to spend. Generic placeholders remain the default.

## 12. Provenance and production review — ART-012

Every shipped asset, including fonts, icons, sounds and generated derivatives, has an asset ID, creator/source, acquisition date, license/rights evidence, allowed uses, required attribution, modifications, source/output hashes and placeholder/production status. Repository license and asset licenses are distinct decisions. A missing rights record blocks release of that asset; public availability alone is not a reuse permission. The [IP policy](legal/ip-policy.md) is canonical for project boundaries.

Use original annotated sketches or source SVG diffs for visual review; do not check proprietary reference sprites into a mood board. Research may link public behavioral references using the source ledger. No REA/binary inspection is needed to implement this art direction.

## 13. Acceptance and consistency checks — ART-013

The following are future validation requirements, not completed asset tests. They map to GDD-016 and the design acceptance examples in GDD-018.

| ID | Required evidence |
| --- | --- |
| `AA-01` | Export original player, projectile, warning and icon sources at the documented densities; verify bounds, alpha, pivot and viewBox against the selected pinned exporter. Record actual version/command and errors. |
| `AA-02` | At 1280x720 and 1920x1080 with 100/125/150% UI scales, inspect readable text, focus, costs, clipped panels and navigation through every shop action. Record screenshots and native input results. |
| `AA-03` | Overlay authoritative hurtboxes and attack shapes; show player/enemy collision cores and capsule/circle warnings aligned through windup, displacement, emission and cancellation. |
| `AA-04` | Inspect hostile/friendly projectiles, pickups and warnings in grayscale, muted audio and reduced effects; identify shape cues without relying on hue. Human readability review remains necessary. |
| `AA-05` | Swap one character from SVG-derived PNG to a differently sized pixel sprite using the catalog alone; same-build gameplay traces match DA-20 and visual pivot alignment is retained. |
| `AA-06` | Audit every runtime image/audio/font against a source and rights record; confirm no unresolved temporary-license asset enters the release package. |
| `AA-07` | Rebuild unchanged sources with the pinned pipeline in the supported environment; compare manifests/hashes, investigate differences and do not label it reproducible until proven. |
| `AA-08` | During dense combat, verify that cosmetic audio/VFX reduction leaves warnings and authoritative projectile cores legible; collect actual frame-time/audio evidence separately from visual preference. |

The GDD owns phase durations, hitboxes, camera bounds and control semantics. This document owns authoring conventions, presentation metadata, palettes, visual/audio feedback and production acceptance. Changes that affect both require a recorded coordinated revision. Stage 3 approval does not mean any of the checks above have passed; implementation, export experiments and native QA await their respective gates.
