// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Kova — augment engine. GUI + serve.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kova", version)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run native GUI (egui).
    Gui(GuiArgs),
    /// Run HTTP API server. Web client at /.
    Serve(ServeArgs),
    /// Worker daemon for swarm. Phase 1: schema stub.
    Node,
    /// Tokenized orchestration. f18–f23 local or broadcast.
    C2(C2Args),
    /// Create ~/.kova, prompts, config. Run on first use.
    Bootstrap,
    /// Print loaded Cursor prompts (baked + external). For testing and debugging.
    Prompts,
    /// Model management (install Qwen2.5-Coder).
    Model(ModelArgs),
    /// Recent changes (f86). Tokenized for LLM context. Stay on task.
    Recent(RecentArgs),
    /// Autopilot: type prompt into Cursor composer. No API costs. Requires Cursor focused.
    Autopilot(AutopilotArgs),
    /// Deploy quality gate: clippy, TRIPLE SIMS, release build, smoke, baked demo. Requires --features tests.
    Test,
}

#[derive(clap::Args)]
struct C2Args {
    #[command(subcommand)]
    cmd: C2Cmd,
}

#[derive(Subcommand)]
enum C2Cmd {
    /// Run tokenized command (f18–f23).
    Run {
        #[arg(value_enum)]
        token: kova::c2::Token,
        #[arg(short, long)]
        project: Option<std::path::PathBuf>,
        #[arg(short, long)]
        broadcast: bool,
        #[arg(long)]
        release: bool,
        /// Restrict broadcast to specific nodes (e.g. lf,bt). Default: all reachable.
        #[arg(long)]
        nodes: Option<String>,
    },
    /// List worker nodes (lf gd bt st).
    Nodes,
    /// Inspect resources (CPU, RAM, disk, GPU) on c2-core + all workers.
    Inspect {
        /// Output JSON for scripting.
        #[arg(long)]
        json: bool,
    },
    /// Print placement recommendations based on inspect data.
    Recommend,
    /// Sync workspace to workers at /mnt/hive/projects/ (replaces sync-hive.sh).
    Sync {
        #[arg(long)]
        dry_run: bool,
        /// Target host (default: st).
        #[arg(long, default_value = "st")]
        target: String,
    },
    /// SSH host CA: init, sign, setup. No host key churn when IPs change.
    SshCa {
        #[command(subcommand)]
        cmd: SshCaCmd,
    },
}

#[derive(clap::Subcommand)]
enum SshCaCmd {
    /// Create CA key, add @cert-authority to known_hosts.
    Init,
    /// Sign host cert for one node, deploy, print sshd instructions.
    Sign {
        #[arg(value_name = "NODE")]
        node: String,
    },
    /// Init + sign all workers (lf gd bt st).
    Setup,
}

#[derive(clap::Args)]
struct GuiArgs {
    /// Enable demo mode: record actions to ~/.kova/demos/
    #[arg(long)]
    demo: bool,
}

#[derive(clap::Args)]
struct ServeArgs {
    /// Open browser to web client after starting
    #[arg(long)]
    open: bool,
    /// Enable demo mode: open with ?demo=1 for action recording
    #[arg(long)]
    demo: bool,
}

#[derive(clap::Args)]
struct AutopilotArgs {
    /// Prompt to type into Cursor agent composer
    #[arg(required = true)]
    prompt: Vec<String>,
}

#[derive(clap::Args)]
struct RecentArgs {
    /// Project directory (default: cwd)
    #[arg(short, long)]
    project: Option<std::path::PathBuf>,
    /// Files modified in last N minutes (default: 30)
    #[arg(short, long, default_value = "30")]
    minutes: u64,
}

#[derive(clap::Args)]
struct ModelArgs {
    #[command(subcommand)]
    cmd: ModelCmd,
}

#[derive(Subcommand)]
enum ModelCmd {
    /// Download Qwen2.5-Coder-0.5B-Instruct GGUF to ~/.kova/models/
    Install,
    /// List configured models and paths (router, coder, fix).
    List,
}

#[cfg(feature = "gui")]
fn run_gui(demo: bool) -> anyhow::Result<()> {
    kova::bootstrap()?;
    kova::gui::run(demo)
}

#[cfg(not(feature = "gui"))]
fn run_gui(_demo: bool) -> anyhow::Result<()> {
    anyhow::bail!("Build with --features gui for GUI mode")
}

#[cfg(feature = "tests")]
fn run_test() -> anyhow::Result<()> {
    // Delegate to kova-test binary to avoid tokio runtime nesting (baked demo uses block_on).
    let exe = std::env::current_exe()?;
    let test_bin = exe
        .parent()
        .unwrap()
        .join("kova-test")
        .with_extension(std::env::consts::EXE_EXTENSION);
    if test_bin.exists() {
        let status = std::process::Command::new(&test_bin).status()?;
        std::process::exit(status.code().unwrap_or(1));
    }
    kova::run_test_suite()
}

#[cfg(feature = "serve")]
async fn run_serve(open: bool, demo: bool) -> anyhow::Result<()> {
    let addr = kova::bind_addr();
    if open {
        let url = if demo {
            format!("http://{}?demo=1", addr)
        } else {
            format!("http://{}", addr)
        };
        kova::serve::run_with_open(addr, &url).await
    } else {
        kova::serve::run(addr).await
    }
}

