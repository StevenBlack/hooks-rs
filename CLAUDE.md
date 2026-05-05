# CLAUDE.md

This file is loaded automatically by Claude Code on any machine. It provides context for AI-assisted development on this project.

## Project

`hooks-rs` is a Rust library implementing a composable hook chain pattern. Hooks are linked nodes that each process a value and pass it to the next hook in the chain.

## Core abstractions

- `HookStorage<T>` — trait providing `hook()` / `hook_mut()` accessors to the next hook in chain
- `Hookable<T>` — main trait; implementors define `execute()` and optionally override `pre_process` / `post_process`
- `Hook<T>` = `Option<Box<dyn Hookable<T>>>` — type alias for a chain link
- `StringHook` = `Option<Box<dyn Hookable<String>>>` — convenience alias for string chains
- `impl_string_hook_storage!(Type)` — macro to reduce HookStorage boilerplate

Concrete implementations live in `src/hooks/utilities.rs`: `TrimHook`, `AppendHook`, `UppercaseHook`.

## Design direction

See [`DESIGN.md`](DESIGN.md) for the ongoing design discussion around extending the hook chain to support heterogeneous content (text, images, PDFs, document sequences).
