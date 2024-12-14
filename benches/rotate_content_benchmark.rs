
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nvim_rotate_chars::operator::rotate_content;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// 生成随机字符串向量
fn generate_random_content(num_lines: usize, line_length: usize) -> Vec<String> {
    let mut rng = thread_rng();
    (0..num_lines)
        .map(|_| {
            (&mut rng).sample_iter(&Alphanumeric)
               .take(line_length)
                .map(char::from)
                .collect()
        })
        .collect()
}

/// 基准测试函数
fn bench_rotate_content(c: &mut Criterion) {
    // 设置不同的测试场景
    let test_cases = vec![
        ("Small Dataset", 100, 50),
        ("Medium Dataset", 1_000, 100),
        ("Large Dataset", 100_000, 100),
    ];

    for (name, num_lines, line_length) in test_cases {
        // 生成测试数据
        let content = generate_random_content(num_lines, line_length);
        let direction = true;
        let steps = 3;

        // 使用黑盒避免编译器优化
        c.bench_function(&format!("rotate_content - {}", name), |b| {
            b.iter(|| {
                let rotated =
                    rotate_content(black_box(&content), black_box(direction), black_box(steps));
                black_box(rotated);
            })
        });
    }
}

criterion_group!(benches, bench_rotate_content);
criterion_main!(benches);
