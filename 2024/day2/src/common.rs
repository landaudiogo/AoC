pub fn is_diff_abs_valid(val: i64) -> bool {
    val.abs() >= 1 && val.abs() <= 3
}

pub fn validate_diff_pair(left: i64, right: i64, trend: &mut Option<bool>) -> bool {
    let (sign_left, sign_right) = (left > 0, right > 0);
    if let Some(trend) = trend {
        return (sign_left == sign_right)
            && (sign_left == *trend)
            && is_diff_abs_valid(left)
            && is_diff_abs_valid(right);
    }

    if (sign_left == sign_right) && is_diff_abs_valid(left) && is_diff_abs_valid(right) {
        *trend = Some(sign_left);
        return true;
    }
    false
}
