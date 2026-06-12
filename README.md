<div align="center">
  <img src="assets/logo.png" width="120" alt="Hiterm logo" />
  <h1>Hiterm</h1>
  <p><em>A fast macOS terminal built for AI coding workflows.</em></p>
</div>

<p align="center">
  <a href="https://github.com/liuxu-about/Hiterm/stargazers"><img src="https://img.shields.io/github/stars/liuxu-about/Hiterm?style=flat-square" alt="Stars"></a>
  <a href="https://github.com/liuxu-about/Hiterm/releases"><img src="https://img.shields.io/github/v/tag/liuxu-about/Hiterm?label=version&style=flat-square" alt="Version"></a>
  <a href="LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://github.com/liuxu-about/Hiterm/commits"><img src="https://img.shields.io/github/commit-activity/m/liuxu-about/Hiterm?style=flat-square" alt="Commits"></a>
</p>

<p align="center">
  <img src="assets/hiterm.jpg" alt="Hiterm Screenshot" width="1000" />
</p>

## Why

Hiterm is a macOS-native terminal emulator tuned for AI-assisted development. It keeps WezTerm-compatible Lua configuration and a GPU-accelerated terminal core, then adds practical defaults, curated shell integration, pane and tab ergonomics, and built-in AI command helpers.

## Features

- **Zero Config**: Defaults with JetBrains Mono, macOS font rendering, and low-res font sizing.
- **Theme-Aware Experience**: Auto-switches between dark and light modes with macOS, with tuned selection colors, font weight, and practical color overrides support.
- **Curated Shell Suite**: Built-in zsh plugins with optional CLI tools for prompt, diff, and navigation workflows.
- **Fast & Lightweight**: Smaller binary, instant startup, lazy loading, stripped-down GPU-accelerated core.
- **WezTerm-Compatible Config**: Use WezTerm's Lua config directly with full API compatibility.
- **Polished Defaults**: Copy on select, clickable file paths, history peek from full-screen apps, pane input broadcast, and visual bell on background tab completion.

## Quick Start

1. [Download the latest Hiterm DMG](https://github.com/liuxu-about/Hiterm/releases/latest) and drag `Hiterm.app` to Applications.
2. Open Hiterm. The app is notarized by Apple, so it opens without security warnings.
3. On first launch, Hiterm automatically sets up your shell environment.

## Usage Guide

| Action | Shortcut |
| :--- | :--- |
| New Tab | `Cmd + T` |
| New Window | `Cmd + N` |
| Close Tab/Pane | `Cmd + W` |
| Navigate Tabs | `Cmd + Shift + [` / `]` or `Cmd + 1-9` |
| Navigate Panes | `Cmd + Opt + Arrows` |
| Split Pane Vertical | `Cmd + D` |
| Split Pane Horizontal | `Cmd + Shift + D` |
| Open Settings Panel | `Cmd + ,` |
| AI Panel | `Cmd + Shift + A` |
| Apply AI Suggestion | `Cmd + Shift + E` |
| Open Lazygit | `Cmd + Shift + G` |
| Yazi File Manager | `Cmd + Shift + Y` or `y` |
| Clear Screen | `Cmd + K` |

Full keybinding reference: [docs/keybindings.md](docs/keybindings.md)

## Hiterm AI

Hiterm has a built-in assistant with two modes and a settings page for AI coding tools.

- **Error recovery**: When a command fails, Hiterm automatically suggests a fix. Press `Cmd + Shift + E` to apply.
- **Natural language to command**: Type `# <description>` at the prompt and press Enter. Hiterm sends the query to the LLM and injects the resulting command back into the prompt, ready to review and run.
- **AI Tools Config**: Manage settings for Claude Code, Codex, Gemini CLI, Copilot CLI, Kimi Code, and more.

### Provider Presets

Select a provider in `hiterm ai` to auto-fill the base URL and models:

| Provider | Base URL | Models |
| :--- | :--- | :--- |
| OpenAI | `https://api.openai.com/v1` | (free text) |
| Custom | (manual) | (manual) |

Full AI assistant docs: [docs/features.md](docs/features.md)

## Performance

| Metric | Upstream | Hiterm | Methodology |
| :--- | :--- | :--- | :--- |
| **Executable Size** | ~67 MB | ~40 MB | Aggressive symbol stripping and feature pruning |
| **Resources Volume** | ~100 MB | ~80 MB | Asset optimization and lazy-loaded assets |
| **Launch Latency** | Standard | Instant | Just-in-time initialization |
| **Shell Bootstrap** | ~200ms | ~100ms | Optimized environment provisioning |

## FAQ

**Is there a Windows or Linux version?** Not currently. Hiterm is macOS-only for now.

**Can I use transparent windows?** Yes, set `config.window_background_opacity` in `~/.config/hiterm/hiterm.lua`.

**The `hiterm` command is missing.** Run `/Applications/Hiterm.app/Contents/MacOS/hiterm init --update-only && exec zsh -l`, then `hiterm doctor`.

Full FAQ: [docs/faq.md](docs/faq.md)

## Docs

- [Keybindings](docs/keybindings.md) - full shortcut reference
- [Features](docs/features.md) - AI assistant, lazygit, yazi, remote files, shell suite
- [Configuration](docs/configuration.md) - themes, fonts, custom keybindings, Lua API
- [CLI Reference](docs/cli.md) - `hiterm ai`, `hiterm config`, `hiterm doctor`, and more
- [FAQ](docs/faq.md) - common questions and troubleshooting

## Background

Hiterm is designed around fast terminal startup, native macOS behavior, strong tab and pane ergonomics, and command-line workflows that increasingly include AI coding tools.

WezTerm provides a robust and highly hackable terminal foundation. Hiterm builds on that foundation with product defaults and shell integration aimed at day-to-day AI-assisted development.

## Contributors

Thanks to everyone whose work made Hiterm possible.

<a href="https://github.com/liuxu-about/Hiterm/graphs/contributors">
  <img src="./CONTRIBUTORS.svg?v=2" width="1000" />
</a>

## Support

- If Hiterm helps you, star this repository and share it with friends.
- Got ideas or bugs? Open an issue or PR.

## License

MIT License, feel free to enjoy and participate in open source.
