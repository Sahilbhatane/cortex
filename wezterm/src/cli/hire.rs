/*
 * Copyright (c) 2026 CX Linux
 * Licensed under the Business Source License 1.1
 * You may not use this file except in compliance with the License.
 */

//! HRM AI Agent Hiring Command
//!
//! Deploys AI agents to managed servers with enterprise-grade compliance.
//! This module is only available when the `hrm` feature is enabled.
//!
//! # Example
//! ```bash
//! # Deploy a DevOps agent
//! cx hire devops --server prod-1 --name "Deploy Bot"
//!
//! # Deploy with custom configuration
//! cx hire security --server sec-cluster --capabilities audit,scan,patch
//! ```

use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

#[cfg(feature = "hrm")]
use hrm_ai::{
    database::{DatabaseConnection, PostgresAgentRepository},
    hire::{AgentHiringService, HireCommand as HrmHireCommand, HireConfig},
    theme::SovereignTheme,
};

/// Agent types available for deployment
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum AgentType {
    /// DevOps automation agent
    Devops,
    /// Security monitoring agent
    Security,
    /// Database administration agent
    Database,
    /// Network management agent
    Network,
    /// Support and helpdesk agent
    Support,
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentType::Devops => write!(f, "devops"),
            AgentType::Security => write!(f, "security"),
            AgentType::Database => write!(f, "database"),
            AgentType::Network => write!(f, "network"),
            AgentType::Support => write!(f, "support"),
        }
    }
}

/// Hire (deploy) an AI agent to a server
#[derive(Debug, Parser, Clone)]
pub struct HireCommand {
    /// Type of agent to deploy
    #[arg(value_enum)]
    pub agent_type: AgentType,

    /// Target server ID for deployment
    #[arg(long, short = 's')]
    pub server: String,

    /// Agent display name
    #[arg(long, short = 'n')]
    pub name: Option<String>,

    /// Agent capabilities (comma-separated)
    #[arg(long, short = 'c', value_delimiter = ',')]
    pub capabilities: Option<Vec<String>>,

    /// Skip confirmation prompt
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Force deployment without checks
    #[arg(long)]
    pub force: bool,

    /// Dry run mode (validate but don't deploy)
    #[arg(long)]
    pub dry_run: bool,

    /// Output format: table, json
    #[arg(long, default_value = "table")]
    pub format: String,
}

impl HireCommand {
    pub fn run(self) -> Result<()> {
        #[cfg(feature = "hrm")]
        {
            run_hire_with_hrm(self)
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
            println!("  The 'hire' command requires the HRM AI premium module.");
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
fn run_hire_with_hrm(cmd: HireCommand) -> Result<()> {
    use tokio::runtime::Runtime;

    let rt = Runtime::new()?;
    rt.block_on(async {
        let theme = SovereignTheme::default();

        // Print header
        println!();
        println!(
            "  {}┌─ CX Linux Agent Deployment ─────────────────────────────────────────┐{}",
            theme.primary.ansi_fg, "\x1b[0m"
        );
        println!(
            "  {}│                                                                      │{}",
            theme.primary.ansi_fg, "\x1b[0m"
        );

        println!();
        println!("  📋 Deployment Request");
        println!("  ─────────────────────────────────────");
        println!("  Agent Type:  {}", cmd.agent_type);
        println!("  Server:      {}", cmd.server);
        if let Some(ref name) = cmd.name {
            println!("  Name:        {}", name);
        }
        if let Some(ref caps) = cmd.capabilities {
            println!("  Capabilities: {}", caps.join(", "));
        }
        println!();

        // Confirm unless --yes or --dry-run
        if !cmd.yes && !cmd.dry_run {
            print!("  Deploy this agent? [y/N] ");
            std::io::Write::flush(&mut std::io::stdout())?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if !input.trim().eq_ignore_ascii_case("y") {
                println!("  ❌ Deployment cancelled");
                return Ok(());
            }
        }

        // Get database URL from environment
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/cx_agents".to_string());

        // Create database connection and repository
        let db_conn = DatabaseConnection::new(&db_url)
            .await
            .map_err(|e| anyhow::anyhow!("Database connection failed: {}", e))?;
        let repository = PostgresAgentRepository::new(db_conn);

        // Create hiring config with default capabilities
        let config = HireConfig {
            database_url: db_url,
            default_capabilities: get_default_capabilities(&cmd.agent_type),
            max_agents_per_server: 5,
        };

        // Create hiring service
        let service = AgentHiringService::new(repository, config);

        // Build HRM hire command
        let hrm_cmd = HrmHireCommand {
            server: cmd.server.clone(),
            agent_type: cmd.agent_type.to_string(),
            name: cmd.name.clone(),
            capabilities: cmd.capabilities.map(|c| c.join(",")),
            force: cmd.force,
            dry_run: cmd.dry_run,
        };

        // Deploy agent
        println!("  🚀 Deploying agent...");

        let result = service
            .hire_agent(&hrm_cmd)
            .await
            .map_err(|e| anyhow::anyhow!("Deployment failed: {}", e))?;

        println!();
        println!(
            "  {}✅ Agent deployed successfully!{}",
            theme.success.ansi_fg, "\x1b[0m"
        );
        println!();
        println!("  Agent ID:    {}", result.agent_id);
        println!("  Server:      {}", result.server_id);
        println!("  Status:      {:?}", result.status);
        println!("  Message:     {}", result.message);
        println!();

        Ok(())
    })
}

#[cfg(feature = "hrm")]
fn get_default_capabilities(agent_type: &AgentType) -> HashMap<String, Vec<String>> {
    let capabilities = match agent_type {
        AgentType::Devops => vec![
            "deploy".into(),
            "rollback".into(),
            "scale".into(),
            "monitor".into(),
        ],
        AgentType::Security => vec![
            "audit".into(),
            "scan".into(),
            "patch".into(),
            "firewall".into(),
        ],
        AgentType::Database => vec![
            "backup".into(),
            "restore".into(),
            "optimize".into(),
            "migrate".into(),
        ],
        AgentType::Network => vec![
            "configure".into(),
            "diagnose".into(),
            "loadbalance".into(),
            "dns".into(),
        ],
        AgentType::Support => vec![
            "ticket".into(),
            "escalate".into(),
            "notify".into(),
            "report".into(),
        ],
    };
    let mut map = HashMap::new();
    map.insert(agent_type.to_string(), capabilities);
    map
}
