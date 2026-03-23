#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/bench-history.sh [history.csv] [results.md]
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HISTORY="${1:-$REPO_ROOT/bench-history.csv}"
RESULTS="${2:-$REPO_ROOT/benchmarks.md}"
BENCH_NAME="benchmarks"

TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
COMMIT=$(git -C "$REPO_ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown")
BRANCH=$(git -C "$REPO_ROOT" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

# Ensure CSV header
if [ ! -f "$HISTORY" ]; then
    echo "timestamp,commit,branch,benchmark,estimate_ns" > "$HISTORY"
fi

echo "Running benchmarks..."
RAW=$(cargo bench --bench "$BENCH_NAME" 2>&1 | sed 's/\x1b\[[0-9;]*m//g')

# Parse criterion output: "benchmark_name  time:   [low mid high]"
while IFS= read -r line; do
    # Match lines like: "trait_behavior_lookup  time:   [1.2345 ns 1.3000 ns 1.3700 ns]"
    if [[ "$line" =~ ^([a-zA-Z0-9_/]+)[[:space:]]+time:[[:space:]]+\[.*[[:space:]]([0-9.]+)[[:space:]]+(ps|ns|µs|us|ms|s)[[:space:]] ]]; then
        NAME="${BASH_REMATCH[1]}"
        VALUE="${BASH_REMATCH[2]}"
        UNIT="${BASH_REMATCH[3]}"

        # Normalize to nanoseconds
        case "$UNIT" in
            ps) NS=$(echo "$VALUE / 1000" | bc -l) ;;
            ns) NS="$VALUE" ;;
            µs|us) NS=$(echo "$VALUE * 1000" | bc -l) ;;
            ms) NS=$(echo "$VALUE * 1000000" | bc -l) ;;
            s)  NS=$(echo "$VALUE * 1000000000" | bc -l) ;;
            *)  NS="$VALUE" ;;
        esac

        echo "$TIMESTAMP,$COMMIT,$BRANCH,$NAME,$NS" >> "$HISTORY"
        printf "  %-40s %s %s\n" "$NAME" "$VALUE" "$UNIT"
    fi
done <<< "$RAW"

echo ""
echo "Results appended to $HISTORY"

# Generate markdown with last 3 runs
python3 -c "
import csv, sys
from collections import defaultdict, OrderedDict

rows = []
with open('$HISTORY') as f:
    reader = csv.DictReader(f)
    for r in reader:
        rows.append(r)

if not rows:
    sys.exit(0)

# Get last 3 unique (timestamp, commit) pairs
seen = OrderedDict()
for r in reversed(rows):
    key = (r['timestamp'], r['commit'])
    if key not in seen:
        seen[key] = True
    if len(seen) >= 3:
        break
runs = list(reversed(seen.keys()))

# Group by benchmark
benchmarks = defaultdict(dict)
for r in rows:
    key = (r['timestamp'], r['commit'])
    if key in seen:
        benchmarks[r['benchmark']][key] = float(r['estimate_ns'])

def fmt_ns(ns):
    if ns < 1:
        return f'{ns*1000:.1f} ps'
    elif ns < 1000:
        return f'{ns:.1f} ns'
    elif ns < 1_000_000:
        return f'{ns/1000:.1f} us'
    elif ns < 1_000_000_000:
        return f'{ns/1_000_000:.1f} ms'
    else:
        return f'{ns/1_000_000_000:.2f} s'

with open('$RESULTS', 'w') as f:
    f.write('# Benchmarks\n\n')
    f.write('Last {} runs.\n\n'.format(len(runs)))

    headers = ['Benchmark'] + [f'{c} ({t[:10]})' for t, c in runs] + ['Delta']
    f.write('| ' + ' | '.join(headers) + ' |\n')
    f.write('| ' + ' | '.join(['---'] * len(headers)) + ' |\n')

    for bench in sorted(benchmarks.keys()):
        vals = benchmarks[bench]
        cols = [bench]
        first_val = None
        last_val = None
        for run in runs:
            if run in vals:
                v = vals[run]
                cols.append(fmt_ns(v))
                if first_val is None:
                    first_val = v
                last_val = v
            else:
                cols.append('-')
        if first_val and last_val and first_val > 0:
            delta = ((last_val - first_val) / first_val) * 100
            cols.append(f'{delta:+.1f}%')
        else:
            cols.append('-')
        f.write('| ' + ' | '.join(cols) + ' |\n')

print(f'Markdown written to $RESULTS')
" 2>/dev/null || echo "(python3 not available -- CSV updated, markdown skipped)"
