#!/usr/bin/env python3
"""Validate that UF2 data blocks stay within an application flash range."""

from pathlib import Path
import struct
import sys

MAGIC_START0 = 0x0A324655
MAGIC_START1 = 0x9E5D5157
MAGIC_END = 0x0AB16F30
BLOCK_SIZE = 512
HEADER_SIZE = 32
TRAILER_SIZE = 4
MAX_PAYLOAD_SIZE = BLOCK_SIZE - HEADER_SIZE - TRAILER_SIZE


def usage() -> None:
    print(f"usage: {sys.argv[0]} UF2_FILE ORIGIN END", file=sys.stderr)


def main() -> int:
    if len(sys.argv) != 4:
        usage()
        return 2

    path = Path(sys.argv[1])
    origin = int(sys.argv[2], 0)
    end = int(sys.argv[3], 0)
    contents = path.read_bytes()
    if len(contents) == 0 or len(contents) % BLOCK_SIZE:
        raise ValueError("UF2 file size is not a positive multiple of 512")

    lowest = None
    highest = None
    expected_blocks = None
    for block_index in range(len(contents) // BLOCK_SIZE):
        block = contents[block_index * BLOCK_SIZE : (block_index + 1) * BLOCK_SIZE]
        magic0, magic1, flags, address, payload_size, number, block_count, family = struct.unpack_from(
            "<8I", block
        )
        end_magic = struct.unpack_from("<I", block, BLOCK_SIZE - TRAILER_SIZE)[0]
        if (magic0, magic1, end_magic) != (MAGIC_START0, MAGIC_START1, MAGIC_END):
            raise ValueError(f"block {block_index}: invalid UF2 magic")
        if flags & 0x00002000 == 0:
            raise ValueError(f"block {block_index}: missing family ID flag")
        if payload_size == 0 or payload_size > MAX_PAYLOAD_SIZE:
            raise ValueError(f"block {block_index}: invalid payload size {payload_size}")
        if number != block_index:
            raise ValueError(f"block {block_index}: unexpected block number {number}")
        if expected_blocks is None:
            expected_blocks = block_count
        elif block_count != expected_blocks:
            raise ValueError(f"block {block_index}: inconsistent block count")

        block_end = address + payload_size
        if address < origin or block_end > end:
            raise ValueError(
                f"block {block_index}: data range 0x{address:X}..0x{block_end:X} "
                f"is outside 0x{origin:X}..0x{end:X}"
            )
        lowest = address if lowest is None else min(lowest, address)
        highest = block_end if highest is None else max(highest, block_end)

    if expected_blocks != len(contents) // BLOCK_SIZE:
        raise ValueError("UF2 block count does not match file size")

    print(f"validated 0x{lowest:X}..0x{highest:X} within 0x{origin:X}..0x{end:X}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1)
