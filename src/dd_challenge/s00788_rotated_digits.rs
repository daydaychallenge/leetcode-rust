pub fn do_sth() {
    println!("Rotated Digits!");

    let nums = &[1, 2, 5, 9];
    let can_rotate_1 = can_rotate_all(nums.to_vec());
    println!("{:?} can rotate: {:?}", nums, can_rotate_1);

    let not_rotate_1 = not_rotate_all(nums.to_vec());
    println!("{:?} not rotate: {:?}", nums, not_rotate_1);

    let is_rotate_1 = is_rotate_all(nums.to_vec());
    println!("{:?} is rotate: {:?}", nums, is_rotate_1);

    let num: usize = 2551;
    let is_rotate_2 = is_rotated_digit(num);
    println!("{:?} is rotate: {:?}", num, is_rotate_2);

    let count_ten = count_rotate_digits(10);
    println!("10 has rotate digits: {:?}", count_ten);

    assert_eq!(count_rotate_digits(10), 4);

    let count_thousand = count_rotate_digits(1000);
    println!("1000 has rotate digits: {:?}", count_thousand);

    let count_twenty = count_rotate_digits(20);
    println!("20 has rotate digits: {:?}", count_twenty);

    assert_eq!(count_rotate_digits(20), 8);
}

const CAN_ROTATE: &[usize] = &[0, 1, 2, 5, 6, 8, 9];
const NOT_ROTATE: &[usize] = &[0, 1, 8];

/// step 1: 所数字分解为只有个位数的数组
/// step 2: 数组中所有数字是否能反转,是下一步，否返回false
/// step 3: 数组中所有数字是否全是能反转的非好数字,是，返回false,否返回true
///
fn is_rotated_digit(num: usize) -> bool {

    let nums = num_to_vec(num);

    is_rotate_all(nums)

}

fn num_to_vec(num: usize) -> Vec<usize> {
    num.to_string().chars().map(|x| x.to_digit(10).unwrap() as usize).collect()
}

fn can_rotate_all(nums: Vec<usize>) -> bool {
    nums.into_iter().all(|n| CAN_ROTATE.contains(&n))
}

fn not_rotate_all(nums: Vec<usize>) -> bool {
    nums.into_iter().all(|n| NOT_ROTATE.contains(&n))
}

fn is_rotate_all(nums: Vec<usize>) -> bool {
    if can_rotate_all(nums.clone()) && !not_rotate_all(nums) {
        true
    } else {
        false
    }
}

pub fn count_rotate_digits(end: usize) -> i32 {
    let mut count = 0;

    for n in 1..end {
        if is_rotated_digit(n) {
            count = count + 1;
        }
    }

    count
}