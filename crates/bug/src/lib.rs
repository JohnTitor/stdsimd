#![feature(repr_simd, platform_intrinsics)]
extern "platform-intrinsic" {
    fn simd_ne<T, U>(x: T, y: T) -> U;
}
#[repr(simd)]
pub struct V(pub [f32; 4]);
#[repr(simd)]
pub struct M(pub [i32; 4]);
pub fn ne(a: V, b: V) -> M {
    unsafe { simd_ne(a, b) }
}
