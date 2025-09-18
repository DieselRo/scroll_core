# Scroll Archive Slot

This directory remains as the mount point for alternate archive bundles. By default the Scroll Core runtime now loads
its canonical scrollbooks from `scroll_core/docs/scrolls`. To swap in a different archive for experiments or downstream
projects, place the desired scroll set here and set `SCROLL_CORE_ARCHIVE_DIR` to `scrolls` before running the CLI.

```
# Use the built-in archive
set SCROLL_CORE_ARCHIVE_DIR=scroll_core/docs/scrolls

# Swap in a custom archive (relative to repo root)
set SCROLL_CORE_ARCHIVE_DIR=scrolls/my-custom-archive
```

Keep this directory empty (or gitignored) when you are not using an alternate archive so the docs copy remains the source
of truth.
