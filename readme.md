## 介绍

这是一个采用nvim-oxi开发的nvim插件，用于旋转选中的文本字符。

(1) example1
比如：
```
aaa
bbb
ccc
```
向右旋转1个字符，结果为：
```
bbb
ccc
ddd
```
向右旋转25个字符，结果为：
```
zzz
aaa
bbb
```

(2) example2
比如：
```
aaa
bbb
ccc
```
向右旋转1个字符，再次向左旋转1个字符，结果为：
```
aaa
bbb
ccc
```

## 编译与配置

```
cargo build --release
```

```
cp target/release/libnvim_rotate_chars.dylib ~/.config/nvim/lua/nvim_rotate_chars.so
cp config/rotate_chars.lua ~/.config/nvim/lua/rotate_chars.lua
```

在你的`init.lua`中添加如下配置：
```
vim.opt.runtimepath:append(',~/.config/nvim/lua')

require('rotate_chars')
```

## 使用
### 使用方法1

通过快捷键`<leader-r>`进行旋转。根据提示输入需要旋转的方向（true代表向右， false代表向左）和字符数。
例如：visual mode下将选中的行，向右旋转1个字符。
```
Enter direction and rotation number: true 1
```

### 使用方法2

通过Command输入进行旋转。
visual mode下将选中的行，向右旋转1个字符。
```bash
:'<,'> RotateChars true 1
```
向左旋转1个字符。
```bash
:'<,'> RotateChars false 1
```

## benchmark

```
rotate_content - Small Dataset
                        time:   [27.263 µs 27.562 µs 28.031 µs]
                        change: [+0.8650% +1.5615% +2.4745%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

rotate_content - Medium Dataset
                        time:   [131.15 µs 133.71 µs 136.47 µs]
                        change: [+4.0361% +6.8314% +9.7280%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

rotate_content - Large Dataset
                        time:   [10.136 ms 10.222 ms 10.336 ms]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
```