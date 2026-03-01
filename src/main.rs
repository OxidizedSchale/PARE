/*
 * Project: PARE (PARE's Accelerated Runtime Elimination)
 * GitHub: https://github.com/OxidizedSchale/PARE-s-Accelerated-Runtime-Elimination
 * 
 * 版权所有 (C) 2026 OxidizedSchale & PARE Contributors
 * 
 * 本程序是自由软件：您可以自由分发和/或修改它。
 * 它遵循由自由软件基金会 (Free Software Foundation) 发布的
 * GNU 通用公共许可证 (GNU General Public License) 第 3 版。
 *
 * ----------------------------------------------------------------------------
 *
 * [项目架构概述 / Architecture Overview]
 *
 * PARE 是一个旨在彻底干掉 Python 运行时的激进转译编译器。
 * 它的核心逻辑如下：
 *
 * 1. 前端解析 (Frontend): 使用 rustpython-parser 将 .py 源码转化为 AST。
 * 2. 类型推导 (Inference): 静态分析变量生命周期，尝试将 PyObject 降维成裸 C 类型。
 * 3. 裸机转译 (Transpilation): 将 Python 语法树映射为极致精简的纯 C 代码。
 * 4. 极限编译 (Backend): 自动调用 GCC (-O3) 或 MSVC (/O2) 开启现代指令集优化 (AVX2/SIMD)。
 * 5. 链接增强 (Linking): 深度集成 mold/sold 链接器，实现瞬间构建。
 *
 * [目标 / Target]
 * - 实现真正的 Zero-Dependency 二进制分发。
 * - 在计算密集型任务中达到与原生 C 相当的速度。
 *
 */
   
#![allow(warnings)] ///关闭rust的大傻逼警告/Turn off the damn rust compiler warnings.

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use rustpython_parser::{ast, Parse};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// PARE 命令行工具：将 Python 脚本献祭给 GCC 以换取力量
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 输入的 Python 文件 (Input .py file)
    #[arg(short, long)]
    input: PathBuf,

    /// 输出的可执行文件名称 (Output binary name)
    #[arg(short, long, default_value = "pare_out")]
    output: PathBuf,

    /// 强制开启指令集优化, 如 -march=native (Enable aggressive optimizations)
    #[arg(long, default_value_t = true)]
    optimize: bool,

    /// 是否使用极速链接器 mold (Use mold linker)
    #[arg(long, default_value_t = false)]
    use_mold: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum CType { Int, Float, Unknown }

