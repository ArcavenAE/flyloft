//! flyloft-mcp
//!
//! MCP server exposing Flyloft's agent-facing tool surface.
//! See `docs/MCP.md` for the tool spec.
//!
//! Tools exposed:
//!   - flyloft_fly
//!   - flyloft_fetch
//!   - flyloft_contribute
//!   - flyloft_annotate
//!   - flyloft_dispute
//!   - flyloft_cite
//!   - flyloft_entities
//!
//! Privileged operations (strike, spike, dog, promote) are deliberately
//! NOT exposed here — they are flyperson-only via the CLI.

use anyhow::Result;
use flyloft_core::Grid;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let grid_root = std::env::var("FLYLOFT_ROOT")
        .unwrap_or_else(|_| ".".to_string());
    let _grid = Grid::open(grid_root)?;

    tracing::info!("flyloft-mcp starting (stub)");

    // TODO(phase 1): wire up MCP SDK, register tools.
    //
    // Sketch:
    //   server.tool("flyloft_fly", fly_handler);
    //   server.tool("flyloft_fetch", fetch_handler);
    //   server.tool("flyloft_contribute", contribute_handler);
    //   server.tool("flyloft_annotate", annotate_handler);
    //   server.tool("flyloft_dispute", dispute_handler);
    //   server.tool("flyloft_cite", cite_handler);
    //   server.tool("flyloft_entities", entities_handler);
    //   server.serve(transport).await?;

    Ok(())
}
