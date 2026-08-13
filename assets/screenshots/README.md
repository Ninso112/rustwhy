# Screenshots

This directory is reserved for visual assets used in the top-level
`README.md` and in `docs/`.

## Generating a demo cast

The README expects two visual assets:

| File              | Used in                  | Source                              |
| ----------------- | ------------------------ | ----------------------------------- |
| `demo.gif`        | Hero block in `README.md`| `casts/demo.cast` (asciinema)       |
| `demo.cast`       | Optional asciinema embed | Recorded with `scripts/record.sh`   |

The pipeline below uses [asciinema] + [agg] so the source recording stays
diff-friendly (plain text) while the rendered asset is a small animated
GIF that GitHub will autoplay inline.

### One-time setup

```bash
# Install asciinema (records terminal sessions to .cast JSON)
sudo apt install asciinema         # Debian / Ubuntu
sudo pacman -S asciinema           # Arch

# Install agg (turns .cast into a .gif)
cargo install --locked agg
```

### Recording a demo

A reproducible recording script lives at `scripts/record.sh`. It captures
the most representative commands and exits cleanly:

```bash
./scripts/record.sh        # writes assets/screenshots/demo.cast
agg assets/screenshots/demo.cast assets/screenshots/demo.gif
```

The script intentionally uses fixed-width fonts, a deterministic color
palette, and short pauses so the resulting GIF stays under ~1 MB.

### Replacing the demo

When the output of `rustwhy` changes (new defaults, new flags, etc.):

1. Edit `scripts/record.sh` to mirror the new behavior.
2. Re-run the two commands above.
3. Open a PR — keep both `demo.cast` (source) and `demo.gif` (rendered)
   in this directory so the README stays self-contained.

[asciinema]: https://asciinema.org/
[agg]: https://github.com/asciinema/agg
