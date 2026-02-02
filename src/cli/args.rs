//! CLI argument definitions using Clap.

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "rustwhy",
    author = "Ninso112",
    version,
    about = "🔍 Unified Linux System Diagnostics - Understand WHY things happen",
    long_about = "RustWhy is a comprehensive system diagnostic tool that explains \
                  why your Linux system behaves a certain way. It combines 13 \
                  specialized diagnostic modules into one powerful CLI.",
    after_help = "EXAMPLES:\n    \
                  rustwhy cpu                  # Quick CPU analysis\n    \
                  rustwhy mem --detailed       # Detailed memory breakdown\n    \
                  rustwhy net --host google.com # Network diagnostics\n    \
                  rustwhy fan --watch          # Live fan monitoring\n    \
                  rustwhy all                  # Run all diagnostics\n\n\
                  For more information, visit: https://github.com/Ninso112/rustwhy"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,

    /// Verbose output with additional details
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze boot performance and slow services
    Boot {
        /// Show systemd blame output
        #[arg(long)]
        blame: bool,

        /// Show critical boot chain
        #[arg(long)]
        critical: bool,

        /// Show overall boot time breakdown
        #[arg(long)]
        time: bool,

        /// Number of slowest services to show
        #[arg(long, default_value = "10")]
        top: usize,
    },

    /// Explain high CPU usage
    Cpu {
        /// Continuous monitoring mode
        #[arg(short, long)]
        watch: bool,

        /// Number of top processes to show
        #[arg(long, default_value = "10")]
        top: usize,

        /// Update interval in seconds
        #[arg(long, default_value = "2")]
        interval: u64,

        /// Group by user
        #[arg(long)]
        by_user: bool,
    },

    /// Explain memory usage
    Mem {
        /// Show detailed memory breakdown
        #[arg(short, long)]
        detailed: bool,

        /// Show swap usage details
        #[arg(long)]
        swap: bool,

        /// Number of top processes to show
        #[arg(long, default_value = "10")]
        top: usize,

        /// Show cache breakdown
        #[arg(long)]
        cache: bool,
    },

    /// Analyze disk space usage
    Disk {
        /// Path to analyze (default: /)
        path: Option<String>,

        /// Maximum depth for directory analysis
        #[arg(long, default_value = "3")]
        depth: usize,

        /// Find files older than N days
        #[arg(long)]
        old: Option<u64>,

        /// Find files larger than SIZE (e.g., 100M, 1G)
        #[arg(long)]
        large: Option<String>,

        /// Include hidden files
        #[arg(long)]
        hidden: bool,
    },

    /// Explain high disk I/O
    Io {
        /// Continuous monitoring mode
        #[arg(short, long)]
        watch: bool,

        /// Filter by device (e.g., sda, nvme0n1)
        #[arg(long)]
        device: Option<String>,

        /// Number of top processes to show
        #[arg(long, default_value = "10")]
        top: usize,

        /// Update interval in seconds
        #[arg(long, default_value = "2")]
        interval: u64,
    },

    /// Diagnose network issues
    Net {
        /// Target host for connectivity tests
        #[arg(long, default_value = "8.8.8.8")]
        host: String,

        /// Run full diagnostic suite
        #[arg(long)]
        full: bool,

        /// Only test DNS resolution
        #[arg(long)]
        dns_only: bool,

        /// Number of ping packets
        #[arg(long, default_value = "5")]
        count: usize,

        /// Show interface statistics
        #[arg(long)]
        interfaces: bool,
    },

    /// Explain fan activity
    Fan {
        /// Continuous monitoring mode
        #[arg(short, long)]
        watch: bool,

        /// Update interval in seconds
        #[arg(long, default_value = "2")]
        interval: u64,

        /// Temperature threshold for alerts (Celsius)
        #[arg(long)]
        threshold: Option<f32>,

        /// Show all sensors
        #[arg(long)]
        all_sensors: bool,
    },

    /// Analyze system temperature
    Temp {
        /// Continuous monitoring mode
        #[arg(short, long)]
        watch: bool,

        /// Show all available sensors
        #[arg(long)]
        all_sensors: bool,

        /// Only show critical temperatures
        #[arg(long)]
        critical: bool,

        /// Update interval in seconds
        #[arg(long, default_value = "2")]
        interval: u64,
    },

    /// Analyze GPU usage
    Gpu {
        /// Continuous monitoring mode
        #[arg(short, long)]
        watch: bool,

        /// Force NVIDIA GPU detection
        #[arg(long)]
        nvidia: bool,

        /// Force AMD GPU detection
        #[arg(long)]
        amd: bool,

        /// Force Intel GPU detection
        #[arg(long)]
        intel: bool,

        /// Show GPU processes
        #[arg(long)]
        processes: bool,
    },

    /// Explain battery drain
    Batt {
        /// Continuous monitoring mode
        #[arg(short, long)]
        watch: bool,

        /// Show detailed power breakdown
        #[arg(short, long)]
        detailed: bool,

        /// Show process wakeups
        #[arg(long)]
        wakeups: bool,

        /// Show power history
        #[arg(long)]
        history: bool,
    },

    /// Diagnose sleep/suspend issues
    Sleep {
        /// Show systemd inhibitors
        #[arg(long)]
        inhibitors: bool,

        /// Show wake sources
        #[arg(long)]
        wake_sources: bool,

        /// Show last N sleep events
        #[arg(long)]
        history: Option<usize>,

        /// Test suspend capability
        #[arg(long)]
        test: bool,
    },

    /// Diagnose USB device issues
    Usb {
        /// Filter by device ID (vendor:product)
        #[arg(long)]
        device: Option<String>,

        /// Show USB device tree
        #[arg(long)]
        tree: bool,

        /// Show recent dmesg for USB
        #[arg(long)]
        dmesg: bool,

        /// Show power information
        #[arg(long)]
        power: bool,
    },

    /// Diagnose mount point issues
    Mount {
        /// Specific mountpoint to analyze
        mountpoint: Option<String>,

        /// Analyze all mount points
        #[arg(long)]
        all: bool,

        /// Check NFS mounts
        #[arg(long)]
        nfs: bool,

        /// Run filesystem checks
        #[arg(long)]
        check: bool,

        /// Show mount options
        #[arg(long)]
        options: bool,
    },

    /// Run all diagnostic modules
    All {
        /// Skip slow checks
        #[arg(long)]
        quick: bool,

        /// Output format
        #[arg(long, value_enum, default_value = "terminal")]
        format: OutputFormat,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Terminal,
    Json,
    Html,
}

#[derive(Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}
