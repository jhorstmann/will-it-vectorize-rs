const DAYS_PER_CYCLE: i32 = 146097;
const DAYS_0000_TO_1970: i32 = (DAYS_PER_CYCLE * 5) - (30 * 365 + 7);

const SUPPORT_NEGATIVE_YEAR: bool = true;

#[inline]
fn is_leap_year(year: i32) -> bool {
    ((year % 4) == 0) & ((year % 100) != 0) | ((year % 400) == 0)
}

fn ymd_to_epoch_days(year: i32, month: i32, day: i32) -> i32 {
    let y = year;
    let m = month;
    let mut total = 365 * y;

    total += if y < 0 && SUPPORT_NEGATIVE_YEAR {
        -(y / -4 - y / -100 + y / -400)
    } else {
        (y + 3) / 4 - (y + 99) / 100 + (y + 399) / 400
    };

    total += (367 * m - 362) / 12;
    total += day - 1;
    total -= if m > 2 {
        if !is_leap_year(year) {
            2
        } else {
            1
        }
    } else {
        0
    };

    total - DAYS_0000_TO_1970
}

pub fn to_epoch_days(ymd: &[(i32, i32, i32); 8], epoch_days: &mut [i32; 8]) {
    // assert!(ymd.len() == epoch_days.len());

    epoch_days
        .iter_mut()
        .zip(ymd.iter())
        .for_each(|(days, (y, m, d))| {
            *days = ymd_to_epoch_days(*y, *m, *d);
        });
}
