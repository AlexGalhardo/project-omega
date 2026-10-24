# Release process

Releases follow the owner's pattern from [vukem](https://github.com/AlexGalhardo/vukem/releases) (DEC-016): tag `vX.Y.Z`, release title equal to the tag, notes copied from the matching [CHANGELOG.md](../CHANGELOG.md) section, and signed artifacts produced by [`.github/workflows/release.yml`](../.github/workflows/release.yml).

## Versioning

SemVer. The version per phase is fixed in [ROADMAP.md section 1.4](../ROADMAP.md#14-versioning-by-phase); the commit type does not decide it. The workspace version in the root `Cargo.toml` must equal the tag; the workflow fails otherwise.

## CHANGELOG format

[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/). Each version section groups entries under `### Added`, `### Changed`, `### Fixed`, `### Removed`, `### Security` (only groups that have entries). Each entry is one commit:

```markdown
- **scope:** Sentence-case summary of the change ([abc1234](https://github.com/AlexGalhardo/project-omega/commit/<full sha>))
```

Map Conventional Commit types to groups: `feat` → Added; `fix` → Fixed; `security` → Security; removals → Removed; `docs`, `refactor`, `perf`, `build`, `ci`, `chore`, `test` → Changed when user-visible or useful to a reader, otherwise omitted. `chore(release)` commits are never listed. Keep `## [Unreleased]` at the top and compare links at the bottom.

## Cutting a release

1. `powershell -NoProfile -File scripts/check.ps1` exits 0 and CI on `main` is green.
2. Bump `version` in the root `Cargo.toml`; run `cargo metadata --format-version 1 > $null` to refresh `Cargo.lock`.
3. Move `[Unreleased]` entries into `## [X.Y.Z] - YYYY-MM-DD`; update the compare links.
4. Commit `chore(release): X.Y.Z` with exactly `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`.
5. `git tag -a vX.Y.Z -m "vX.Y.Z"`, then `git push origin main` and `git push origin vX.Y.Z` (one tag per push: GitHub sends no push event when more than three tags go up at once).
6. The release workflow builds `project-omega_X.Y.Z_windows_amd64.zip` (binaries, `assets/presentation`, `assets/generated`, README, CHANGELOG, Bevy license files), its SPDX SBOM (syft, from the `Cargo.lock` graph), `checksums.txt`, the cosign keyless bundle `checksums.txt.sigstore.json` and the provenance bundle `project-omega.intoto.jsonl`, then publishes the release with the notes from `scripts/release-notes.sh X.Y.Z`.
7. If the tag went up without a run, dispatch the workflow with the version instead of moving the tag.

## Verifying a release

```bash
sha256sum -c checksums.txt --ignore-missing
cosign verify-blob checksums.txt --bundle checksums.txt.sigstore.json \
  --certificate-identity "https://github.com/AlexGalhardo/project-omega/.github/workflows/release.yml@refs/tags/vX.Y.Z" \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
gh attestation verify project-omega_X.Y.Z_windows_amd64.zip --repo AlexGalhardo/project-omega
```

## Retroactive releases

`v0.1.0`, `v0.1.1` and `v0.2.0` cover the planning history before any code existed. They were published with `gh release create vX.Y.Z --target <sha> --title vX.Y.Z --notes-file <section>` and carry notes only: there was nothing to build. Unattended runs never publish releases.
