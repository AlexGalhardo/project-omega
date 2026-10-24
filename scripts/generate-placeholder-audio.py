#!/usr/bin/env python3
"""Synthesizes the original placeholder sound cues and encodes them to Ogg Vorbis.

Every sample is computed here from sine, square and seeded-noise generators: no recording,
sample pack or third-party material is involved. Run from the repository root:

    python scripts/generate-placeholder-audio.py

Needs ffmpeg on PATH for the Vorbis encode. The committed outputs and their hashes are
recorded in assets/audio/provenance.json (ART-012).
"""

import hashlib
import json
import math
import random
import struct
import subprocess
import sys
import tempfile
import wave
from pathlib import Path

Samples = list[float]

RATE = 22050
OUT = Path("assets/audio/placeholder")
PEAK = 0.5


def tone(freq_start: float, freq_end: float, seconds: float, shape: str = "sine", decay: float = 3.0) -> Samples:
	count = int(RATE * seconds)
	phase = 0.0
	out = []
	for i in range(count):
		t = i / count
		freq = freq_start + (freq_end - freq_start) * t
		phase += 2 * math.pi * freq / RATE
		if shape == "square":
			value = 1.0 if math.sin(phase) >= 0 else -1.0
		else:
			value = math.sin(phase)
		out.append(value * math.exp(-decay * t))
	return out


def noise(seconds: float, seed: int, decay: float = 6.0) -> Samples:
	rng = random.Random(seed)
	count = int(RATE * seconds)
	return [rng.uniform(-1, 1) * math.exp(-decay * i / count) for i in range(count)]


def mix(*parts: Samples) -> Samples:
	length = max(len(p) for p in parts)
	return [sum(p[i] for p in parts if i < len(p)) for i in range(length)]


def seq(*parts: Samples) -> Samples:
	out = []
	for p in parts:
		out.extend(p)
	return out


def scale(samples: Samples, gain: float) -> Samples:
	return [s * gain for s in samples]


CUES = {
	"shot": mix(scale(noise(0.06, 1), 0.5), scale(tone(900, 500, 0.06), 0.5)),
	"hit_enemy": mix(scale(noise(0.09, 2), 0.6), scale(tone(300, 150, 0.09), 0.6)),
	"hit_player": mix(scale(noise(0.2, 3), 0.5), scale(tone(130, 80, 0.2, "square"), 0.5)),
	"enemy_death": mix(scale(noise(0.22, 4, 4.0), 0.5), scale(tone(420, 90, 0.22), 0.7)),
	"pickup": tone(1200, 1800, 0.08, decay=2.0),
	"purchase": seq(tone(600, 600, 0.08, decay=1.5), tone(900, 900, 0.1, decay=2.0)),
	"denied": tone(160, 150, 0.16, "square", 2.0),
	"refresh": seq(*[tone(700 + 150 * i, 700 + 150 * i, 0.04, decay=4.0) for i in range(3)]),
	"energy_tick": tone(700, 700, 0.04, decay=4.0),
	"wave_start": tone(300, 620, 0.25, decay=1.5),
	"wave_end": seq(tone(620, 620, 0.12, decay=2.0), tone(420, 420, 0.2, decay=2.5)),
	"victory": seq(*[tone(f, f, 0.14, decay=2.0) for f in (523, 659, 784, 1046)]),
	"defeat": seq(*[tone(f, f, 0.22, "square", 2.0) for f in (330, 262, 196)]),
}


def write_wav(path: Path, samples: Samples) -> None:
	peak = max(1e-9, max(abs(s) for s in samples))
	data = b"".join(struct.pack("<h", int(32767 * PEAK * s / peak)) for s in samples)
	with wave.open(str(path), "wb") as w:
		w.setnchannels(1)
		w.setsampwidth(2)
		w.setframerate(RATE)
		w.writeframes(data)


def main() -> int:
	OUT.mkdir(parents=True, exist_ok=True)
	records = []
	with tempfile.TemporaryDirectory() as tmp:
		for name, samples in CUES.items():
			wav_path = Path(tmp) / f"{name}.wav"
			ogg_path = OUT / f"{name}.ogg"
			write_wav(wav_path, samples)
			subprocess.run(
				["ffmpeg", "-y", "-loglevel", "error", "-i", str(wav_path),
				 "-map_metadata", "-1", "-c:a", "libvorbis", "-q:a", "3", str(ogg_path)],
				check=True,
			)
			data = ogg_path.read_bytes()
			records.append({
				"asset_id": f"audio_placeholder_{name}",
				"file": f"placeholder/{name}.ogg",
				"creator": "Project Omega synthesis script (scripts/generate-placeholder-audio.py)",
				"acquired": "2026-10-24",
				"license": "Original work of the project; no third-party material",
				"allowed_uses": "Any use within the project and its releases",
				"attribution": "None required",
				"modifications": "None after encode",
				"status": "placeholder",
				"bytes": len(data),
				"sha256": hashlib.sha256(data).hexdigest(),
			})
	manifest = {"schema": 1, "assets": records}
	Path("assets/audio/provenance.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
	print(f"wrote {len(records)} cues")
	return 0


if __name__ == "__main__":
	sys.exit(main())
