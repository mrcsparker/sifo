use rayon::prelude::*;
use rustler::{NifStruct, ResourceArc};
use sysinfo::{CpuExt, SystemExt};

use crate::SystemResource;

#[derive(Debug, NifStruct)]
#[module = "Cpu"]
pub struct ExCpu {
    pub cpu_usage: f32,
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
}

impl From<&sysinfo::Cpu> for ExCpu {
    fn from(cpu: &sysinfo::Cpu) -> Self {
        Self {
            cpu_usage: cpu.cpu_usage(),
            name: cpu.name().to_string(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        }
    }
}

#[rustler::nif]
fn cpus(resource: ResourceArc<SystemResource>) -> Vec<ExCpu> {
    resource
        .0
        .read()
        .unwrap()
        .cpus()
        .par_iter()
        .map(Into::into)
        .collect::<Vec<ExCpu>>()
}
