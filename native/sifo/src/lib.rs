use rustler::ResourceArc;
use std::sync::Mutex;
use sysinfo::{System, SystemExt};

pub struct SystemResource(Mutex<System>);

#[rustler::nif]
fn sys() -> ResourceArc<SystemResource> {
    ResourceArc::new(SystemResource(Mutex::new(System::new_all())))
}

#[rustler::nif]
fn refresh_all(resource: ResourceArc<SystemResource>) -> ResourceArc<SystemResource> {
    {
        let mut s = resource.0.lock().unwrap();
        s.refresh_all();
    }
    resource
}

#[rustler::nif]
fn available_memory(resource: ResourceArc<SystemResource>) -> u64 {
    resource.0.lock().unwrap().available_memory()
}

#[rustler::nif]
fn total_memory(resource: ResourceArc<SystemResource>) -> u64 {
    resource.0.lock().unwrap().total_memory()
}

rustler::init!(
    "Elixir.Sifo",
    [sys, refresh_all, available_memory, total_memory],
    load = load
);

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    rustler::resource!(SystemResource, env);
    true
}