#[cfg(not(feature = "serve"))]
async fn run_serve(_open: bool, _demo: bool) -> anyhow::Result<()> {
    anyhow::bail!("Build with --features serve for serve mode")
}

#[cfg(feature = "daemon")]
fn run_node() -> anyhow::Result<()> {
    kova::daemon::run();
    Ok(())
}

#[cfg(not(feature = "daemon"))]
fn run_node() -> anyhow::Result<()> {
    anyhow::bail!("Build with --features daemon for node mode")
}

async fn run_c2(args: C2Args) -> anyhow::Result<()> {
    kova::bootstrap()?;
    match args.cmd {
        C2Cmd::Nodes => {
            kova::c2::run_nodes();
            Ok(())
        }
        C2Cmd::Run {
            token,
            project,
            broadcast,
            release,
            nodes,
        } => kova::c2::run_command(token, project, broadcast, release, nodes),
        C2Cmd::Inspect { json } => {
            let hosts = kova::inspect::run_inspect();
            if json {
                kova::inspect::print_json(&hosts);
            } else {
                kova::inspect::print_table(&hosts);
            }
            Ok(())
        }
        C2Cmd::Recommend => {
            let hosts = kova::inspect::run_inspect();
            kova::inspect::print_table(&hosts);
            kova::inspect::print_recommend(&hosts);
            Ok(())
        }
        C2Cmd::Sync { dry_run, target } => kova::c2::run_sync(dry_run, &target),
        C2Cmd::SshCa { cmd } => match cmd {
            SshCaCmd::Init => kova::ssh_ca::run_init(),
            SshCaCmd::Sign { node } => kova::ssh_ca::run_sign(&node),
            SshCaCmd::Setup => kova::ssh_ca::run_setup(),
        },
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "serve")]
    {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    }

    let args = Args::parse();
    let cmd = args.cmd;

    match cmd {
        Some(Cmd::Gui(args)) => run_gui(args.demo),
        Some(Cmd::Serve(args)) => run_serve(args.open, args.demo).await,
        Some(Cmd::Node) => run_node(),
        Some(Cmd::C2(args)) => run_c2(args).await,
        Some(Cmd::Model(args)) => match args.cmd {
            ModelCmd::Install => {
                #[cfg(feature = "inference")]
                {
                    kova::bootstrap()?;
                    kova::model::f77().await
                }
                #[cfg(not(feature = "inference"))]
                {
                    anyhow::bail!("Build with --features inference for model install")
                }
            }
            ModelCmd::List => {
                kova::bootstrap()?;
                for (role, path) in [
                    ("router", kova::f78(kova::ModelRole::Router)),
                    ("coder", kova::f78(kova::ModelRole::Coder)),
                    ("fix", kova::f78(kova::ModelRole::Fix)),
                ] {
                    match path {
                        Some(p) => eprintln!("  {}: {}", role, p.display()),
                        None => eprintln!("  {}: (not found)", role),
                    }
                }
                eprintln!("  orchestration: max_fix_retries={} run_clippy={}",
                    kova::orchestration_max_fix_retries(),
                    kova::orchestration_run_clippy());
                Ok(())
            }
        }
        Some(Cmd::Bootstrap) => {
            kova::bootstrap()?;
            eprintln!("Bootstrap complete. ~/.kova/ ready.");
            eprintln!("  prompts: ~/.kova/prompts/");
            eprintln!("  config:  ~/.kova/config.toml");
            Ok(())
        }
        Some(Cmd::Prompts) => {
            kova::bootstrap()?;
            let project = kova::default_project();
            let out = kova::cursor_prompts::load_cursor_prompts(&project);
            if out.is_empty() {
                eprintln!("Prompts disabled (config [cursor] prompts_enabled = false or no rules found)");
            } else {
                print!("{}", out);
            }
            Ok(())
        }
        Some(Cmd::Autopilot(autopilot_args)) => {
            #[cfg(feature = "autopilot")]
            {
                kova::autopilot::run(autopilot_args.prompt.join(" "))
            }
            #[cfg(not(feature = "autopilot"))]
            {
                let _ = autopilot_args;
                anyhow::bail!("Build with --features autopilot for autopilot mode")
            }
        }
        #[cfg(feature = "tests")]
        Some(Cmd::Test) => run_test(),
        #[cfg(not(feature = "tests"))]
        Some(Cmd::Test) => {
            anyhow::bail!("Build with --features tests for kova test")
        }
        Some(Cmd::Recent(args)) => {
            let project = args
                .project
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
            let changes = kova::recent_changes::f86(
                &project,
                std::time::Duration::from_secs(args.minutes * 60),
            );
            let out = kova::recent_changes::f87(&changes);
            if out.is_empty() {
                eprintln!("No files modified in last {} minutes.", args.minutes);
            } else {
                print!("{}", out);
            }
            Ok(())
        }
        None => {
            #[cfg(feature = "gui")]
            {
                run_gui(false)
            }
            #[cfg(not(feature = "gui"))]
            {
                eprintln!("Usage: kova <COMMAND>");
                eprintln!("  kova gui   — native GUI");
                eprintln!("  kova serve — HTTP API");
                eprintln!("Build with --features gui for GUI, --features serve for serve.");
                Ok(())
            }
        }
    }
}
