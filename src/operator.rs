use rayon::prelude::*;

/// 假设 rotate_char 是一个高效的旋转字符函数
#[inline]
fn rotate_char(c: u8, direction: bool, steps: u8) -> u8 {
    let steps = steps % 26;
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
pub fn rotate_content(content: &[String], direction: bool, steps: u8) -> Vec<String> {
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
