#!/usr/bin/env python3
"""
Query JSONL change logs with various filters.

Usage:
    python query_changes.py [options]

Options:
    --log PATH          Log file path (default: .claude/tracking/changes.jsonl)
    --today             Filter to today's events only
    --date DATE         Filter to specific date (YYYY-MM-DD)
    --type TYPE         Filter by event type (e.g., file.modified, prd.created)
    --actor ACTOR       Filter by actor (user, agent, skill, hook)
    --entity ENTITY     Filter by entity name/id
    --last N            Show only last N events
    --count             Show count by event type
    --summary           Show activity summary
    --json              Output as JSON (default: table)
"""

import json
import sys
from datetime import datetime, date
from pathlib import Path
from collections import defaultdict


def parse_jsonl(path: Path):
    """Stream parse JSONL file, yielding valid entries."""
    if not path.exists():
        return
    with open(path, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    yield json.loads(line)
                except json.JSONDecodeError:
                    continue


def filter_events(events, filters: dict):
    """Apply filters to event stream."""
    for event in events:
        if filters.get('date'):
            ts = event.get('timestamp', '')[:10]
            if ts != filters['date']:
                continue

        if filters.get('type'):
            if not event.get('eventType', '').startswith(filters['type']):
                continue

        if filters.get('actor'):
            if event.get('actor') != filters['actor']:
                continue

        if filters.get('entity'):
            entity = filters['entity'].lower()
            data = event.get('data', {})
            match = False
            for key in ['name', 'entityId', 'path', 'taskId', 'epicName']:
                if entity in str(data.get(key, '')).lower():
                    match = True
                    break
            if not match:
                continue

        yield event


def format_table(events: list):
    """Format events as a simple table."""
    if not events:
        print("No events found.")
        return

    print(f"{'Timestamp':<20} {'Event Type':<25} {'Actor':<8} {'Details'}")
    print("-" * 80)

    for e in events:
        ts = e.get('timestamp', '')[:19].replace('T', ' ')
        et = e.get('eventType', 'unknown')[:25]
        actor = e.get('actor', '-')[:8]

        # Extract key detail from data
        data = e.get('data', {})
        detail = data.get('name') or data.get('path') or data.get('taskId') or ''
        if len(detail) > 30:
            detail = detail[:27] + '...'

        print(f"{ts:<20} {et:<25} {actor:<8} {detail}")


def count_by_type(events: list):
    """Count events by type."""
    counts = defaultdict(int)
    for e in events:
        counts[e.get('eventType', 'unknown')] += 1

    print(f"{'Count':<8} Event Type")
    print("-" * 40)
    for et, count in sorted(counts.items(), key=lambda x: -x[1]):
        print(f"{count:<8} {et}")
    print("-" * 40)
    print(f"{sum(counts.values()):<8} TOTAL")


def activity_summary(events: list):
    """Generate activity summary."""
    categories = defaultdict(lambda: defaultdict(int))

    for e in events:
        et = e.get('eventType', '')
        category = et.split('.')[0] if '.' in et else 'other'
        action = et.split('.')[1] if '.' in et else et
        categories[category][action] += 1

    print("## Activity Summary\n")
    print(f"**Total Events**: {len(events)}\n")

    if events:
        first = events[0].get('timestamp', '')[:10]
        last = events[-1].get('timestamp', '')[:10]
        print(f"**Period**: {first} to {last}\n")

    print("### By Category\n")
    print("| Category | Count | Breakdown |")
    print("|----------|-------|-----------|")

    for cat in sorted(categories.keys()):
        actions = categories[cat]
        total = sum(actions.values())
        breakdown = ', '.join(f"{a}:{c}" for a, c in sorted(actions.items()))
        print(f"| {cat} | {total} | {breakdown} |")


def main():
    import argparse

    parser = argparse.ArgumentParser(description='Query JSONL change logs')
    parser.add_argument('--log', default='.claude/tracking/changes.jsonl', help='Log file path')
    parser.add_argument('--today', action='store_true', help='Filter to today')
    parser.add_argument('--date', help='Filter to date (YYYY-MM-DD)')
    parser.add_argument('--type', help='Filter by event type')
    parser.add_argument('--actor', help='Filter by actor')
    parser.add_argument('--entity', help='Filter by entity name/id')
    parser.add_argument('--last', type=int, help='Show last N events')
    parser.add_argument('--count', action='store_true', help='Count by type')
    parser.add_argument('--summary', action='store_true', help='Activity summary')
    parser.add_argument('--json', action='store_true', help='Output as JSON')

    args = parser.parse_args()

    log_path = Path(args.log)

    filters = {}
    if args.today:
        filters['date'] = date.today().isoformat()
    elif args.date:
        filters['date'] = args.date
    if args.type:
        filters['type'] = args.type
    if args.actor:
        filters['actor'] = args.actor
    if args.entity:
        filters['entity'] = args.entity

    events = list(filter_events(parse_jsonl(log_path), filters))

    if args.last:
        events = events[-args.last:]

    if args.json:
        print(json.dumps(events, indent=2))
    elif args.count:
        count_by_type(events)
    elif args.summary:
        activity_summary(events)
    else:
        format_table(events)


if __name__ == '__main__':
    main()
