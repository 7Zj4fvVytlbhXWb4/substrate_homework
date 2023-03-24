fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for num in numbers {
        match sum.checked_add(*num) {
            Some(value) => sum = value,
            None => return None, // 溢出，返回 None
        }
    }
    Some(sum) // 返回求和结果
}
