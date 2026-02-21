use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum, Debug)]
pub enum Device {
    Terminal,
}

#[derive(Parser, Debug)]
#[command(name = "intel-4004")]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub rom: Option<PathBuf>,

    #[arg(short, long, default_value = "35")]
    pub steps: usize,

    #[arg(short = 'd', long = "device", value_name = "DEVICE")]
    pub devices: Vec<Device>,
}

pub fn wants_terminal(devices: &[Device]) -> bool {
    devices.iter().any(|d| matches!(d, Device::Terminal))
}
