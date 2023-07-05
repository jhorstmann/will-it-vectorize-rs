pub fn arithmetic_add(a: &[f32; 32], b: &[f32; 32], c: &mut [f32; 32]) {
    c.iter_mut()
        .zip(a.iter().zip(b.iter()))
        .for_each(|(c, (a, b))| {
            *c = *a + *b;
        });
}

pub fn arithmetic_sum_u32(a: &[u32; 32]) -> u32 {
    a.iter().copied().sum()
}

pub fn arithmetic_sum_f32(a: &[f32; 32]) -> f32 {
    a.iter().copied().sum()
}

/// "fun, safe math"
pub fn arithmetic_sum_unsafe(a: &[f32; 32]) -> f32 {
    a.iter()
        .copied()
        .fold(0.0, |a, x| unsafe { std::intrinsics::fadd_fast(a, x) })
}

#[cfg(test)]
mod tests {
    use crate::arithmetic::{arithmetic_sum_f32, arithmetic_sum_unsafe};
    use std::hint::black_box;

    #[test]
    fn test_sum_f32() {
        assert_eq!(f32::MAX + f32::MIN + 0.25, 0.25);
        assert_eq!(f32::MAX + 0.25 + f32::MIN, 0.0);

        let input = black_box({
            let mut tmp = [0_f32; 32];
            tmp[0] = f32::MAX;
            tmp[1] = 0.25;
            tmp[8] = f32::MIN;
            tmp
        });

        assert_eq!(arithmetic_sum_f32(&input), 0.0);
        assert_eq!(
            arithmetic_sum_unsafe(&input),
            if !cfg!(debug_assertions) { 0.25 } else { 0.0 }
        );
    }
}
