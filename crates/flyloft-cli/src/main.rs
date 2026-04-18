//! flyloft
//!
//! The flyperson's CLI. See `docs/CLI.md` for the full command spec.
//!
//! Commands mirror the theatrical verbs: rig, fly, spike, strike, dog,
//! annotate, dispute, groom, weed, promote. Plus lifecycle (init, serve,
//! status) and introspection (cue, entity, config, doctor, reindex).

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "flyloft")]
#[command(about = "A retrieval substrate for human-AI teams", long_about = None)]
#[command(version)]
struct Cli {
    /// Path to the flyloft grid. Defaults to `$FLYLOFT_ROOT` or `.`.
    #[arg(long, global = true)]
    root: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    // -- lifecycle --
    /// Initialize a new flyloft at the current directory.
    Init(InitArgs),
    /// Start the MCP server (and optional REST endpoint).
    Serve(ServeArgs),
    /// Print a one-screen status summary.
    Status,

    // -- rigging --
    /// Ingest material into the grid.
    Rig(RigArgs),
    /// Review, accept, or reject agent-contributed material.
    Contribute(ContributeArgs),

    // -- retrieval --
    /// Fly material in for a query (for spot-checks from the terminal).
    Fly(FlyArgs),

    // -- curation (privileged) --
    /// Mark a batten as authoritative.
    Spike { batten_id: String, #[arg(long)] reason: Option<String> },
    /// Retire a batten or line set.
    Strike { id: String, #[arg(long)] reason: Option<String> },
    /// Lock a line set against modification.
    Dog { line_set_id: String, #[arg(long)] release: bool },
    /// Attach a note to a batten.
    Annotate { batten_id: String, #[arg(long)] note: Option<String> },
    /// Register or resolve a dispute.
    #[command(subcommand)]
    Dispute(DisputeCmd),
    /// Promote a spiked batten into KOS.
    Promote { batten_id: String, #[arg(long)] to: String },

    /// Convert a cataloged batten into a held batten by fetching and
    /// persisting its current content into the stacks.
    Adopt { batten_id: String, #[arg(long)] reason: Option<String> },

    // -- catalogs --
    #[command(subcommand)]
    Catalog(CatalogCmd),

    // -- grooming --
    /// Interactive grooming session.
    Groom,
    /// Report retirement candidates.
    Weed(WeedArgs),
    /// Query the cue sheet.
    #[command(subcommand)]
    Cue(CueCmd),

    // -- entities --
    #[command(subcommand)]
    Entity(EntityCmd),

    // -- config / ops --
    #[command(subcommand)]
    Config(ConfigCmd),
    /// Health-check the grid.
    Doctor { #[arg(long)] fix: bool },
    /// Rebuild the derived index from the grid.
    Reindex { #[arg(long)] only: Option<String> },
}

#[derive(clap::Args)]
struct InitArgs {
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    remote: Option<String>,
}

#[derive(clap::Args)]
struct ServeArgs {
    #[arg(long)]
    mcp: bool,
    #[arg(long)]
    rest: bool,
    #[arg(long, default_value_t = 7070)]
    port: u16,
}

#[derive(clap::Args)]
struct RigArgs {
    /// Source path, URL, or omitted when using --catalog.
    source: Option<String>,
    #[arg(long)]
    tag: Vec<String>,
    #[arg(long)]
    rigged_by: Option<String>,
    #[arg(long)]
    chunk_strategy: Option<String>,
    #[arg(long)]
    dry_run: bool,
    /// Rig a slice from a registered catalog instead of a path/URL.
    #[arg(long)]
    catalog: Option<String>,
    /// Query for the catalog slice (required with --catalog unless --collection-id).
    #[arg(long)]
    query: Option<String>,
    /// Adapter-specific collection identifier (e.g. a Confluence space key).
    #[arg(long)]
    collection_id: Option<String>,
}

#[derive(Subcommand)]
enum ContributeCmd {
    /// List pending agent contributions.
    Review,
    /// Rig a pending contribution.
    Accept { pending_id: String },
    /// Discard a pending contribution.
    Reject { pending_id: String, #[arg(long)] reason: Option<String> },
    /// Hold for later.
    Defer { pending_id: String },
}

#[derive(clap::Args)]
struct ContributeArgs {
    #[command(subcommand)]
    cmd: ContributeCmd,
}

#[derive(clap::Args)]
struct FlyArgs {
    query: String,
    #[arg(long, default_value_t = 8)]
    k: usize,
    #[arg(long)]
    filter: Vec<String>,
    /// Source selection: all (default), only (catalogs only), none (stacks only).
    #[arg(long, default_value = "all")]
    catalogs: String,
    /// Restrict to a specific catalog (repeatable). Implies catalogs scope.
    #[arg(long)]
    catalog: Vec<String>,
}

#[derive(Subcommand)]
enum DisputeCmd {
    /// Register a new dispute.
    Open { batten_id: String, #[arg(long)] reason: String },
    /// Resolve an open dispute.
    Resolve {
        dispute_id: String,
        #[arg(long)] action: String, // strike | annotate | dismiss
        #[arg(long)] note: Option<String>,
    },
}

#[derive(clap::Args)]
struct WeedArgs {
    #[arg(long)]
    stale: Option<String>,
    #[arg(long)]
    disused: bool,
    #[arg(long)]
    superseded: bool,
}

#[derive(Subcommand)]
enum CueCmd {
    Recent { #[arg(long, default_value_t = 50)] n: usize },
    Gaps,
    Coldspots,
    Hotspots,
    ByRequester { id: String },
}

#[derive(Subcommand)]
enum EntityCmd {
    List { #[arg(long)] kind: Option<String> },
    Show { name: String },
    Merge { from: String, to: String },
}

#[derive(Subcommand)]
enum CatalogCmd {
    /// List all registered catalogs with health status.
    List,
    /// Ping each adapter and report status.
    Health,
    /// Show full details for one catalog (adapter config, cache policy, recent usage).
    Describe { id: String },
    /// Register a new catalog.
    Add {
        #[arg(long)] id: String,
        #[arg(long)] adapter: String,
        #[arg(long)] federate: bool,
        /// Key=value pairs for adapter-specific config (repeatable).
        #[arg(long)] config: Vec<String>,
    },
    /// Unregister a catalog. Cataloged battens referencing it become unresolvable.
    Remove { id: String, #[arg(long)] force: bool },
}

#[derive(Subcommand)]
enum ConfigCmd {
    Show,
    Set { key: String, value: String },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Init(_) => todo!("phase 0"),
        Command::Serve(_) => todo!("phase 1"),
        Command::Status => todo!("phase 0"),
        Command::Rig(_) => todo!("phase 0"),
        Command::Contribute(_) => todo!("phase 2"),
        Command::Fly(_) => todo!("phase 1"),
        Command::Spike { .. } => todo!("phase 2"),
        Command::Strike { .. } => todo!("phase 2"),
        Command::Dog { .. } => todo!("phase 2"),
        Command::Annotate { .. } => todo!("phase 2"),
        Command::Dispute(_) => todo!("phase 2"),
        Command::Promote { .. } => todo!("phase 5"),
        Command::Adopt { .. } => todo!("phase 1.5: fetch from catalog, persist as held batten"),
        Command::Catalog(_) => todo!("phase 1.5"),
        Command::Groom => todo!("phase 3"),
        Command::Weed(_) => todo!("phase 3"),
        Command::Cue(_) => todo!("phase 3"),
        Command::Entity(_) => todo!("phase 4"),
        Command::Config(_) => todo!("phase 0"),
        Command::Doctor { .. } => todo!("phase 0"),
        Command::Reindex { .. } => todo!("phase 1"),
    }
}
