use legion::*;

use super::systems::*;

pub fn init_world(mut resources1: Resources) -> Result<(World, Resources), String> {
    let mut world = World::default();
    let mut resources2 = init_resources(resources1)?;

    Ok((world, resources2))
}
