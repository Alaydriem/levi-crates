<div align="center">

# levi-crates

[![Build](https://img.shields.io/github/actions/workflow/status/alaydriem/levi-crates/ci.yml?branch=master&label=build)](https://github.com/alaydriem/levi-crates/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/alaydriem/levi-crates)](./LICENSE)

</div>

The shared egui UI toolkit and translation runtime for Alaydriem's LeviLamina mods. The mods and
[Alaydriem's Mod Manager](https://github.com/alaydriem/levi-modmanager) use these crates by path, from a
`levi-crates` checkout next to their own.

| Crate | Path | Owns |
| --- | --- | --- |
| `levi-ui` | `crates/levi-ui/` | The general UI toolkit: the generated tokens and token rules, the theme and fonts, `Icon`, glyphs, `Draw`, `Motion`, the state model, every general widget (one per Paper sheet) and the widget gallery. |
| `levi-ui-gen` | `crates/levi-ui-gen/` | Writes the token types of `levi-ui` and `studio-ui` from `crates/levi-ui/design/tokens.json` and `ownership.json` (`design:generate`) |
| `levi-i18n` | `crates/levi-i18n/` | The translation runtime: `Message`, `Domain`, `Text`, `UserText`, `Args`, plural rules, `LocalizationService` |
| `levi-i18n-tool` | `crates/levi-i18n-tool/` | Build-time translation tools: `.pot` writing, pack compiling, the `en_XA` pseudo-locale, coverage reports, and `CatalogCheck` for the message tests |
| `levi-rules` | `crates/levi-rules/` | Test-only source scanners for the UI and design rules (rule 22): `DesignLiteralScan`, `WidgetNames` with `DuplicateScan`, `ForbiddenFolders`. |
| `studio-ui` | `crates/studio-ui/` | The studio tokens, glyphs and components, built on `levi-ui`. |

## Building

Requires Windows x64, [mise](https://mise.jdx.dev), rustup and [cargo-nextest](https://nexte.st).

```powershell
mise trust
mise run preflight
mise run check
mise run test
```

`mise run levi-ui:gallery` and `mise run studio-ui:gallery` open the widget galleries.

## License

BSD 3-Clause. See [LICENSE](./LICENSE). [Nordwand Mono](https://github.com/tywr/Nordwand-Mono) is under the SIL Open Font License 1.1.
