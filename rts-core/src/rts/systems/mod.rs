use legion::*;
use legion::systems::Builder;
use super::components::viewport::*;
use super::components::time::*;

pub fn init_systems(mut builder: Builder) -> Result<Builder, String> {
    builder
    // here go core systems
        .flush();
    Ok(builder)
}

pub fn init_resources(mut resources: Resources) -> Result<Resources, String> {
    resources.insert(DeltaTime::default());
    resources.insert(ViewPort::default());
    Ok(resources)
}
