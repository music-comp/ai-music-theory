#!/usr/bin/env bash

SOURCE_FILE=$1
OUTPUT_DIR=$2
echo "Reading PDF file '$SOURCE_FILE' ..."
echo "Preparing output directory '$OUTPUT_DIR' ..."
mkdir -p "$OUTPUT_DIR"

# Claude Code does not support this usage; you need to get an
# API key and set up authentication via that mechanism, which
# uses a different billing model :-(
#
# --llm_service marker.services.claude.ClaudeService
source .venv/bin/activate
TORCH_DEVICE=cpu marker_single \
  --output_format markdown \
  --output_dir "$OUTPUT_DIR" \
  "$SOURCE_FILE"

# Example Usage
: <<'EXAMPLES'

scripts/process-pdf.sh \
  ~/Dropbox/Apps/Oxford\ University\ Press/\[2012]\ Gollin\ -\ The\ Oxford\ Handbook\ of\ Neo-Riemannian\ Music\ Theories.pdf \
  sources-md/neo-riemannian-handbook/

scripts/process-pdf.sh \
  ~/Dropbox/Apps/Oxford\ University\ Press/\[2013]\ Caplin\ -\ Analyzing\ Classical\ Form\ -\ An\ Approach\ for\ the\ Classroom.pdf  \
  sources-md/analyzing-classical-form/

EXAMPLES
