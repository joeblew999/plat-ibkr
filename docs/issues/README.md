# rust-ibapi Enhancement Proposals

Issues/suggestions for [wboayue/rust-ibapi](https://github.com/wboayue/rust-ibapi).

## Summary

| # | Title | Priority | Impact |
|---|-------|----------|--------|
| [001](001-hardcoded-client-ids.md) | Hardcoded Client IDs | High | Breaks multi-client usage |
| [002](002-no-structured-output.md) | No Structured Output | Medium | Limits automation |
| [003](003-missing-cli-args.md) | Missing CLI Arguments | Medium | Requires code edits |
| [004](004-example-args-pattern.md) | Example Args Pattern | High | Proposed solution |

## Quick Fix

Issue #004 proposes a lightweight pattern using environment variables that:
- Requires no new dependencies
- Is easy to implement across all 70+ examples
- Maintains focus on demonstrating the API

## To Submit

These can be submitted as GitHub issues or a single PR with:
1. Updated examples using the env var pattern
2. `OUTPUT` env var support for json/csv output
3. Updated doc comments showing usage

## Related

- [rust-ibapi examples](https://github.com/wboayue/rust-ibapi/tree/main/examples/sync)
- [ibapi crate docs](https://docs.rs/ibapi)
