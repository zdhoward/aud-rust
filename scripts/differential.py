#!/usr/bin/env python3
"""Differential harness: run Python `aud` and the Rust `aud` on identical trees.

Usage: python scripts/differential.py [--rust PATH] [--reference DIR] [--only ID]

Both sides get a fresh copy of each scenario tree. Stdout is decoded as JSON
when possible and compared after replacing the tree root with $ROOT; exit codes
are compared too. Any difference fails the run (allowed-diffs arrive in M8).
Standard library only. Run it with the reference checkout's Python 3.10+.
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

BS = chr(92)
REPO = Path(__file__).resolve().parent.parent


def find_reference(explicit):
    candidates = [explicit, os.environ.get("AUD_REFERENCE"), REPO.parent / "aud", REPO / ".reference" / "aud"]
    for candidate in candidates:
        if candidate and (Path(candidate) / "aud" / "cli.py").is_file():
            return Path(candidate).resolve()
    sys.exit("Python reference not found: pass --reference, set AUD_REFERENCE, or run scripts/fetch-reference.ps1")


def find_rust(explicit):
    if explicit:
        return Path(explicit).resolve()
    exe = "aud.exe" if os.name == "nt" else "aud"
    for profile in ("debug", "release"):
        path = REPO / "target" / profile / exe
        if path.is_file():
            return path
    sys.exit("Rust binary not found: run `cargo build` or pass --rust")


def touch(root, *names):
    for name in names:
        (root / name).write_bytes(b"x")


def mkdir(root, name):
    (root / name).mkdir()


def case_collision(root):
    touch(root, "a.wav", "A.wav", "b.wav")
    return len([p for p in root.iterdir() if p.name.lower() == "a.wav"]) == 2


def symlink(root):
    touch(root, "real.wav")
    try:
        os.symlink(root / "real.wav", root / "link.wav")
    except (OSError, NotImplementedError):
        return False
    return True


def scenarios():
    u = chr
    ext = ["--extensions", "wav", "flac"]
    v1 = ["--json", "--json-version", "1"]
    mixed = ["a.wav", "b.flac", "c.mp3", "d.txt", "e.WAV"]
    return [
        dict(id="extensions", files=mixed, args=lambda r: ["list", str(r), *ext, *v1]),
        dict(id="no-extensions", files=mixed, args=lambda r: ["list", str(r), *v1]),
        dict(id="pattern", files=["01 a.wav", "02 b.wav", "03 c.wav", "01 d.txt"],
             args=lambda r: ["list", str(r), "--extensions", "wav", "--pattern", "0[12]*", *v1]),
        dict(id="pattern-case-sensitive", files=["A.wav"],
             args=lambda r: ["list", str(r), "--extensions", "wav", "--pattern", "a*", *v1]),
        dict(id="extension-case", files=["A.WAV", "b.Wav", "c.wav", "d.flac"],
             args=lambda r: ["list", str(r), "--extensions", ".WAV", *v1]),
        dict(id="odd-names", files=[".wav", "a.", "a.b.wav", "..wav", "noext", " spaced name.wav", "x.wav "],
             args=lambda r: ["list", str(r), "--extensions", "wav", *v1]),
        dict(id="unicode", files=[u(0xE9) + ".wav", "e" + u(0x301) + ".wav", u(0x1F3B5) + ".wav",
                                  u(0xFFEE) + ".wav", u(0xDF) + ".wav", "SS.wav", "Z.wav"],
             args=lambda r: ["list", str(r), "--extensions", "wav", *v1]),
        dict(id="sort-order", files=["B.wav", "a.wav", "C.wav", "b2.wav"],
             args=lambda r: ["list", str(r), "--extensions", "wav", *v1]),
        dict(id="empty-dir", files=[], args=lambda r: ["list", str(r), "--extensions", "wav", *v1]),
        dict(id="subdirectory-ignored", files=["y.wav"], setup=lambda r: mkdir(r, "x.wav"),
             args=lambda r: ["list", str(r), "--extensions", "wav", *v1]),
        dict(id="case-collision", files=[], setup=case_collision,
             args=lambda r: ["list", str(r), "--extensions", "wav", *v1]),
        dict(id="symlink-skipped", files=[], setup=symlink,
             args=lambda r: ["list", str(r), "--extensions", "wav", *v1]),
        dict(id="plain-json", files=mixed, args=lambda r: ["list", str(r), *ext, "--json"]),
        dict(id="human-output", files=mixed, text=True, args=lambda r: ["list", str(r), *ext]),
        dict(id="missing-directory", files=[],
             args=lambda r: ["list", str(r / "missing"), *v1], mask_error=False),
        dict(id="file-as-directory", files=["a.wav"],
             args=lambda r: ["list", str(r / "a.wav"), *v1]),
        dict(id="missing-directory-human", files=[], text=True,
             args=lambda r: ["list", str(r / "missing")]),
        dict(id="version-without-json", files=[], text=True,
             args=lambda r: ["list", str(r), "--json-version", "1"], mask_error=True),
        dict(id="usage-error-json", files=[], args=lambda r: ["list", str(r), "--json", "--bogus"], mask_error=True),
        dict(id="legacy-count", files=["a.wav"], text=True, args=lambda r: [str(r)]),
        dict(id="legacy-missing", files=[], text=True, args=lambda r: [str(r / "missing")]),
    ]


def normalize_text(text, root):
    root_s = str(root)
    out = []
    for line in text.splitlines():
        for variant in (root_s, root_s.replace(BS, BS + BS)):
            while variant in line:
                head, _, tail = line.partition(variant)
                rest = ""
                for ch in tail:
                    if ch in ('"', " "):
                        break
                    rest += ch
                line = head + "$ROOT" + rest.replace(BS + BS, "/").replace(BS, "/") + tail[len(rest):]
        out.append(line)
    return chr(10).join(out)


def normalize_json(value, root):
    if isinstance(value, str):
        return normalize_text(value, root)
    if isinstance(value, list):
        return [normalize_json(v, root) for v in value]
    if isinstance(value, dict):
        return {k: normalize_json(v, root) for k, v in value.items()}
    return value


def observe(argv, cwd, env, root, scenario):
    done = subprocess.run(argv, cwd=cwd, env=env, capture_output=True, timeout=120)
    stdout = done.stdout.decode("utf-8")
    stderr = done.stderr.decode("utf-8")
    result = {"code": done.returncode}
    if scenario.get("text"):
        result["stdout"] = normalize_text(stdout, root)
        result["stderr"] = normalize_text(stderr, root)
        return result
    text = stdout if stdout.strip() else stderr
    try:
        doc = normalize_json(json.loads(text), root)
    except ValueError:
        doc = {"unparsed": normalize_text(text, root)}
    result["stream"] = "stdout" if stdout.strip() else "stderr"
    result["doc"] = doc
    return result


def mask_error_text(result):
    doc = result.get("doc")
    if isinstance(doc, dict):
        if "error" in doc:
            doc["error"] = "<masked>"
        payload = doc.get("payload")
        if isinstance(payload, dict) and "error" in payload:
            payload["error"] = "<masked>"
    for key in ("stdout", "stderr"):
        if key in result and result[key].startswith("aud: "):
            result[key] = "<masked usage message>"


def run_scenario(scenario, reference, rust):
    results = {}
    for side in ("python", "rust"):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve() / "tree"
            root.mkdir()
            touch(root, *scenario["files"])
            if "setup" in scenario and scenario["setup"](root) is False:
                return "SKIP", None
            argv_tail = scenario["args"](root)
            env = dict(os.environ, PYTHONIOENCODING="utf-8", PYTHONUTF8="1")
            if side == "python":
                env["PYTHONPATH"] = str(reference)
                argv = [sys.executable, "-X", "utf8", "-m", "aud.cli", *argv_tail]
                cwd = reference
            else:
                argv = [str(rust), *argv_tail]
                cwd = root.parent
            results[side] = observe(argv, cwd, env, root, scenario)
    if scenario.get("mask_error", False):
        for side in results.values():
            mask_error_text(side)
    return ("PASS" if results["python"] == results["rust"] else "FAIL"), results


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--rust")
    parser.add_argument("--reference")
    parser.add_argument("--only")
    args = parser.parse_args()
    reference, rust = find_reference(args.reference), find_rust(args.rust)
    print("python reference:", reference)
    print("rust binary:     ", rust)
    failed = 0
    ran = 0
    for scenario in scenarios():
        if args.only and scenario["id"] != args.only:
            continue
        status, results = run_scenario(scenario, reference, rust)
        print(f"{status:5} {scenario['id']}")
        if status == "FAIL":
            failed += 1
            print("  python:", json.dumps(results["python"], ensure_ascii=True))
            print("  rust:  ", json.dumps(results["rust"], ensure_ascii=True))
        if status != "SKIP":
            ran += 1
    print(f"{ran} scenarios run, {failed} differ")
    if ran < 10 and not args.only:
        print("fewer than 10 scenarios ran", file=sys.stderr)
        return 1
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
