pub fn rgba_to_b_loop(rgba: &[[u8; 4]; 16], output: &mut [u8; 16]) {
    rgba.iter()
        .zip(output.iter_mut())
        .for_each(|([_r, _g, b, _a], out)| {
            *out = *b;
        })
}

pub fn rgba_to_b_local(rgba: &[[u8; 4]; 16], output: &mut [u8; 16]) {
    let mut tmp = [0_u8; 16];
    rgba.iter()
        .zip(tmp.iter_mut())
        .for_each(|([_r, _g, b, _a], out)| {
            *out = *b;
        });
    *output = tmp;
}

/// # Safety
/// Requires avx512 support
#[target_feature(enable = "avx512f,avx512vl")]
pub unsafe fn rgba_to_b_intrinsic(rgba: &[[u8; 4]; 16], output: &mut [u8; 16]) {
    use std::arch::x86_64::*;

    let rgba_x16 = _mm512_loadu_si512(rgba.as_ptr().cast());
    let ba00_x16 = _mm512_srli_epi32::<16>(rgba_x16);
    let b_x16 = _mm512_cvtepi32_epi8(ba00_x16);
    _mm_storeu_si128(output.as_mut_ptr().cast(), b_x16);
}
