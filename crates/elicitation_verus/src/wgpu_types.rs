use verus_builtin_macros::verus;

verus! {

pub struct ShadowWgpuExtent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

pub fn make_wgpu_extent3d(width: u32, height: u32, depth: u32) -> (result: ShadowWgpuExtent3d)
    ensures
        result.width == width,
        result.height == height,
        result.depth_or_array_layers == depth,
{
    ShadowWgpuExtent3d { width, height, depth_or_array_layers: depth }
}

pub fn verify_wgpu_extent3d_fields() -> (result: ShadowWgpuExtent3d)
    ensures
        result.width == 1920,
        result.height == 1080,
        result.depth_or_array_layers == 1,
{
    make_wgpu_extent3d(1920u32, 1080u32, 1u32)
}

pub fn verify_wgpu_extent3d_zero() -> (result: ShadowWgpuExtent3d)
    ensures
        result.width == 0,
        result.height == 0,
        result.depth_or_array_layers == 0,
{
    make_wgpu_extent3d(0u32, 0u32, 0u32)
}

pub struct ShadowWgpuOrigin3d {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub fn make_wgpu_origin3d(x: u32, y: u32, z: u32) -> (result: ShadowWgpuOrigin3d)
    ensures
        result.x == x,
        result.y == y,
        result.z == z,
{
    ShadowWgpuOrigin3d { x, y, z }
}

pub fn verify_wgpu_origin3d_fields() -> (result: ShadowWgpuOrigin3d)
    ensures
        result.x == 10,
        result.y == 20,
        result.z == 30,
{
    make_wgpu_origin3d(10u32, 20u32, 30u32)
}

}
