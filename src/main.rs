use std::env;
use std::fs::{self, File, OpenOptions};
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle}; // 进度条库
use std::error::Error;
use std::io::Write;

fn check_ffmpeg_installed() -> bool {
    let ffmpeg_check = Command::new("ffmpeg").arg("-version").output();
    ffmpeg_check.is_ok()
}

fn install_instructions() {
    let os_type = env::consts::OS;
    match os_type {
        "macos" => {
            println!("请使用 Homebrew 安装 ffmpeg：");
            println!("brew install ffmpeg");
        }
        "linux" => {
            println!("请使用以下命令安装 ffmpeg：");
            println!("Debian/Ubuntu: sudo apt-get install ffmpeg");
            println!("CentOS: sudo yum install ffmpeg");
        }
        "windows" => {
            println!("请从 https://ffmpeg.org/download.html 下载并安装 Windows 版本的 ffmpeg，并确保将其添加到系统 PATH 中。");
        }
        _ => {
            println!("无法确定操作系统，请访问 https://ffmpeg.org 下载并安装 ffmpeg。");
        }
    }
}

fn print_help() {
    println!("AudioMorpher - 音频转换工具");
    println!("用法：");
    println!("  audiomorpher [输入目录]");
    println!("选项：");
    println!("  -h          显示帮助信息");
    println!("说明：");
    println!("  将指定目录下的所有 .m4a 文件转换为 .mp3，并输出到 ./output 目录中。如果没有指定输入目录，默认使用当前目录。");
}

fn convert_m4a_to_mp3(input_path: &str, output_path: &str, log_file: &mut File) -> Result<(), Box<dyn Error>> {
    // 调用ffmpeg并将日志重定向到log文件
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg(output_path)
        .stdout(Stdio::null())  // 将标准输出重定向
        .stderr(Stdio::piped()) // 将标准错误输出重定向
        .spawn()?
        .wait_with_output()?;

    // 将ffmpeg的stderr写入log文件
    if !status.stdout.is_empty() {
        log_file.write_all(&status.stdout)?;
    }
    if !status.stderr.is_empty() {
        log_file.write_all(&status.stderr)?;
    }

    if !status.status.success() {
        eprintln!("Failed to convert {} to MP3", input_path);
        return Err(Box::from("FFmpeg conversion failed"));
    }

    // 将成功转换的路径写入日志文件
    writeln!(log_file, "Successfully converted: {} -> {}", input_path, output_path)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 检查是否安装了ffmpeg
    if !check_ffmpeg_installed() {
        eprintln!("Error: ffmpeg 未安装或无法找到。");
        install_instructions();
        return Ok(());
    }

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 如果传递了 -h 参数，打印帮助信息并退出
    if args.len() > 1 && args[1] == "-h" {
        print_help();
        return Ok(());
    }

    // 如果传递了参数，使用用户指定的目录；否则使用当前执行目录
    let input_directory = if args.len() > 1 {
        &args[1]
    } else {
        "." // 默认使用当前目录
    };

    let output_directory = "./output"; // 输出目录
    let log_file_path = "./ffmpeg.log"; // 日志文件路径

    // 创建输出目录
    fs::create_dir_all(output_directory)?;

    // 打开日志文件，追加模式
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)?;

    // 收集所有待转换的文件
    let files: Vec<PathBuf> = WalkDir::new(input_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().extension().and_then(|e| e.to_str()) == Some("m4a"))
        .map(|entry| entry.path().to_path_buf())
        .collect();

    // 统计待转换文件数量
    let total_files = files.len();
    if total_files == 0 {
        println!("没有找到 .m4a 文件。");
        return Ok(());
    }

    // 设置进度条
    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")?
            .progress_chars("#>-")
    );

    // 遍历所有文件并进行转换
    for file in files {
        let input_file = file.to_str().unwrap();
        let output_file_name = file.file_stem().unwrap().to_str().unwrap().to_owned() + ".mp3";
        let output_file = Path::new(output_directory).join(output_file_name);

        // 如果输出文件已经存在，跳过转换
        if output_file.exists() {
            writeln!(log_file, "文件已存在，跳过：{}", output_file.to_str().unwrap())?;
            progress_bar.inc(1);
            continue;
        }

        // 执行转换，将日志输出到文件
        if let Err(e) = convert_m4a_to_mp3(input_file, output_file.to_str().unwrap(), &mut log_file) {
            writeln!(log_file, "Error converting file {}: {}", input_file, e)?;
        }

        progress_bar.inc(1); // 只更新一次进度条
    }

    // 完成进度条
    progress_bar.finish_with_message("转换完成！");

    // 提示打开输出目录
    println!("转换完成，请打开 {} 查看转换后的文件。", fs::canonicalize(output_directory)?.display());

    Ok(())
}
