pub mod operator;

use nvim_oxi as oxi;
use oxi::{api, Dictionary, Function, Object};
use std::usize;

use operator::rotate_unicode_content;

fn get_visual_selection_content(buffer: &api::Buffer) -> oxi::Result<(usize, usize)> {
    let start_mark = buffer.get_mark('<')?;
    let end_mark = buffer.get_mark('>')?;
    let start_row = start_mark.0;
    // let start_col = start_mark.1;
    let end_row = end_mark.0;
    // let end_col = end_mark.1;

    // println!(
    //     "start_row : {}, start_col : {}, end_row : {}, end_col : {}",
    //     start_row, start_col, end_row, end_col
    // );
    Ok((start_row, end_row))
}

/// 插件的主要逻辑函数
fn rotate_chars(
    buffer: &mut api::Buffer,
    row_range: std::ops::Range<usize>,
    // col_range: std::ops::Range<usize>,
    direction: bool,
    steps: usize,
) -> oxi::Result<()> {
    // 获取选中的范围lines [start_row, end_row)
    let content = buffer.get_lines(row_range.to_owned(), false)?;

    let content_string_list = content
        .enumerate()
        .map(|(_, s)| s.to_string())
        .collect::<Vec<_>>();

    // let rotated_content = rotate_content(&content_string_list, direction, steps as u8);

    let rotated_content = rotate_unicode_content(&content_string_list, direction, steps as u8, false);

    // 替换选中的内容
    buffer.set_lines(row_range, false, rotated_content)?;

    Ok(())
}

/// 注册插件和命令
#[oxi::plugin]
fn nvim_rotate_chars() -> Dictionary {
    let rotate_chars_func = Function::from_fn(move |(direction, steps): (bool, usize)| {
        // 获取当前缓冲区
        let mut buffer = api::get_current_buf();

        // 获取选中的范围
        let selection = get_visual_selection_content(&buffer).unwrap();

        // println!("selection : {:?}", selection);

        // convert selection to range
        let selection = std::ops::Range {
            start: selection.0 - 1,
            end: selection.1,
        };

        // 调用旋转函数
        let _ = rotate_chars(&mut buffer, selection, direction, steps);
    });

    let rotate_chars_with_range_func = Function::from_fn(
        move |(direction, steps, start_line, end_line): (bool, usize, usize, usize)| {
            // 获取当前缓冲区
            let mut buffer = api::get_current_buf();


            // println!("start_line : {}, end_line : {}", start_line, end_line);
            // convert selection to range
            let selection = std::ops::Range {
                start: start_line,
                end: end_line,
            };

            // 调用旋转函数
            let _ = rotate_chars(&mut buffer, selection, direction, steps);
        },
    );

    Dictionary::from_iter([
        ("RotateChars", Object::from(rotate_chars_func)),
        (
            "RotateCharsWithRange",
            Object::from(rotate_chars_with_range_func),
        ),
    ])
}
