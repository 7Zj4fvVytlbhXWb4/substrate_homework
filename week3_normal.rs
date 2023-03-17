fn main() {
    let v = sort_array(vec![5,1,1,2,0,0]);
    println!("{:?}  ", v);
}

fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();
    let length = nums.len();
    let mut a = 0;
    while a < length {
        let mut b = 0;
        while b < (length - 1 - a) {
            if nums[b] < nums[b+1] {
                let temp = nums[b+1];
                nums[b+1] = nums[b];
                nums[b] = temp;
            }
            b += 1;
        }
        a += 1;
        v.push(nums[length - a]);
    }
    v
}
