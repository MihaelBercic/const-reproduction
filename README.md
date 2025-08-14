`note: These settings are formatted for the Zed editor, for VSC and others, they're just in different json format.`

The struct and field in question is located at: `crate::core::matter_message::header::MatterMessageHeader` the `message_extensions` field.
Turns out that r-a can "calculate" the size of the struct field only on some targets:
```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "target": "riscv32imac-unknown-none-elf", // It just says "No Drop" from lsp
          // "target": "aarch64-unknown-none", // It says "size = 11 (0xB), align = 0x1, offset = 0x0, no Drop", which is correct
          "allTargets": false
        }
      }
    }
  }
}
```
