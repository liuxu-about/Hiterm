use anyhow::{anyhow, bail, Context};
use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Parser, Clone, Default)]
pub struct InitCommand {
    /// Refresh shell integration without interactive prompts
    #[arg(long)]
    pub update_only: bool,
}

impl InitCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        imp::run(self.update_only)
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    use anyhow::bail;

    pub fn run(_update_only: bool) -> anyhow::Result<()> {
        bail!("`hiterm init` is currently supported on macOS only")
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use super::*;
    use crate::shell::{detect_shell_kind, ShellKind};
    use std::os::unix::fs::PermissionsExt;

    pub fn run(update_only: bool) -> anyhow::Result<()> {
        ensure_user_config().context("ensure user config exists")?;

        install_cli_wrapper().context("install hiterm wrapper")?;
        remove_legacy_wrappers();

        let shell = detect_shell_kind();
        let script_name = match shell {
            ShellKind::Fish => "setup_fish.sh",
            _ => "setup_zsh.sh",
        };
        let script = resolve_setup_script(script_name)
            .ok_or_else(|| anyhow!("failed to locate {} for Hiterm initialization", script_name))?;

        let mut cmd = Command::new("/bin/bash");
        cmd.arg(&script)
            .env("HITERM_INIT_INTERNAL", "1")
            .env("KAKU_INIT_INTERNAL", "1");
        if update_only {
            cmd.arg("--update-only");
        }
        let status = cmd
            .status()
            .with_context(|| format!("run {}", script.display()))?;

        if status.success() {
            return Ok(());
        }

        bail!("hiterm init failed with status {}", status);
    }

    fn install_cli_wrapper() -> anyhow::Result<()> {
        let wrapper_path = wrapper_path();
        let wrapper_dir = wrapper_path
            .parent()
            .ok_or_else(|| anyhow!("invalid wrapper path"))?;
        config::create_user_owned_dirs(wrapper_dir).context("create wrapper directory")?;

        if fs::symlink_metadata(&wrapper_path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            fs::remove_file(&wrapper_path).with_context(|| {
                format!("remove legacy symlink wrapper {}", wrapper_path.display())
            })?;
        }

        let preferred_bin = resolve_preferred_cli_bin()
            .unwrap_or_else(|| PathBuf::from("/Applications/Hiterm.app/Contents/MacOS/hiterm"));
        let preferred_bin = escape_for_double_quotes(&preferred_bin.display().to_string());

        let script = format!(
            r#"#!/bin/bash
set -euo pipefail

if [[ -n "${{HITERM_BIN:-${{KAKU_BIN:-}}}}" && -x "${{HITERM_BIN:-${{KAKU_BIN:-}}}}" ]]; then
	exec "${{HITERM_BIN:-${{KAKU_BIN:-}}}}" "$@"
fi

for candidate in \
	"{preferred_bin}" \
	"/Applications/Hiterm.app/Contents/MacOS/hiterm" \
	"$HOME/Applications/Hiterm.app/Contents/MacOS/hiterm"; do
	if [[ -n "$candidate" && -x "$candidate" ]]; then
		exec "$candidate" "$@"
	fi
done

echo "hiterm: Hiterm.app not found. Expected /Applications/Hiterm.app." >&2
exit 127
"#
        );

        let mut file = fs::File::create(&wrapper_path)
            .with_context(|| format!("create wrapper {}", wrapper_path.display()))?;
        file.write_all(script.as_bytes())
            .with_context(|| format!("write wrapper {}", wrapper_path.display()))?;
        fs::set_permissions(&wrapper_path, fs::Permissions::from_mode(0o755))
            .with_context(|| format!("chmod wrapper {}", wrapper_path.display()))?;
        Ok(())
    }

    /// Remove the wrappers older releases installed: the AI-chat `k`
    /// entry point (dropped entirely) and the pre-rename `kaku` CLI
    /// (replaced by `hiterm`). Only files that look like our generated
    /// wrappers are touched; anything user-owned is left alone.
    fn remove_legacy_wrappers() {
        for shell_dir in ["zsh", "fish"] {
            for name in ["k", "kaku"] {
                // The hiterm config dir; pre-migration installs reach the
                // same tree through the ~/.config/kaku symlink.
                let path = config::HOME_DIR
                    .join(".config")
                    .join("hiterm")
                    .join(shell_dir)
                    .join("bin")
                    .join(name);
                let is_symlink = fs::symlink_metadata(&path)
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false);
                let is_our_wrapper = fs::read_to_string(&path)
                    .map(|content| content.contains("Kaku") || content.contains("Hiterm"))
                    .unwrap_or(false);
                if is_symlink || is_our_wrapper {
                    let _ = fs::remove_file(&path);
                }
            }
        }
    }

    fn wrapper_path() -> PathBuf {
        let dir = match detect_shell_kind() {
            ShellKind::Fish => "fish",
            _ => "zsh",
        };
        config::HOME_DIR
            .join(".config")
            .join("hiterm")
            .join(dir)
            .join("bin")
            .join("hiterm")
    }

    fn resolve_preferred_cli_bin() -> Option<PathBuf> {
        for var in ["HITERM_BIN", "KAKU_BIN"] {
            if let Some(path) = std::env::var_os(var) {
                let path = PathBuf::from(path);
                if is_executable_file(&path) {
                    return Some(path);
                }
            }
        }

        if let Ok(exe) = std::env::current_exe() {
            if exe
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.eq_ignore_ascii_case("hiterm") || n.eq_ignore_ascii_case("kaku"))
                .unwrap_or(false)
                && is_executable_file(&exe)
            {
                return Some(exe);
            }
        }

        for candidate in [
            PathBuf::from("/Applications/Hiterm.app/Contents/MacOS/hiterm"),
            config::HOME_DIR
                .join("Applications")
                .join("Hiterm.app")
                .join("Contents")
                .join("MacOS")
                .join("hiterm"),
        ] {
            if is_executable_file(&candidate) {
                return Some(candidate);
            }
        }

        None
    }

    fn is_executable_file(path: &Path) -> bool {
        fs::metadata(path)
            .map(|meta| meta.is_file() && (meta.permissions().mode() & 0o111 != 0))
            .unwrap_or(false)
    }

    fn escape_for_double_quotes(value: &str) -> String {
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('$', "\\$")
            .replace('`', "\\`")
    }

    fn resolve_setup_script(script_name: &str) -> Option<PathBuf> {
        let mut candidates = Vec::new();

        if let Ok(cwd) = std::env::current_dir() {
            candidates.push(
                cwd.join("assets")
                    .join("shell-integration")
                    .join(script_name),
            );
        }

        if let Ok(exe) = std::env::current_exe() {
            if let Some(contents_dir) = exe.parent().and_then(|p| p.parent()) {
                candidates.push(contents_dir.join("Resources").join(script_name));
            }
        }

        candidates.push(PathBuf::from(format!(
            "/Applications/Hiterm.app/Contents/Resources/{}",
            script_name
        )));
        candidates.push(
            config::HOME_DIR
                .join("Applications")
                .join("Hiterm.app")
                .join("Contents")
                .join("Resources")
                .join(script_name),
        );

        candidates.into_iter().find(|p| p.exists())
    }

    fn ensure_user_config() -> anyhow::Result<()> {
        config::ensure_user_config_exists().context("ensure user config exists")?;
        Ok(())
    }
}
