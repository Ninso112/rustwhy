//! RustWhy - Unified Linux System Diagnostics
//!
//! Entry point: parse CLI, run selected module(s), output report.

use clap::CommandFactory;
use clap::Parser;
use rustwhy::cli::{Cli, Commands, Shell};
use rustwhy::core::{ModuleConfig, run_module};
use rustwhy::modules::{all_modules, get_module};
use rustwhy::output::{write_report_json, write_report_terminal};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            match shell {
                Shell::Bash => rustwhy::cli::print_completion(clap_complete::Shell::Bash, &mut cmd),
                Shell::Zsh => rustwhy::cli::print_completion(clap_complete::Shell::Zsh, &mut cmd),
                Shell::Fish => rustwhy::cli::print_completion(clap_complete::Shell::Fish, &mut cmd),
                Shell::PowerShell => {
                    rustwhy::cli::print_completion(clap_complete::Shell::PowerShell, &mut cmd)
                }
            }
            return Ok(());
        }
        Commands::All { quick: _, format: _ } => {
            return run_all_and_output(&cli);
        }
        _ => {}
    }

    let (module_name, config) = command_to_module_config(&cli)?;
    let module = get_module(&module_name).ok_or_else(|| anyhow::anyhow!("Unknown module: {}", module_name))?;
    let rt = tokio::runtime::Runtime::new()?;
    let report = rt.block_on(run_module(module, &config))?;

    let mut stdout = io::stdout().lock();
    if cli.json {
        write_report_json(&mut stdout, &report)?;
    } else {
        write_report_terminal(&mut stdout, &report, !cli.no_color);
    }
    stdout.flush()?;
    Ok(())
}

fn command_to_module_config(cli: &Cli) -> anyhow::Result<(String, ModuleConfig)> {
    let mut extra = HashMap::new();
    let config = ModuleConfig {
        verbose: cli.verbose,
        watch: false,
        interval: 2,
        top_n: 10,
        json_output: cli.json,
        extra_args: extra.clone(),
    };

    let (name, config) = match &cli.command {
        Commands::Boot { top, .. } => ("boot".into(), ModuleConfig { top_n: *top, ..config }),
        Commands::Cpu { watch, top, interval, .. } => (
            "cpu".into(),
            ModuleConfig { watch: *watch, top_n: *top, interval: *interval, ..config },
        ),
        Commands::Mem { top, .. } => ("mem".into(), ModuleConfig { top_n: *top, ..config }),
        Commands::Disk { path, depth, old, large, hidden, .. } => {
            extra.insert("path".into(), path.clone().unwrap_or_else(|| "/".into()));
            extra.insert("depth".into(), depth.to_string());
            if let Some(o) = old {
                extra.insert("old".into(), o.to_string());
            }
            if let Some(ref l) = large {
                extra.insert("large".into(), l.clone());
            }
            extra.insert("hidden".into(), hidden.to_string());
            ("disk".into(), ModuleConfig { extra_args: extra, ..config })
        }
        Commands::Io { watch, top, interval, device, .. } => {
            if let Some(ref d) = device {
                extra.insert("device".into(), d.clone());
            }
            (
                "io".into(),
                ModuleConfig { watch: *watch, top_n: *top, interval: *interval, extra_args: extra, ..config },
            )
        },
        Commands::Net { host, .. } => {
            extra.insert("host".into(), host.clone());
            ("net".into(), ModuleConfig { extra_args: extra, ..config })
        }
        Commands::Fan { watch, interval, threshold, .. } => {
            if let Some(t) = threshold {
                extra.insert("threshold".into(), t.to_string());
            }
            (
                "fan".into(),
                ModuleConfig { watch: *watch, interval: *interval, extra_args: extra, ..config },
            )
        }
        Commands::Temp { watch, interval, critical, .. } => {
            extra.insert("critical".into(), critical.to_string());
            (
                "temp".into(),
                ModuleConfig { watch: *watch, interval: *interval, extra_args: extra, ..config },
            )
        }
        Commands::Gpu { .. } => ("gpu".into(), config),
        Commands::Batt { detailed, .. } => {
            extra.insert("detailed".into(), detailed.to_string());
            ("batt".into(), ModuleConfig { extra_args: extra, ..config })
        }
        Commands::Sleep { inhibitors, .. } => {
            extra.insert("inhibitors".into(), inhibitors.to_string());
            ("sleep".into(), ModuleConfig { extra_args: extra, ..config })
        }
        Commands::Usb { device, dmesg, .. } => {
            if let Some(ref d) = device {
                extra.insert("device".into(), d.clone());
            }
            extra.insert("dmesg".into(), dmesg.to_string());
            ("usb".into(), ModuleConfig { extra_args: extra, ..config })
        }
        Commands::Mount { mountpoint, nfs, options, .. } => {
            if let Some(ref m) = mountpoint {
                extra.insert("mountpoint".into(), m.clone());
            }
            extra.insert("nfs".into(), nfs.to_string());
            extra.insert("options".into(), options.to_string());
            ("mount".into(), ModuleConfig { extra_args: extra, ..config })
        }
        Commands::All { .. } | Commands::Completions { .. } => {
            anyhow::bail!("Unreachable")
        }
    };
    Ok((name, config))
}

fn run_all_and_output(cli: &Cli) -> anyhow::Result<()> {
    let config = ModuleConfig {
        verbose: cli.verbose,
        watch: false,
        interval: 2,
        top_n: 10,
        json_output: cli.json,
        extra_args: HashMap::new(),
    };
    let modules = all_modules();
    let rt = tokio::runtime::Runtime::new()?;
    let mut stdout = io::stdout().lock();

    if cli.json {
        let mut reports = Vec::new();
        for module in &modules {
            match rt.block_on(run_module(module.clone(), &config)) {
                Ok(r) => reports.push(r),
                Err(e) => {
                    eprintln!("Module {} failed: {}", module.name(), e);
                }
            }
        }
        let json = serde_json::to_string_pretty(&reports)?;
        writeln!(stdout, "{}", json)?;
    } else {
        for module in &modules {
            match rt.block_on(run_module(module.clone(), &config)) {
                Ok(report) => write_report_terminal(&mut stdout, &report, !cli.no_color),
                Err(e) => {
                    eprintln!("Module {} failed: {}", module.name(), e);
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
