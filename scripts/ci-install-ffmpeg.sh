#!/usr/bin/env bash
# Install FFmpeg (ffmpeg + ffprobe) on a GitHub Actions runner. Usage: scripts/ci-install-ffmpeg.sh
set -euo pipefail

case "${RUNNER_OS:-$(uname -s)}" in
  Linux)
    sudo apt-get update
    sudo apt-get install -y ffmpeg
    ;;
  macOS)
    brew install ffmpeg
    ;;
  Windows)
    choco install ffmpeg --no-progress -y
    ;;
  *)
    echo "unsupported OS: ${RUNNER_OS:-$(uname -s)}" >&2
    exit 1
    ;;
esac

ffmpeg -version | head -n 1
ffprobe -version | head -n 1
