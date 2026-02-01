# Documentation Structure Overview

This document shows the complete documentation structure for your Advent of Code project.

## File Organization

```sh
Advent-of-Code/
├── README.md                           # Main project overview
├── documentation/                      # All guides and documentation
│   ├── getting-started.md             # Setup and first steps
│   ├── fetching-assets.md             # Detailed guide for fetch scripts
│   ├── rust-guide.md                  # Rust-specific implementation guide
│   ├── python-guide.md                # Python guide (placeholder)
│   ├── zig-guide.md                   # Zig guide (placeholder)
│   └── contributing.md                # Contribution guidelines
└── scripts/                           # Utility scripts
    ├── README.md                      # Scripts documentation
    ├── fetch-aoc.sh                  # Single day fetcher
    └── fetch-aoc-batch.sh            # Batch fetcher
```

## Documentation Index

### Main Documentation

| File | Purpose | Target Audience |
| --- | --- | --- |
| `README.md` | Project overview, quick start | Everyone |
| `documentation/getting-started.md` | Complete setup guide | New users |
| `documentation/contributing.md` | How to contribute | Contributors |

### Asset Management

| File | Purpose |
| --- | --- |
| `documentation/fetching-assets.md` | Comprehensive guide to fetching puzzles |
| `scripts/README.md` | Script-specific documentation |

### Language Guides

| File | Status | Purpose |
|------|--------|---------|
| `documentation/rust-guide.md` | ✅ Complete | Rust implementation guide |
| `documentation/python-guide.md` | 🚧 Placeholder | Python guide (coming soon) |
| `documentation/zig-guide.md` | 🚧 Placeholder | Zig guide (coming soon) |

## Quick Navigation

### For New Users

1. Start with `README.md`
2. Follow `documentation/getting-started.md`
3. Read `documentation/fetching-assets.md`
4. Choose your language guide

### For Contributors

1. Read `documentation/contributing.md`
2. Review relevant language guide
3. Check `documentation/fetching-assets.md` for asset management

### For Specific Languages

- **Rust Users:** → `documentation/rust-guide.md`
- **Python Users:** → `documentation/python-guide.md` (coming soon)
- **Zig Users:** → `documentation/zig-guide.md` (coming soon)

## Content Summary

### README.md (Main)

- Quick start guide
- Project structure overview
- Links to all documentation
- Quick examples
- Features list

### getting-started.md

- Prerequisites (tools needed)
- Session cookie setup
- Script permissions
- First puzzle fetch
- Workflow overview
- Troubleshooting

### fetching-assets.md

- Detailed script documentation
- Usage examples
- Directory structure
- Advanced usage patterns
- Security best practices
- Troubleshooting guide

### rust-guide.md

- Rust project structure
- Workspace configuration
- Running solutions
- Creating new solutions
- Common patterns
- Performance tips
- Testing strategies

### contributing.md

- Code of conduct
- How to contribute
- Solution guidelines
- Code style (per language)
- Adding new languages
- PR process

### scripts/README.md

- Individual script documentation
- Setup instructions
- Common issues
- Technical details
- POSIX compliance notes

## Where to Put New Documentation

| Topic | Location |
|-------|----------|
| General project info | Update `README.md` |
| Setup/installation | `documentation/getting-started.md` |
| New language support | Create `documentation/<language>-guide.md` |
| Script usage | `documentation/fetching-assets.md` or `scripts/README.md` |
| Contributing process | `documentation/contributing.md` |
| Algorithm explanations | Inline comments or language guide |

## Maintenance Checklist

When updating the project:

- [ ] Update version/status in relevant guides
- [ ] Keep Quick Start in README.md current
- [ ] Update language status (planned → in progress → complete)
- [ ] Add new scripts to scripts/README.md
- [ ] Update contributing.md with new processes
- [ ] Keep troubleshooting sections current

## Documentation Standards

All documentation follows these standards:

- ✅ Markdown format (.md)
- ✅ Clear heading hierarchy
- ✅ Code examples in appropriate language
- ✅ Consistent formatting
- ✅ Cross-references between docs
- ✅ Emoji for status indicators (✅ 🚧 ❌)
- ✅ Tables for structured data
- ✅ Checklists for step-by-step guides

---

**All documentation is in the `documentation/` folder for easy navigation!**
