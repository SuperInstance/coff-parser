# COFF Parser — Common Object File Format Reader

**COFF (Common Object File Format)** is a binary format for object files, executables, and shared libraries, originally defined by AT&T Unix System V and used today as the basis for PE (Windows executables), XCOFF (AIX), and embedded toolchains (TI DSPs, MIPS). This crate provides a parser for COFF headers, section tables, and symbol tables.

## Why It Matters

Every `.obj` file compiled on Windows, every `.out` built for a TI DSP, and every Mach-O predecessor uses COFF or a close variant. If you're building a linker, a debugger, a binary analysis tool (like a reverse-engineering framework or a malware sandbox), or a cross-compiler toolchain, you need to parse COFF. The format is also the foundation of PE (Portable Executable) — the format of every Windows `.exe` and `.dll` — so COFF parsing is the first step of PE parsing. Understanding binary formats is essential for systems programmers working on build tools, profilers, and security analysis.

## How It Works

A COFF file consists of three regions:

```
┌─────────────────────┐
│   COFF Header       │  20 bytes (fixed)
├─────────────────────┤
│  Optional Header    │  variable (only for executables)
├─────────────────────┤
│  Section Headers    │  40 bytes each × number_of_sections
├─────────────────────┤
│  Section Data       │  raw bytes for each section
├─────────────────────┤
│  Relocation Tables  │  per-section
├─────────────────────┤
│  Symbol Table       │  18 bytes per entry
├─────────────────────┤
│  String Table       │  for symbol names > 8 chars
└─────────────────────┘
```

### COFF Header (20 bytes)

| Field | Size | Description |
|---|---|---|
| `machine` | 2 | Target architecture (e.g., `0x14c` = i386, `0x8664` = x86-64) |
| `number_of_sections` | 2 | Count of section headers following the optional header |
| `time_date_stamp` | 4 | Unix timestamp of file creation |
| `pointer_to_symbol_table` | 4 | File offset to symbol table |
| `number_of_symbols` | 4 | Count of symbol table entries |
| `size_of_optional_header` | 2 | Size of the optional header (0 for object files) |
| `characteristics` | 2 | Flags: executable, DLL, line numbers stripped, etc. |

### Section Header (40 bytes)

Each section (`.text`, `.data`, `.bss`, `.rdata`) has its own header with virtual address, raw data size, and file offset. The format is little-endian on x86 and big-endian on some embedded platforms.

**Complexity**: Header parsing is `O(1)` — fixed-size reads. Full section enumeration is `O(n)` where `n = number_of_sections`.

## Quick Start

```rust
use coff_parser::{parse_coff_header, CoffHeader};

let data = std::fs::read("object.o")?;
let header = parse_coff_header(&data)?;
println!("Machine: 0x{:04x}", header.machine);
println!("Sections: {}", header.number_of_sections);
println!("Symbols:  {}", header.number_of_symbols);
```

## API

| Struct / Function | Description |
|---|---|
| `CoffHeader` | Parsed COFF file header (7 fields). |
| `SectionHeader` | Parsed section header: name, virtual size/address, raw data. |
| `parse_coff_header(data)` | Parse a `&[u8]` into a `CoffHeader`. Returns `Result<CoffHeader, String>`. |

## Architecture Notes

COFF parsing serves the η (evaluation/analysis) side of γ + η = C in SuperInstance — it's a binary analysis primitive for inspecting compiled artifacts. It supports static analysis, linking, and build verification. See [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Microsoft PE and COFF Specification. <https://learn.microsoft.com/en-us/windows/win32/debug/pe-format> — The authoritative reference.
2. System V Application Binary Interface, Intel386 Architecture Processor Supplement. — Unix COFF variant.
3. Levine, J. R. (2000). *Linkers and Loaders*. Morgan Kaufmann. — Covers COFF and related formats in depth.

## License

MIT
