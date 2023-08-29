use rustler::ResourceArc;
use std::sync::RwLock;
use sysinfo::{System, SystemExt};

mod ex_component;
mod ex_cpu;
pub struct SystemResource(RwLock<System>);

#[rustler::nif]
fn sys() -> ResourceArc<SystemResource> {
    ResourceArc::new(SystemResource(RwLock::new(System::new_all())))
}

macro_rules! refresh_fn {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[rustler::nif]
        fn $name(resource: ResourceArc<SystemResource>) -> ResourceArc<SystemResource> {
            {
                let mut s = resource.0.write().unwrap();
                s.$name();
            }
            resource
        }
    };
}

macro_rules! scalar_fn {
    ($name:ident, $return_type:ty, $doc:literal) => {
        #[doc = $doc]
        #[rustler::nif]
        fn $name(resource: ResourceArc<SystemResource>) -> $return_type {
            resource.0.read().unwrap().$name()
        }
    };
}

refresh_fn!(
    refresh_all,
    "Refreshes all system, processes, disks and network interfaces information."
);
refresh_fn!(
    refresh_system,
    "Refreshes system information (RAM, swap, CPU usage and components' temperature)."
);
refresh_fn!(refresh_memory, "Refreshes RAM and SWAP usage.");
refresh_fn!(refresh_cpu, "Refreshes CPUs information.");
refresh_fn!(refresh_components, "Refreshes components' temperature.");
refresh_fn!(refresh_components_list, "Refreshes components list.");
refresh_fn!(
    refresh_processes,
    "Gets all processes and updates their information."
);
refresh_fn!(refresh_disks, "Refreshes the listed disks' information.");
refresh_fn!(
    refresh_disks_list,
    "The disk list will be emptied then completely recomputed."
);
refresh_fn!(refresh_users_list, "Refreshes users list.");
refresh_fn!(refresh_networks, "Refreshes networks data.");
refresh_fn!(refresh_networks_list, "he network list will be updated: removing not existing anymore interfaces and adding new ones.");

scalar_fn!(
    physical_core_count,
    Option<usize>,
    "Returns the number of physical cores on the CPU or `None` if it couldn't get it."
);

scalar_fn!(total_memory, u64, "Returns the RAM size in bytes.");
scalar_fn!(free_memory, u64, "Returns the amount of free RAM in bytes.");
scalar_fn!(
    available_memory,
    u64,
    "Returns the amount of available RAM in bytes."
);
scalar_fn!(used_memory, u64, "Returns the amount of used RAM in bytes.");
scalar_fn!(total_swap, u64, "Returns the SWAP size in bytes.");
scalar_fn!(free_swap, u64, "Returns the amount of free SWAP in bytes.");
scalar_fn!(used_swap, u64, "Returns the amount of used SWAP in bytes.");

rustler::init!(
    "Elixir.Sifo",
    [
        sys,
        refresh_all,
        refresh_system,
        refresh_memory,
        refresh_cpu,
        refresh_components,
        refresh_components_list,
        refresh_processes,
        refresh_disks,
        refresh_disks_list,
        refresh_users_list,
        refresh_networks,
        refresh_networks_list,
        ex_cpu::cpus,
        physical_core_count,
        total_memory,
        free_memory,
        available_memory,
        used_memory,
        total_swap,
        free_swap,
        used_swap,
        ex_component::components
    ],
    load = load
);

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    rustler::resource!(SystemResource, env);
    true
}
