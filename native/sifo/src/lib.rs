use rustler::ResourceArc;
use std::sync::Arc;
use sysinfo::{System, SystemExt};

pub struct SystemResource(System);

#[rustler::nif]
fn sys() -> ResourceArc<SystemResource> {
    let mut s = System::new_all();
    s.refresh_all();
    let system_resource = SystemResource(s);
    ResourceArc::new(system_resource)
}

#[rustler::nif]
fn available_memory(resource: ResourceArc<SystemResource>) -> u64 {
    resource.0.available_memory()
}

#[rustler::nif]
fn total_memory(resource: ResourceArc<SystemResource>) -> u64 {
    resource.0.total_memory()
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

rustler::init!(
    "Elixir.Sifo",
    [add, sys, available_memory, total_memory],
    load = load
);

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    rustler::resource!(SystemResource, env);
    true
}
