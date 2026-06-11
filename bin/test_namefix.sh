#!/bin/bash

# Test script for namefix - creates a temporary directory with problematic filenames

set -e

# Get the project root (parent of bin directory)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "Building namefix..."
cd "$PROJECT_ROOT"
cargo build --bin namefix --quiet

NAMEFIX="$PROJECT_ROOT/target/debug/namefix"
if [ ! -x "$NAMEFIX" ]; then
    echo "ERROR: Failed to build namefix"
    exit 1
fi
echo "INFO: namefix built successfully"
echo ""

# Create temporary test directory
TEST_DIR=$(mktemp -d)
echo "INFO: Created test directory: $TEST_DIR"

# Create files with various problematic names

# File 1: Valid UTF-8 name (baseline)
touch "$TEST_DIR/normal_file.txt"
echo "INFO: Created: normal_file.txt"

# File 2: File with invalid UTF-8 sequence (0xFF)
touch "$TEST_DIR/$(printf 'invalid\xff_char.txt')" 2>/dev/null || true
echo "INFO: Created: invalid_char.txt (with 0xFF byte)"

# File 3: File with another invalid UTF-8 sequence (0x80)
touch "$TEST_DIR/$(printf 'bad\x80encoding.log')" 2>/dev/null || true
echo "INFO: Created: bad_encoding.log (with 0x80 byte)"

# File 4: File with invalid UTF-8 continuation byte (0xC0 without proper continuation)
touch "$TEST_DIR/$(printf 'broken\xc0name.md')" 2>/dev/null || true
echo "INFO: Created: broken_name.md (with 0xC0 byte)"

# File 5: File with multiple invalid bytes
touch "$TEST_DIR/$(printf 'multi\xff\x80\xc1bad.txt')" 2>/dev/null || true
echo "INFO: Created: multi_bad.txt (with 0xFF, 0x80, 0xC1 bytes)"

# File 6: Subdirectory with invalid name
mkdir -p "$TEST_DIR/$(printf 'bad\xffdir')" 2>/dev/null || true
echo "INFO: Created: bad_dir (with 0xFF byte)"

# File 7: File inside the problematic subdirectory
touch "$TEST_DIR/$(printf 'bad\xffdir')/$(printf 'nested\x80file.txt')" 2>/dev/null || true
echo "INFO: Created: nested_file.txt in bad_dir"

echo ""
echo "INFO: Test directory ready at: $TEST_DIR"
echo ""
echo "INFO: Listing contents before fix (names may appear garbled):"
ls -la "$TEST_DIR"
echo ""
echo "INFO: Running namefix with --verbose and --dry-run:"
echo "========================================"
"$NAMEFIX" --verbose --dry-run "$TEST_DIR"
echo ""
echo "INFO: Running namefix to actually fix files:"
echo "========================================"
"$NAMEFIX" --verbose "$TEST_DIR"
echo ""
echo "INFO: Listing contents after fix:"
ls -la "$TEST_DIR"
echo ""
echo "INFO: Test directory: $TEST_DIR"
echo "INFO: To clean up:"
echo "  rm -rf '$TEST_DIR'"
