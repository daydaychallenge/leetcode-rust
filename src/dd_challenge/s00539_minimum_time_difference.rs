use crate::dd_challenge::Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut mins = time_points.iter()
            .map(|ms|
                     ms
                         .split(':')
                         .zip(vec![60, 1].iter())
                         .fold(0, |a, c| a + (c.0.parse::<i32>().unwrap() * c.1))
            )
            .collect::<Vec<_>>();
        mins.sort();
        mins.push(mins[0] + 1440);
        mins.iter()
            .take(mins.len() - 1)
            .zip(mins.iter().skip(1))
            .map(|prev_sur| prev_sur.1 - prev_sur.0)
            .fold(i32::max_value(), |a, c| if c < a { c } else { a })

    }
}

pub fn do_sth() {
    let m = vec![60, 1];
    let ms = "20:31".to_string();
    let n: Vec<_> = ms.split(":").collect::<Vec<_>>();
    let mn =  n.iter().zip(m.iter());
    let mm = mn.clone().fold(0, |a, c| a + (c.0.parse::<i32>().unwrap() * c.1));
    println!("{:?}", mm);
    for z in mn {
        println!("{:?}", z);
    }
    
}

#[test]
fn test_find_min_difference() {
   assert_eq!(Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]), 1);
}