use std::char;

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

#[inline]
fn rotate_unicode_char(c: char, direction: bool, steps: u8, is_jump: bool) -> char {

    if !c.is_ascii() && is_jump {
        return c;
    }

    if c == ' ' || c == '\t' || c == '\n' {
        return c;
    }

    let base = c as u32;
    let max_unicode = 0x10FFFF;
    let steps = steps as u32 % (max_unicode + 1);

    let rotated = if direction {
        // rotate right
        base.wrapping_add(steps) % (max_unicode + 1)
    } else {
        // rotate left
        base.wrapping_sub(steps) % (max_unicode + 1)
    };

    match char::from_u32(rotated) {
        Some(rotated_char) => rotated_char,
        None => c, // In case the char conversion fails, return original, though it shouldn't.
    }
}

pub fn rotate_unicode_content(content: &[String], direction: bool, steps: u8, is_jump: bool) -> Vec<String> {
    content
        .par_iter() // Use Rayon for parallel processing
        .map(|line| {
            let mut rotated = String::with_capacity(line.chars().count());

            for c in line.chars() {
                rotated.push(rotate_unicode_char(c, direction, steps, is_jump));
            }

            rotated
        })
        .collect()
}
