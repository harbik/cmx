use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents the command-line arguments for the xtask utility.
#[derive(Parser)]
#[command(name = "xtask", about = "Custom build tasks for the cmx library")]
struct XtaskArgs {
    #[command(subcommand)]
    command: Commands,
}

// Update Commands enum to include Gen with subcommand
#[derive(Subcommand)]
enum Commands {
    /// Check formatting, clippy, and build
    Check,
    /// Run tests
    Test,
    /// Builds the documentation, and opens it
    Doc,
    /// Build cmx-icc and publish it to npm as "cmx-icc"
    PublishNpm {
        /// Print what would be published without actually uploading
        #[arg(long)]
        dry_run: bool,
    },
    /// Run pre-publish checks and publish the cmx crate to crates.io
    PublishCrate {
        /// Run all checks and a cargo publish --dry-run without uploading
        #[arg(long)]
        dry_run: bool,
    },
}

impl Commands {
    fn handle(&self) {
        match self {
            Commands::Check => {
                run("cargo", &["fmt", "--", "--check"]);
                run(
                    "cargo",
                    &[
                        "clippy",
                        "--all-targets",
                        "--all-features",
                        "--",
                        "-D",
                        "warnings",
                    ],
                );
                run("cargo", &["check", "--all-targets"]);
                run("cargo", &["rdme", "--check"]);
            }
            Commands::Test => {
                run("cargo", &["test", "--all-features"]);
                run("cargo", &["test", "--no-default-features"]);
            }
            Commands::Doc => {
                check_or_force_rdme();
                run_env(
                    "cargo",
                    &["doc", "--all-features", "--no-deps"],
                    &[("RUSTDOCFLAGS", "--deny warnings")],
                )
                .expect("failed to run cargo doc");
                println!("✅ Documentation build complete");
            }
            Commands::PublishNpm { dry_run } => {
                let root = workspace_root();
                let icc_dir = root.join("cmx-icc");
                let pkg_dir = icc_dir.join("pkg");

                // Regenerate cmx-icc/README.md from src/lib.rs doc comment.
                // wasm-pack copies it into pkg/ so it lands on the npm page.
                check_or_force_rdme_workspace("cmx-icc");

                // Build
                run_in(
                    &root,
                    "wasm-pack",
                    &["build", "cmx-icc", "--target", "bundler", "--release"],
                );
                println!("✅ wasm-pack build complete");

                if *dry_run {
                    println!("Dry run — skipping npm publish.");
                    println!("Package would be published from: {}", pkg_dir.display());
                    let pkg_json = std::fs::read_to_string(pkg_dir.join("package.json"))
                        .expect("pkg/package.json not found after build");
                    println!("{pkg_json}");
                } else {
                    // Publish
                    run_in(
                        &root,
                        "npm",
                        &["publish", "cmx-icc/pkg", "--access", "public"],
                    );
                    println!("✅ Published cmx-icc to npm");
                }
            }
            Commands::PublishCrate { dry_run } => {
                // ── Pre-publish checks (mirrors CLAUDE.md release checklist) ──
                println!("Running tests...");
                run("cargo", &["test", "--all-features"]);
                run("cargo", &["test", "--doc"]);

                println!("Running clippy...");
                run(
                    "cargo",
                    &[
                        "clippy",
                        "--all-targets",
                        "--all-features",
                        "--",
                        "-D",
                        "warnings",
                    ],
                );

                println!("Checking docs...");
                run_env(
                    "cargo",
                    &["doc", "--all-features", "--no-deps"],
                    &[("RUSTDOCFLAGS", "--deny warnings")],
                )
                .expect("cargo doc failed");

                println!("Checking README...");
                check_or_force_rdme();

                // ── Publish ───────────────────────────────────────────────────
                if *dry_run {
                    run("cargo", &["publish", "-p", "cmx", "--dry-run"]);
                    println!("✅ Dry run complete — no files uploaded");
                } else {
                    run("cargo", &["publish", "-p", "cmx"]);
                    println!("✅ Published cmx to crates.io");
                }
            }
        }
    }
}

fn main() {
    let args = XtaskArgs::parse();
    args.command.handle();
}

/// Same as [`check_or_force_rdme`] but targets a specific workspace member
/// using `cargo rdme -w <project>`.
fn check_or_force_rdme_workspace(project: &str) {
    let status = Command::new("cargo")
        .args(["rdme", "-w", project, "--check"])
        .status()
        .unwrap_or_else(|e| panic!("failed to run cargo rdme: {e}"));

    if status.success() {
        println!("✅ {project}/README.md is up to date");
    } else {
        println!("⚠️  {project}/README.md is out of date, regenerating...");
        let force = Command::new("cargo")
            .args(["rdme", "-w", project, "--force"])
            .status()
            .unwrap_or_else(|e| panic!("failed to run cargo rdme --force: {e}"));

        if force.success() {
            println!("✅ {project}/README.md regenerated successfully");
        } else {
            eprintln!("❌ Failed to regenerate {project}/README.md");
            std::process::exit(force.code().unwrap_or(1));
        }
    }
}

fn check_or_force_rdme() {
    let status = Command::new("cargo")
        .args(["rdme", "--check"])
        .status()
        .expect("faid to run cargo rdme --check");

    if status.success() {
        println!("✅ README is up to date")
    } else {
        println!("⚠️ README is out of date, regenerating...");
        let force_status = Command::new("cargo")
            .args(["rdme", "--force"])
            .status()
            .expect("failed to run cargo rdme --force");

        if force_status.success() {
            println!("✅ README regenerated successfully");
        } else {
            eprintln!("❌ Failed to regenerate README");
            std::process::exit(force_status.code().unwrap_or(1));
        }
    }
}

fn run(cmd: &str, args: &[&str]) {
    println!("$ {} {}", cmd, args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect("failed to run command");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

/// Returns the workspace root (the directory that contains the root Cargo.toml).
/// When invoked via `cargo xtask`, CARGO_MANIFEST_DIR is the xtask crate directory,
/// so the workspace root is one level up.
fn workspace_root() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set — run this via `cargo xtask`");
    Path::new(&manifest_dir)
        .parent()
        .expect("xtask manifest has no parent directory")
        .to_path_buf()
}

/// Run `cmd args` from `dir`, exiting on failure.
fn run_in(dir: &Path, cmd: &str, args: &[&str]) {
    println!("$ {} {}", cmd, args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .status()
        .unwrap_or_else(|e| panic!("failed to start `{cmd}`: {e}"));
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_env(cmd: &str, args: &[&str], env: &[(&str, &str)]) -> Result<(), std::io::Error> {
    let mut c = Command::new(cmd);
    c.args(args);
    for (k, v) in env {
        c.env(k, v);
    }
    let status = c.status()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    } else {
        Ok(())
    }
}
