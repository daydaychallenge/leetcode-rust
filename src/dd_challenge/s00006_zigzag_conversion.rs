pub fn do_sth() {
    let mut matrix: Vec<_> = Vec::new();
    for n in 0..3 {
        let mut v = Vec::new();
        v.push(n);
        matrix.push(v);
    }

    let r1 = matrix.get(0).map(|x| x.clone());
    r1.unwrap().push(32);

    for n in matrix.into_iter() {
        for nn in n.into_iter() {
            println!("{:?} is ..", nn);
        }
    }
}
pub fn convert(s: String, num_rows: i32) -> String {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for _ in 1..num_rows {
        matrix.push(Vec::new())
    }

    let mut row: usize = 0;
    let mut is_slash = false;

    for ss in s.chars() {
        let mut mr = matrix.get(row).map(|x| x.clone());
        mr.unwrap().push(ss);
        //matrix.get(row).unwrap().push(ss);
        if row >= num_rows as usize {
            is_slash = true;
        }
        if row <= 0 {
            is_slash = false;
        }
        row = next_position(row, is_slash);
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