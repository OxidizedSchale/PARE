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
 * 2. 类型推导 (Inference): 递归分析表达式，将 PyObject 降维成裸 C 类型 (long long / double)。
 * 3. 递归转译 (Recursive Transpilation): 支持嵌套循环 (Nested Loops) 和复杂表达式计算。
 * 4. 极限编译 (Backend): 自动探测 GCC (-O3) 或 MSVC (/O2) 开启硬件级优化。
 * 5. 链接增强 (Linking): 支持 mold 链接器，实现毫秒级构建。
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

/// PARE 命令行工具：将 Python 脚本献祭给 C 语言以换取绝对速度
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 输入的 Python 文件 (Input .py file)
    #[arg(short, long)]
    input: PathBuf,

    /// 输出的可执行文件名称 (Output binary name)
    #[arg(short, long, default_value = "pare_out")]
    output: PathBuf,

    /// 强制开启指令集优化 (Enable aggressive optimizations like -march=native)
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
    println!("{}", "⚡ PARE: 正在剥离 Python 运行时，准备降维打击...".bright_cyan().bold());

    // 1. 读取源码
    let source = fs::read_to_string(&args.input)
        .with_context(|| format!("无法读取文件: {:?}", args.input))?;

    // 2. 解析 AST
    let ast_suite = ast::Suite::parse(&source, "<pare>")
        .with_context(|| "❌ Python 语法错误！传教士拒绝处理异端代码。".red())?;

    // 3. 递归生成 C 代码
    let mut symbols = HashMap::new();
    let mut c_body = String::new();
    
    // 核心：递归处理所有语句
    for stmt in &ast_suite {
        c_body.push_str(&transpile_stmt(stmt, &mut symbols, 1)?);
    }

    // 拼接成完整的 C 程序
    let mut full_c_code = String::new();
    full_c_code.push_str("#include <stdio.h>\n#include <stdlib.h>\n#include <math.h>\n\n");
    full_c_code.push_str("int main(int argc, char **argv) {\n");
    full_c_code.push_str(&c_body);
    full_c_code.push_str("    return 0;\n}\n");

    // 4. 写入临时文件
    let temp_dir = tempfile::tempdir()?;
    let c_path = temp_dir.path().join("transpiled.c");
    fs::write(&c_path, &full_c_code)?;

    // 调试用：如果需要看生成的 C 代码，取消下面注释
    // println!("--- Generated C Code ---\n{}\n-----------------------", full_c_code);

    // 5. 调用底层编译器
    compile(&c_path, &args)?;

    Ok(())
}

/// 核心：递归转译 Python 语句
fn transpile_stmt(stmt: &ast::Stmt, symbols: &mut HashMap<String, CType>, indent_level: usize) -> Result<String> {
    let mut code = String::new();
    let indent = "    ".repeat(indent_level);

    match stmt {
        // [赋值语句] a = x + y
        ast::Stmt::Assign(a) => {
            if let Some(ast::Expr::Name(name_node)) = a.targets.first().map(|e| &**e) {
                let var_name = name_node.id.as_str();
                let (val_str, val_type) = eval_expr(&a.value, symbols);
                
                if !symbols.contains_key(var_name) {
                    code.push_str(&format!("{}{} {} = {};\n", indent, val_type.to_c_str(), var_name, val_str));
                    symbols.insert(var_name.to_string(), val_type);
                } else {
                    code.push_str(&format!("{}{} = {};\n", indent, var_name, val_str));
                }
            }
        }

        // [循环语句] for i in range(n): (支持嵌套)
        ast::Stmt::For(f) => {
            if let ast::Expr::Name(target) = &*f.target {
                if let ast::Expr::Call(call) = &*f.iter {
                    if let ast::Expr::Name(func_name) = &*call.func {
                        if func_name.id.as_str() == "range" {
                            let limit = if let Some(arg) = call.args.first() {
                                let (s, _) = eval_expr(arg, symbols); s
                            } else { "0".to_string() };
                            
                            let var = target.id.as_str();
                            // 在符号表注册循环变量
                            symbols.insert(var.to_string(), CType::Int);
                            
                            code.push_str(&format!("{}for (long long {} = 0; {} < {}; {}++) {{\n", indent, var, var, limit, var));
                            
                            // 递归处理循环体内部语句
                            for body_stmt in &f.body {
                                code.push_str(&transpile_stmt(body_stmt, symbols, indent_level + 1)?);
                            }
                            
                            code.push_str(&format!("{}}}\n", indent));
                        }
                    }
                }
            }
        }

        // [函数调用] 目前只处理 print()
        ast::Stmt::Expr(e) => {
            if let ast::Expr::Call(call) = &*e.value {
                if let ast::Expr::Name(n) = &*call.func {
                    if n.id.as_str() == "print" {
                        if let Some(arg) = call.args.first() {
                            let (val_str, val_type) = eval_expr(arg, symbols);
                            let fmt = match val_type {
                                CType::Float => "%f",
                                _ => "%lld",
                            };
                            code.push_str(&format!("{}printf(\"{}\\n\", {});\n", indent, fmt, val_str));
                        }
                    }
                }
            }
        }
        
        _ => code.push_str(&format!("{}// [PARE: 暂不支持的语法节点]\n", indent)),
    }
    Ok(code)
}

