# AudioMorpher - 音频转换工具

AudioMorpher 是一个用 Rust 编写的命令行工具，用于将 `m4a` 格式的音频文件转换为 `mp3` 格式。它支持指定输入目录和输出目录，并提供了一个进度条来显示转换进度。转换完成后，程序会提示用户查看输出目录中的文件。

## 安装

1. 确保你的系统已经安装了Rust编译器。如果没有，请访问[Rust官网](https://www.rust-lang.org/)，根据你的操作系统下载并安装Rust工具链。

2. 克隆AudioMorpher项目到本地，进入到项目目录中。

3. 在命令行中，运行 `cargo build --release` 来编译项目，编译成功后，将会在 `target\release` 目录下生成一个名为 `audiomorpher` 的可执行文件。

## 使用方法

1. **批量转换文件**：如果你需要转换一个目录下的所有m4a文件，可以在命令行中输入：`audiomorpher 输入目录`。AudioMorpher 会自动扫描指定目录下的所有m4a文件，并将它们转换为mp3格式。输出目录默认为当前目录下的 `output` 文件夹。

2. **查看帮助**：如果你需要查看帮助信息，可以在命令行中输入：`audiomorpher -h`，将会显示工具的帮助信息，包括用法、选项和说明。

3. **Example**:
```
(base)  ~/ audiomorpher "~/folder"
[00:00:10] [########################################] 21/21 (100%)
转换完成，请打开 /Users/output 查看转换后的文件。
```
## 贡献

欢迎贡献代码或提供反馈，有任何问题或建议，请在GitHub项目页面上提交Issue或通过邮件联系我。

欢迎贡献！请遵循以下步骤：

1. Fork 本项目
2. 创建您的特性分支 (`git checkout -b feature/YourFeature`)
3. 提交您的更改 (`git commit -m 'Add some feature'`)
4. 推送到分支 (`git push origin feature/YourFeature`)
5. 创建一个新的 Pull Request

## 许可证
本项目采用 MIT 许可证

## 版权声明

Copyright (c) 2024 版权所有
