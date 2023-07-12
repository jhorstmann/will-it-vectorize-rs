pub fn check_range_all(keys: &[u32], max: u32) -> bool {
    keys.iter().all(|x| *x < max)
}

pub fn check_range_copied_all(keys: &[u32], max: u32) -> bool {
    keys.iter().copied().all(|x| x < max)
}

#[allow(clippy::unnecessary_fold)]
pub fn check_range_fold(keys: &[u32], max: u32) -> bool {
    keys.iter().fold(true, |a, x| a && *x < max)
}
#[allow(clippy::unnecessary_fold)]
pub fn check_range_copied_fold(keys: &[u32], max: u32) -> bool {
    keys.iter().copied().fold(true, |a, x| a && x < max)
}
