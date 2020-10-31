#![cfg_attr(target_arch = "spirv", no_std)]
#![feature(lang_items)]
#![feature(register_attr)]
#![register_attr(spirv)]

use core::f32::consts::PI;
use glam::{const_vec2, const_vec3, Mat4, Vec2, Vec3, Vec4};
use spirv_std::{Input, MathExt, Output};


#[allow(unused_attributes)]
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: Input<i32>,
    in_pos: Input<Vec2>,
    #[spirv(position)] mut out_pos: Output<Vec4>,
    mut out_color: Output<Vec2>
) {
    let vert_id = vert_id.load();
    let in_pos = in_pos.load();
    out_pos.store(Vec4::new(in_pos.x(), in_pos.y(), 0.0, 1.0));
    out_color.store(in_pos);
}

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main_fs(in_color: Input<Vec2>, mut output: Output<Vec4>) {
    let in_color = in_color.load();
    output.store(Vec4::new(in_color.x(), in_color.y(), 0.0, 1.0))
}

#[cfg(all(not(test), target_arch = "spirv"))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(test), target_arch = "spirv"))]
#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}
