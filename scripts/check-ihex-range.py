#!/usr/bin/env python3
"""Validate that Intel HEX data stays within an application flash range."""

from pathlib import Path
import sys


def usage() -> None:
    print(f"usage: {sys.argv[0]} HEX_FILE ORIGIN END", file=sys.stderr)


def main() -> int:
    if len(sys.argv) != 4:
        usage()
        return 2

    path = Path(sys.argv[1])
    origin = int(sys.argv[2], 0)
    end = int(sys.argv[3], 0)
    base = 0
    lowest = None
    highest = None

    for line_number, raw_line in enumerate(path.read_text().splitlines(), 1):
        line = raw_line.strip()
        if not line:
            continue
        if not line.startswith(":"):
            raise ValueError(f"line {line_number}: missing record marker")

        record = bytes.fromhex(line[1:])
        if len(record) < 5 or sum(record) & 0xFF:
            raise ValueError(f"line {line_number}: invalid record or checksum")

        count = record[0]
        address = int.from_bytes(record[1:3], "big")
        record_type = record[3]
        data = record[4:-1]
        if len(data) != count:
            raise ValueError(f"line {line_number}: invalid data length")

        if record_type == 0x00:
            start = base + address
            record_end = start + count
            if start < origin or record_end > end:
                raise ValueError(
                    f"line {line_number}: data range 0x{start:X}..0x{record_end:X} "
                    f"is outside 0x{origin:X}..0x{end:X}"
                )
            if count:
                lowest = start if lowest is None else min(lowest, start)
                highest = record_end if highest is None else max(highest, record_end)
        elif record_type == 0x01:
            break
        elif record_type == 0x02:
            if count != 2:
                raise ValueError(f"line {line_number}: invalid segment address record")
            base = int.from_bytes(data, "big") << 4
        elif record_type == 0x04:
            if count != 2:
                raise ValueError(f"line {line_number}: invalid extended address record")
            base = int.from_bytes(data, "big") << 16
        elif record_type in (0x03, 0x05):
            continue
        else:
            raise ValueError(f"line {line_number}: unsupported record type 0x{record_type:02X}")

    if lowest is None:
        raise ValueError("HEX file contains no data records")

    print(f"validated 0x{lowest:X}..0x{highest:X} within 0x{origin:X}..0x{end:X}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1)
