use verus_builtin_macros::verus;

verus! {

pub struct ShadowWinitPhysicalSize {
    pub width: u32,
    pub height: u32,
}

pub fn make_winit_physical_size(width: u32, height: u32) -> (result: ShadowWinitPhysicalSize)
    ensures
        result.width == width,
        result.height == height,
{
    ShadowWinitPhysicalSize { width, height }
}

pub fn verify_winit_physical_size_fields() -> (result: ShadowWinitPhysicalSize)
    ensures
        result.width == 1920,
        result.height == 1080,
{
    make_winit_physical_size(1920u32, 1080u32)
}

pub fn verify_winit_physical_size_zero() -> (result: ShadowWinitPhysicalSize)
    ensures
        result.width == 0,
        result.height == 0,
{
    make_winit_physical_size(0u32, 0u32)
}

pub struct ShadowWinitLogicalSize {
    pub width: u32,
    pub height: u32,
}

pub fn make_winit_logical_size(width: u32, height: u32) -> (result: ShadowWinitLogicalSize)
    ensures
        result.width == width,
        result.height == height,
{
    ShadowWinitLogicalSize { width, height }
}

pub fn verify_winit_logical_size_fields() -> (result: ShadowWinitLogicalSize)
    ensures
        result.width == 1280,
        result.height == 720,
{
    make_winit_logical_size(1280u32, 720u32)
}

pub struct ShadowWinitLogicalPosition {
    pub x: u32,
    pub y: u32,
}

pub fn make_winit_logical_position(x: u32, y: u32) -> (result: ShadowWinitLogicalPosition)
    ensures
        result.x == x,
        result.y == y,
{
    ShadowWinitLogicalPosition { x, y }
}

pub fn verify_winit_logical_position_fields() -> (result: ShadowWinitLogicalPosition)
    ensures
        result.x == 100,
        result.y == 200,
{
    make_winit_logical_position(100u32, 200u32)
}

}
