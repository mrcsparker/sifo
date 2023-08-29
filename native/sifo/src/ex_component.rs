use rayon::prelude::*;
use rustler::{NifStruct, ResourceArc};
use sysinfo::{ComponentExt, SystemExt};

use crate::SystemResource;

#[derive(Debug, NifStruct)]
#[module = "Component"]
pub struct ExComponent {
    pub temperature: f32,
    pub max: f32,
    pub critical: Option<f32>,
    pub label: String,
}

impl From<&sysinfo::Component> for ExComponent {
    fn from(component: &sysinfo::Component) -> Self {
        Self {
            temperature: component.temperature(),
            max: component.max(),
            critical: component.critical(),
            label: component.label().to_string(),
        }
    }
}

/// Returns the components list.
#[rustler::nif]
pub fn components(resource: ResourceArc<SystemResource>) -> Vec<ExComponent> {
    resource
        .0
        .read()
        .unwrap()
        .components()
        .par_iter()
        .map(Into::into)
        .collect::<Vec<ExComponent>>()
}