impl CType {
    fn to_c_str(&self) -> &str {
        match self {
            CType::Int => "long long",
            CType::Float => "double",
            _ => "void*",
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{}", "PARE: 启动加速运行时消除程序...".bright_cyan().bold());

    // 1. 读取源码 (Read Source)
    let source = fs::read_to_string(&args.input)
        .with_context(|| format!("无法读取文件: {:?}", args.input))?;

    // 2. 解析 AST (Parse AST)
    let ast_suite = ast::Suite::parse(&source, "<pare>")
        .with_context(|| "Python 语法解析失败，请检查代码合法性。".red())?;

    // 3. 生成代码 (Generate Code)
    let mut symbols = HashMap::new();
    let c_code = transpile(&ast_suite, &mut symbols)?;

    let temp_dir = tempfile::tempdir()?;
    let c_path = temp_dir.path().join("out.c");
    fs::write(&c_path, c_code)?;

    // 4. 调用编译器 (Invoke Compiler)
    compile(&c_path, &args)?;

    Ok(())
}

/// 核心转译引擎 (Core Transpiler Engine)
fn transpile(suite: &ast::Suite, symbols: &mut HashMap<String, CType>) -> Result<String> {
    let mut code = String::new();
    code.push_str("#include <stdio.h>\n#include <stdlib.h>\n\nint main() {\n");

    for stmt in suite {
        match stmt {
            // 处理赋值: a = 1
            ast::Stmt::Assign(a) => {
                if let Some(ast::Expr::Name(name_node)) = a.targets.first().map(|e| &**e) {
                    let var_name = name_node.id.as_str();
                    let (val_str, val_type) = eval_expr(&a.value, symbols);
                    
                    if !symbols.contains_key(var_name) {
                        code.push_str(&format!("    {} {} = {};\n", val_type.to_c_str(), var_name, val_str));
                        symbols.insert(var_name.to_string(), val_type);
                    } else {
                        code.push_str(&format!("    {} = {};\n", var_name, val_str));
                    }
                }
            }
            // 处理简单的 For 循环: for i in range(10):
            ast::Stmt::For(f) => {
                if let ast::Expr::Name(target) = &*f.target {
                    if let ast::Expr::Call(call) = &*f.iter {
                        if let ast::Expr::Name(func_name) = &*call.func {
                            if func_name.id.as_str() == "range" {
                                let limit = if let Some(arg) = call.args.first() {
                                    let (s, _) = eval_expr(arg, symbols); s
                                } else { "0".to_string() };
                                
                                let var = target.id.as_str();
                                code.push_str(&format!("    for (long long {} = 0; {} < {}; {}++) {{\n", var, var, limit, var));
                                // 这里简化处理，实际需要递归调用 transpile 处理 body
                                code.push_str("        // Body translation logic here...\n");
                                code.push_str("    }\n");
                            }
                        }
                    }
                }
            }
            // 处理 Print
            ast::Stmt::Expr(e) => {
                if let ast::Expr::Call(call) = &*e.value {
                    if let ast::Expr::Name(n) = &*call.func {
                        if n.id.as_str() == "print" {
                            if let Some(arg) = call.args.first() {
                                let (s, t) = eval_expr(arg, symbols);
                                let fmt = if t == CType::Float { "%f" } else { "%lld" };
                                code.push_str(&format!("    printf(\"{}\\n\", {});\n", fmt, s));
                            }
                        }
                    }
                }
            }
            _ => code.push_str("    // [暂不支持的语法节点]\n"),
        }
    }

    code.push_str("    return 0;\n}\n");
    Ok(code)
}

fn eval_expr(expr: &ast::Expr, _symbols: &HashMap<String, CType>) -> (String, CType) {
    match expr {
        ast::Expr::Constant(c) => match &c.value {
            ast::Constant::Int(i) => (i.to_string(), CType::Int),
            ast::Constant::Float(f) => (f.to_string(), CType::Float),
            _ => ("0".to_string(), CType::Unknown),
        },
        ast::Expr::Name(n) => (n.id.to_string(), CType::Int), // 简化推导
        ast::Expr::BinOp(b) => {
            let (l, _) = eval_expr(&b.left, _symbols);
            let (r, _) = eval_expr(&b.right, _symbols);
            (format!("({} + {})", l, r), CType::Int) // 演示用，暂只处理加法
        }
        _ => ("0".to_string(), CType::Unknown),
    }
}

/// 自动化跨平台编译执行器 (Cross-platform Compiler Invoker)
fn compile(c_src: &Path, args: &Args) -> Result<()> {
    let is_windows = cfg!(target_os = "windows");
    
    // 优先尝试 GCC (GNU)，如果找不到且在 Windows 则尝试 MSVC
    let mut cmd = if is_windows {
        // 探测系统中是否有 gcc (MinGW)
        if Command::new("gcc").arg("--version").output().is_ok() {
            println!("🛠️  检测到 GNU 工具链...");
            get_gcc_cmd(c_src, args)
        } else {
            println!("🛠️  未检测到 GCC, 尝试切换至 MSVC (cl.exe)...");
            get_msvc_cmd(c_src, args)
        }
    } else {
        get_gcc_cmd(c_src, args)
    };

    let status = cmd.status().context("编译器启动失败，请检查 GCC 或 MSVC 是否安装")?;
    
    if status.success() {
        println!("{}", "🎉 编译成功！输出二进制已就绪。".bright_green().bold());
    } else {
        println!("{}", "❌ 编译过程中断，底层编译器报错。".red().bold());
    }

    Ok(())
}

fn get_gcc_cmd(src: &Path, args: &Args) -> Command {
    let mut c = Command::new("gcc");
    c.arg("-O3").arg(src).arg("-o").arg(&args.output);
    if args.optimize { c.arg("-march=native").arg("-flto"); }
    if args.use_mold { c.arg("-fuse-ld=mold"); }
    c
}

fn get_msvc_cmd(src: &Path, args: &Args) -> Command {
    let mut c = Command::new("cl.exe");
    c.arg("/O2").arg("/Ot").arg(src).arg(format!("/Fe:{}", args.output.display()));
    if args.optimize { c.arg("/arch:AVX2"); } // 假设现代 CPU 支持 AVX2
    c
}
