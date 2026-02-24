/*
 * Copyright (c) 2026 CX Linux
 * Licensed under the Business Source License 1.1
 * You may not use this file except in compliance with the License.
 */

//! HRM AI Agent Termination Command
//!
//! Safely terminates AI agents with confirmation and audit logging.
//! This module is only available when the `hrm` feature is enabled.
//!
//! # Example
//! ```bash
//! # Terminate an agent by ID
//! cx fire abc123-def456
//!
//! # Force termination (skip confirmation)
//! cx fire abc123-def456 --force
//!
//! # Terminate with reason
//! cx fire abc123-def456 --reason "Migrating to new server"
//! ```

use anyhow::Result;
use clap::Parser;

#[cfg(feature = "hrm")]
use hrm_ai::{
    database::{DatabaseConnection, PostgresAgentRepository},
    fire::{AgentTerminationService, FireCommand as HrmFireCommand, TerminationConfig},
    theme::SovereignTheme,
};

/// Terminate (fire) an AI agent
#[derive(Debug, Parser, Clone)]
pub struct FireCommand {
    /// Agent ID or Server ID to terminate
    pub target: String,

    /// Skip confirmation prompt (dangerous)
    #[arg(long, short = 'f')]
    pub force: bool,

    /// Reason for termination (for audit log)
    #[arg(long, short = 'r')]
    pub reason: Option<String>,

    /// Termination type: immediate, graceful, force
    #[arg(long, short = 't', default_value = "graceful")]
    pub termination_type: String,

    /// Dry run mode (validate but don't terminate)
    #[arg(long)]
    pub dry_run: bool,

    /// Output format: table, json
    #[arg(long, default_value = "table")]
    pub format: String,
}

impl FireCommand {
    pub fn run(self) -> Result<()> {
        #[cfg(feature = "hrm")]
        {
            run_fire_with_hrm(self)
        }

        #[cfg(not(feature = "hrm"))]
        {
            println!(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            );
            println!("  🔒 HRM AI Premium Feature");
            println!(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            );
            println!();
            println!("  The 'fire' command requires the HRM AI premium module.");
            println!();
            println!("  To enable HRM AI capabilities, rebuild with:");
            println!("    cargo build --features hrm");
            println!();
            println!("  HRM AI Features:");
            println!("    • cx hire <agent-type>  - Deploy AI agents");
            println!("    • cx fire <agent-id>    - Terminate agents");
            println!("    • PostgreSQL integration for fleet management");
            println!("    • Enterprise compliance automation");
            println!();
            println!("  License: BSL 1.1 (Business Source License)");
            println!(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            );
            Ok(())
        }
    }
}

#[cfg(feature = "hrm")]
fn run_fire_with_hrm(cmd: FireCommand) -> Result<()> {
    use tokio::runtime::Runtime;

    let rt = Runtime::new()?;
    rt.block_on(async {
        let theme = SovereignTheme::default();

        // Print header
        println!();
        println!(
            "  {}┌─ CX Linux Agent Termination ────────────────────────────────────────┐{}",
            theme.primary.ansi_fg, "\x1b[0m"
        );
        println!(
            "  {}│                                                                      │{}",
            theme.primary.ansi_fg, "\x1b[0m"
        );

        println!();
        println!("  ⚠️  Termination Request");
        println!("  ─────────────────────────────────────");
        println!("  Target:      {}", cmd.target);
        println!("  Type:        {}", cmd.termination_type);
        if let Some(ref reason) = cmd.reason {
            println!("  Reason:      {}", reason);
        }
        if cmd.dry_run {
            println!("  Mode:        DRY RUN");
        }
        println!();

        // Get database URL from environment
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/cx_agents".to_string());

        // Create database connection and repository
        let db_conn = DatabaseConnection::new(&db_url)
            .await
            .map_err(|e| anyhow::anyhow!("Database connection failed: {}", e))?;
        let repository = PostgresAgentRepository::new(db_conn);

        // Create termination config
        let config = TerminationConfig {
            graceful_timeout_seconds: 30,
            cleanup_data: true,
            notify_stakeholders: true,
        };

        // Create termination service
        let service = AgentTerminationService::new(repository, config);

        // Build HRM fire command
        let hrm_cmd = HrmFireCommand {
            target: cmd.target.clone(),
            reason: cmd.reason.clone(),
            termination_type: cmd.termination_type.clone(),
            force: cmd.force,
            dry_run: cmd.dry_run,
        };

        // Terminate agent(s)
        if !cmd.dry_run {
            println!("  🔥 Initiating {} termination...", cmd.termination_type);
        } else {
            println!("  🔍 Validating termination (dry run)...");
        }

        let results = service
            .fire_agent(&hrm_cmd)
            .await
            .map_err(|e| anyhow::anyhow!("Termination failed: {}", e))?;

        println!();

        if results.is_empty() {
            println!("  ❌ No agents were terminated (cancelled or not found)");
        } else {
            println!(
                "  {}✅ Termination complete!{}",
                theme.success.ansi_fg, "\x1b[0m"
            );
            println!();
            println!("  Agents terminated: {}", results.len());
            for status in &results {
                println!("    • {:?}", status);
            }
        }
        println!();

        Ok(())
    })
}
