pub fn is_leap_year(year: u64) -> bool {
    if year % 4 != 0 {
        return false;
    } else if year % 100 != 0 {
        return true;
    } else {
        return year % 400 == 0;
    }
}
