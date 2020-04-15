use std::collections::HashMap;

const I: i32 = 1;
const V: i32 = 5;
const X: i32 = 10;
const L: i32 = 50;
const C: i32 = 100;
const D: i32 = 500;
const M: i32 = 1000;

/// step 0: 从左向右遍历，当前累计为0,前导字符对就的值prev为None
/// step 1: 记下当前字符对应 的值curr
/// step 2: 如果curr > prev，curr - prev的结果累加到累计值,并设置prev为None，否则prev加到累计值，并把curr赋值给prev
/// step 3: 遍历完成后,prev的值加到累计值
/// step 4：最后返回累计值
pub fn roman_to_int(s: String) -> i32 {
    let mut roman_int: HashMap<char, i32> = HashMap::new();
    roman_int.insert('I', I);
    roman_int.insert('V', V);
    roman_int.insert('X', X);
    roman_int.insert('L', L);
    roman_int.insert('C', C);
    roman_int.insert('D', D);
    roman_int.insert('M', M);

    let mut sum = 0;
    let mut curr: Option<i32> = None;
    let mut prev: Option<i32> = None;
    for ss in s.chars() {
        curr = roman_int.get(&ss).map(|x| *x);
        if sum == 0 && prev.is_none() {
            prev = curr;
        }
        else if !prev.is_none() && curr > prev {
            sum += (curr.unwrap() - prev.unwrap());
            prev = None;
        } else {
            if let Some(p) = prev {
                sum += p;
            }
            prev = curr
        }

    }
    if let Some(p) = prev {
        sum += p;
    }
    print!("The roman {:?} is {:?} .", s, sum);
    sum

}