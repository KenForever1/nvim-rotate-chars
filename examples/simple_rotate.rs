/// 轮转单个字符的函数
fn rotate_char(c: char, direction: usize, steps: usize) -> char {
    match c {
        'a'..='z' => rotate_within_range(c, 'a', 'z', direction, steps),
        'A'..='Z' => rotate_within_range(c, 'A', 'Z', direction, steps),
        '0'..='9' => rotate_within_range(c, '0', '9', direction, steps),
        _ => c, // 非目标字符不变
    }
}

/// 在给定范围内轮转字符
fn rotate_within_range(c: char, start: char, end: char, direction: usize, steps: usize) -> char {
    let range = (start as u8)..=(end as u8);
    let size = range.end() - range.start() + 1;
    // println!("size is : {}", size);
    let steps = steps % size as usize;
    let current = c as usize - start as usize;

    // left
    let new_pos = if direction == 0 {
        (current + size as usize - steps) % size as usize
    } else {
        // 默认向右
        (current + steps) % size as usize
    };

    (start as usize + new_pos) as u8 as char
}

fn main() {
    let direction = 0;
    let steps = 2;
    let content = vec!["abcdef", "ghijkl", "mnopqr"];

    // 处理每一行
    let rotated_content: Vec<String> = content
        .iter()
        .enumerate()
        .map(|(i, line)| {
            println!("line is : {}", line);
            let rotated_middle = line
                .to_string()
                .chars()
                .map(|c| rotate_char(c, direction, steps))
                .collect::<String>();

            // 重新组合
            println!("rotated_middle is : {}", rotated_middle);
            rotated_middle
        })
        .collect();

    for line in rotated_content {
        println!("{}", line);
    }
}
