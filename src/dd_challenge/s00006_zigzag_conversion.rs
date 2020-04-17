pub fn do_sth() {
    let mut matrix: Vec<_> = Vec::new();
    for n in 0..3 {
        let mut v = Vec::new();
        v.push(n);
        v.push(n + 1);
        matrix.push(v);
    }

    //let r1 = matrix.get(0).map(|x| x.clone());
    //r1.unwrap().push(32);

    for n in matrix.into_iter() {
        for nn in n.into_iter() {
            println!("{:?} is ..", nn);
        }
    }

    let mut m2 = build_vec_vec(4);

    m2.get_mut(0).unwrap().push('h');
    let r2 = m2.get(0).unwrap();

    println!("{:?}", r2);
    let s ="PAYPALISHIRING".to_string();
    let r = convert(s, 4);
    println!("converted is {:?} ", r);
}

fn build_vec_vec(num_rows: i32) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_rows {
        matrix.push(Vec::new())
    }
    matrix
}

pub fn convert(s: String, num_rows: i32) -> String {

    let mut matrix = build_vec_vec(num_rows);

    let mut row: usize = 0;
    let mut is_slash = false;

    for ss in s.chars() {
        //let mut mr = matrix.get(row).map(|x| x.clone());
        //mr.unwrap().push(ss);
        println!("put {:?} to row: {:?}", ss, row);
        matrix.get_mut(row)
            .unwrap().push(ss);
        if row >= (num_rows - 1) as usize {
            is_slash = true;
        }
        if row <= 0 {
            is_slash = false;
        }
        row = next_position(row, is_slash) % num_rows as usize;
        println!("The next position is: {:?}, is_slash is: {:?}", row, is_slash);
    }

    let result: String = matrix.iter().map(|v| v.iter().collect::<String>()).collect();

    result

}

/// 下一个位置的(row, col)
fn next_position(row: usize, is_slash: bool) -> usize {
    // 如果斜着走
   if is_slash {
       row - 1
   } else {
       row + 1
   }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_zigzag_conversion() {
        assert_eq!(
            super::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            super::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(super::convert("A".to_string(), 1), "A");
        assert_eq!(super::convert("AY".to_string(), 2), "AY");
    }
}