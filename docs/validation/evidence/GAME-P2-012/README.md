# Evidence: GAME-P2-012

Status: IN_REVIEW (code complete; native audition is the owner's, see the local questions file). Base commit: `e529149`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `python scripts/generate-placeholder-audio.py` | 0 | 13 Ogg Vorbis cues and `assets/audio/provenance.json` |
| `cargo test -p omega_game --no-default-features --locked --test audio_contract` | 0 | 6 passed |
| `cargo run -p omega_game --features client --locked --bin omega-game -- --play --screenshot target/audio-smoke.png` | 0 | starts with audio plugin, no load error logged |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Every clip has provenance with matching hash (ART-012) | `every_cue_has_a_clip_and_a_matching_provenance_record` |
| Accepted hit/death/pickup/attack cues, grouping, player hit distinct | `accepted_events_map_to_cues_and_bursts_group_into_one_sound` |
| Shop cues | `shop_messages_map_to_purchase_refresh_and_denied` |
| SP-015-AC1 one result cue, wave start/end edges, teardown | `ac1_result_cue_plays_once_and_wave_edges_pair_up` |
| SP-015-AC2 trace unchanged with audio, mute and master zero silent | `ac2_the_simulation_trace_is_identical_with_and_without_audio`, `mix_gain_is_master_times_sfx_and_mute_or_zero_silences` |

## Limits

Placeholders are synthesized from sine, square and seeded noise by `scripts/generate-placeholder-audio.py` (needs ffmpeg for the Vorbis encode); no third-party material. Music, ambience, telegraph cues and looping teardown arrive with GAME-P4-005. Pause does not yet freeze in-flight one-shots (clips are under 0.3 s). SP-015-AC3/AC4 need enemy windups and native audition (Phase 3/4).
