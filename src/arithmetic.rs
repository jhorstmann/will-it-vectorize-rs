pub fn arithmetic_add(a: &[f32; 32], b: &[f32; 32], c: &mut [f32; 32]) {
    c.iter_mut()
        .zip(a.iter().zip(b.iter()))
        .for_each(|(c, (a, b))| {
            *c = *a + *b;
        });
}

pub fn arithmetic_sum_iter(a: &[f32; 32]) -> f32 {
    a.iter().copied().sum()
}

pub fn arithmetic_sum_fast(a: &[f32; 32]) -> f32 {
    a.iter()
        .copied()
        .fold(0.0, |a, x| unsafe { std::intrinsics::fadd_fast(a, x) })
}
