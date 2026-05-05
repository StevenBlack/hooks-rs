# Design Notes

## Hook chain — current state

The chain is generic over `T`: `Hookable<T>` / `Hook<T>`. Currently used with `T = String`.

Each hook implements:
- `execute(&self, value: T) -> T` — the hook's own logic
- `pre_process` / `post_process` — optional lifecycle overrides
- `process` — orchestrates pre → execute → post → process_next (delegates down the chain)
- `sethook` — appends a hook to the end of the chain

`HookStorage<T>` provides the `hook` field accessors. `impl_string_hook_storage!` generates the boilerplate.

---

## Extending to line-oriented processing

**Question:** Can `Lines<'a>` be passed through a `LineHook` chain?

**Conclusion:** No — `Lines<'a>` borrows from the source string and can't outlive a local `String` created inside a hook's `execute`. The practical owned alternative is `Vec<String>` (collected lines). A `LineHook` chain would use `T = Vec<String>`, with each hook receiving owned lines it can freely add, remove, or modify before passing forward.

---

## HookFlavor — heterogeneous content

**Proposal:** Replace `Hookable<T>` with a single `Hookable` implemented over an enum:

```rust
enum HookFlavor {
    Str(String),
    StrVec(Vec<String>),
    Bytes(Vec<u8>),
    ImageFile(PathBuf),
    PdfFile(PathBuf),
    Document(Vec<HookFlavor>),  // recursive — a document is a sequence of flavors
    // ...
}
```

**What this enables:**
- A single chain can hold hooks that operate on different content types
- A hook can transform the flavor mid-chain (e.g., a `SplitHook` converts `Str` → `StrVec`)
- `Document(Vec<HookFlavor>)` models a real document: text blocks, image blocks, metadata, all in sequence — a content AST

**What this trades away:**
- Compile-time chain homogeneity — the compiler no longer enforces that hooks are type-compatible
- Hooks must handle (or explicitly ignore) all variants; adding a new variant requires auditing existing hooks
- Silent no-ops are possible if a hook receives an unexpected flavor

**Expected usage pattern:**
95% of hooks will care only about `Str` or `StrVec` and ignore everything else. The framework (the `process` method) should handle `Document` traversal — detecting a `Document(vec)` variant and mapping the hook over each element — so hook authors only need to pattern-match on the leaf types they care about.

---

## Open decisions

1. **Traversal ownership:** Should the framework recurse into `Document(vec)` automatically, or should each hook decide its own traversal strategy? Framework-level traversal is simpler for hook authors; per-hook traversal is more flexible.

2. **Flat vs nested documents:** Is `Document(Vec<HookFlavor>)` the right model, or should the top-level chain type simply be `Vec<HookFlavor>` (always a sequence, no wrapper variant needed)?

3. **Binary content discrimination:** `Bytes(Vec<u8>)` is ambiguous — image bytes and PDF bytes look identical at the type level. The enum variants (`ImageFile`, `PdfFile`, `ImageBytes`, `PdfBytes`) should carry semantic meaning, not just shape.
