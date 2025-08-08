---
title: CLI Command Reference
status: active
audience: user, dev
---

# CLI Command Reference

## Chat

```
cargo run -- chat <construct> [--stream|--no-stream] [--no-banner]
```

- Talk to Mythscribe: `cargo run -- chat mythscribe --no-stream`

## Rituals

- Validate one: `cargo run -- ritual --action validate --file <relpath>`
- Validate all: `cargo run -- ritual --action validate-all`
- Write (persist) and index: `cargo run -- ritual --action write --file <relpath> --update-index`
- Seal: `cargo run -- ritual --action seal --file <relpath>`

## Index

- List: `cargo run -- index --action list`
- Add: `cargo run -- index --action add --file <relpath>`
- Remove: `cargo run -- index --action remove --file <relpath>`

## Docs maintenance

- Build index: `cargo run -- doc --action index`
- Recent report: `cargo run -- doc --action recent`
- Classify (json/md): `cargo run -- doc --action classify`
- Normalize headers: `cargo run -- doc --action normalize`
- Fix missing headers (adds minimal YAML): `cargo run -- doc --action classify --fix-headers`