/// 核心：递归推导并生成表达式
fn eval_expr(expr: &ast::Expr, symbols: &HashMap<String, CType>) -> (String, CType) {
    match expr {
        // 常量数字
        ast::Expr::Constant(c) => match &c.value {
            ast::Constant::Int(i) => (i.to_string(), CType::Int),
            ast::Constant::Float(f) => (f.to_string(), CType::Float),
            _ => ("0".to_string(), CType::Unknown),
        },
        
        // 变量引用
        ast::Expr::Name(n) => {
            let name = n.id.as_str();
            let t = symbols.get(name).cloned().unwrap_or(CType::Int);
            (name.to_string(), t)
        },
        
        // 二元运算 (递归处理 a + b * c)
        ast::Expr::BinOp(b) => {
            let (l_str, l_type) = eval_expr(&b.left, symbols);
            let (r_str, r_type) = eval_expr(&b.right, symbols);
            
            let op_str = match b.op {
                ast::Operator::Add => "+",
                ast::Operator::Sub => "-",
                ast::Operator::Mult => "*",
                ast::Operator::Div => "/",
                ast::Operator::Mod => "%",
                _ => "+",
            };
            
            let res_type = if l_type == CType::Float || r_type == CType::Float {
                CType::Float
            } else {
                CType::Int
            };
            
            (format!("({} {} {})", l_str, op_str, r_str), res_type)
        },
        
        _ => ("0".to_string(), CType::Unknown),
    }
}

/// 跨平台编译器调用逻辑
fn compile(c_src: &Path, args: &Args) -> Result<()> {
    let is_windows = cfg!(target_os = "windows");
    
    // 尝试寻找编译器
    let mut cmd = if is_windows {
        if Command::new("gcc").arg("--version").output().is_ok() {
            println!("🛠️  检测到 GNU 工具链 (MinGW/GCC)...");
            get_gcc_cmd(c_src, args)
        } else {
            println!("🛠️  未检测到 GCC, 尝试切换至 MSVC (cl.exe)...");
            get_msvc_cmd(c_src, args)
        }
    } else {
        get_gcc_cmd(c_src, args)
    };

    let status = cmd.status().context("❌ 启动编译器失败！请确保你安装了 GCC 或 MSVC 并在环境变量中。")?;
    
    if status.success() {
        println!("{}", "🎉 [PARE] 编译成功！Python 运行时已被彻底消除。".bright_green().bold());
        println!("🚀 输出目标: {:?}", args.output);
    } else {
        println!("{}", "❌ 底层编译器报错，降维打击失败。".red().bold());
    }

    Ok(())
}

fn get_gcc_cmd(src: &Path, args: &Args) -> Command {
    let mut c = Command::new("gcc");
    c.arg("-O3").arg(src).arg("-o").arg(&args.output).arg("-lm"); // -lm 链接数学库
    if args.optimize {
        c.arg("-march=native").arg("-flto").arg("-ffast-math");
    }
    if args.use_mold {
        c.arg("-fuse-ld=mold");
    }
    c
}

fn get_msvc_cmd(src: &Path, args: &Args) -> Command {
    let mut c = Command::new("cl.exe");
    c.arg("/O2").arg("/Ot").arg("/GL").arg(src).arg(format!("/Fe:{}", args.output.display()));
    if args.optimize {
        c.arg("/arch:AVX2"); // 默认现代机器
    }
    c
}
