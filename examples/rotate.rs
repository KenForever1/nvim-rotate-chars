use rayon::prelude::*;

/// 假设 rotate_char 是一个高效的旋转字符函数
#[inline]
fn rotate_char(c: u8, direction: bool, steps: u8) -> u8 {
    // 例如，一个简单的 Caesar cipher 实现（假设输入是 ASCII 字母）
    if (b'a'..=b'z').contains(&c) {
        let base = b'a';
        ((c - base + if direction { steps } else { 26 - steps }) % 26) + base
    } else if (b'A'..=b'Z').contains(&c) {
        let base = b'A';
        ((c - base + if direction { steps } else { 26 - steps }) % 26) + base
    } else {
        c
    }
}

/// 高效旋转内容的函数
fn rotate_content(content: &[String], direction: bool, steps: u8) -> Vec<String> {
    content
        .par_iter() // 使用 Rayon 进行并行处理
        .map(|line| {
            let bytes = line.as_bytes();
            let mut rotated = String::with_capacity(bytes.len());
            for &c in bytes {
                rotated.push(rotate_char(c, direction, steps) as char);
            }
            rotated
        })
        .collect()
}

fn main() {
    let content = vec![
        "Hello, World!".to_string(),
        "Rust is awesome.".to_string(),
        "Rotate this content.".to_string(),
    ];

    let direction = true; // true 表示向前旋转，false 表示向后旋转
    let steps = 3;

    let rotated_content = rotate_content(&content, direction, steps);

    for line in rotated_content {
        println!("{}", line);
    }
}
