pub const DEF_WORLD_WIDTH: u32 = 1496;
pub const DEF_WORLD_HEIGHT: u32 = 1024;

#[derive(Copy, Clone, Debug)]
pub struct ViewPort(pub u32, pub u32);

impl Default for ViewPort {
    fn default() -> Self {
        ViewPort(DEF_WORLD_WIDTH, DEF_WORLD_HEIGHT)
    }
}
