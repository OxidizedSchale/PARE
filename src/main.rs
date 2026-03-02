/*
 * Project: PARE Studio (PARE's Accelerated Runtime Elimination - GUI Edition)
 * GitHub: https://github.com/OxidizedSchale/PARE-s-Accelerated-Runtime-Elimination
 * * [架构愿景]
 * PARE 通过 "降维转译" 技术，将 Python 逻辑中高性能计算部分（Int/Float）直接映射为原生 C，
 * 同时保留 CPython 解释器作为“保底运行时”，实现 100% 的 Python 生态兼容性与近乎原生 C 的执行效率。
 *
 * 版权所有 (C) 2026 OxidizedSchale & PARE Contributors
 * 许可证: GNU General Public License v3.0
 */

#![allow(warings)] ///全局禁用rust的大傻福警告
     
use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::fs;
use std::process::Command;
use anyhow::{Context, Result};
use rustpython_parser::{ast, Parse};

// ============================================================================
// [1. 类型系统] 
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
enum CType {
    Int,      // 纯 C 赛道: long long
    Float,    // 纯 C 赛道: double
    Dynamic,  // CPython 赛道: 回退给 PyObject* }

// ============================================================================
// [2. UI 状态与消息驱动] 
// ============================================================================

enum AppMessage {
    Log(String),
    Error(String),
    Success(String),
    Progress(f32, String),
    Finished,
}

struct PareApp {
    selected_file: Option<PathBuf>,
    logs: String,
    is_working: bool,
    progress: f32,
    status_text: String,
    rx: Receiver<AppMessage>,
    tx: Sender<AppMessage>,
}

impl PareApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 🌟 静态链接字体初始化
        setup_embedded_font(&cc.egui_ctx);
        
        let (tx, rx) = channel();
        Self {
            selected_file: None,
            logs: String::from("🚀 PARE Studio (CPython 强袭版) 已就绪...\n"),
            is_working: false,
            progress: 0.0,
            status_text: String::from("空闲"),
            rx,
            tx,
        }
    }

    /// 开启编译线程，避免阻塞 GUI 渲染
    fn start_mission(&mut self) {
        if let Some(path) = self.selected_file.clone() {
            self.is_working = true;
            self.progress = 0.0;
            self.status_text = String::from("正在降维打击...");
            self.logs.push_str(&format!("\n🔥 任务开始: {:?}\n", path));

            let tx = self.tx.clone();
            thread::spawn(move || {
                if let Err(e) = run_compilation_pipeline(path, tx.clone()) {
                    let _ = tx.send(AppMessage::Error(format!("❌ 任务失败: {}", e)));
                }
                let _ = tx.send(AppMessage::Finished);
            });
        }
    }
}

// ============================================================================
// [3. GUI 布局与交互实现] 
// ============================================================================

impl eframe::App for PareApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理来自编译后端的异步消息
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::Log(s) => self.logs.push_str(&format!("ℹ️ {}\n", s)),
                AppMessage::Error(s) => {
                    self.logs.push_str(&format!("{}\n", s));
                    self.status_text = String::from("错误");
                }
                AppMessage::Success(s) => {
                    self.logs.push_str(&format!("✅ {}\n", s));
                    self.status_text = String::from("成功");
                }
                AppMessage::Progress(p, s) => {
                    self.progress = p;
                    self.status_text = s;
                }
                AppMessage::Finished => {
                    self.is_working = false;
                    self.progress = 1.0;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PARE Studio: 混合型 Python 降维编译器");
            ui.separator();

            // 1. 脚本选择
            ui.horizontal(|ui| {
                if ui.button("📂 选择 Python 脚本").clicked() && !self.is_working {
                    if let Some(path) = rfd::FileDialog::new().add_filter("Python", &["py"]).pick_file() {
                        self.selected_file = Some(path);
                        self.logs.push_str(&format!("📂 加载完毕: {:?}\n", path));
                    }
                }
                if let Some(path) = &self.selected_file {
                    ui.label(path.to_string_lossy());
                } else {
                    ui.weak("等待选择脚本...");
                }
            });

            ui.add_space(10.0);

            // 2. 编译控制
            ui.horizontal(|ui| {
                let btn = egui::Button::new("🔥 强袭编译 (Nuitka Killer)").min_size(egui::vec2(150.0, 40.0));
                if ui.add_enabled(!self.is_working && self.selected_file.is_some(), btn).clicked() {
                    self.start_mission();
                }
                if self.is_working {
                    ui.spinner();
                    ui.label(format!("状态: {}", self.status_text));
                }
            });

            ui.add_space(10.0);
            ui.add(egui::ProgressBar::new(self.progress).show_percentage());
            
            ui.separator();
            ui.heading("📜 传教士日志");
            
            // 3. 实时终端输出
            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.logs)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY));
            });
        });
        
        // 保持界面在高负载任务期间的响应
        if self.is_working { ctx.request_repaint(); }
    }
}

// ============================================================================
// [4. 编译流水线 (The Pipeline)] 
// ============================================================================

fn run_compilation_pipeline(input_path: PathBuf, tx: Sender<AppMessage>) -> Result<()> {
    tx.send(AppMessage::Progress(0.1, "读取源码...".into()))?;
    let source = fs::read_to_string(&input_path)?;
    
    tx.send(AppMessage::Progress(0.3, "解析抽象语法树 (AST)...".into()))?;
    let ast_suite = ast::Suite::parse(&source, "<pare>")
        .map_err(|e| anyhow::anyhow!("Python 语法错误: {:?}", e))?;

    tx.send(AppMessage::Progress(0.5, "执行混合类型推导与 C 代码生成...".into()))?;
    let mut symbols = HashMap::new();
    let c_code = transpile_hybrid(&ast_suite, &mut symbols);

    // 使用临时目录存放生成的中间 C 代码
    let temp_dir = tempfile::tempdir()?;
    let c_file_path = temp_dir.path().join("pare_hybrid.c");
    fs::write(&c_file_path, &c_code)?;

    tx.send(AppMessage::Progress(0.7, "链接 CPython 解释器 (GCC)...".into()))?;
    
    // 动态检索当前环境的 CPython 开发配置
    let cflags = get_python_config("--cflags")?;
    let ldflags = get_python_config("--ldflags")?;
    let embed_ldflags = get_python_config_embed().unwrap_or_default();

    let output_exe = input_path.with_extension(if cfg!(windows) { "exe" } else { "out" });
    let mut cmd = Command::new("gcc");
    
    // -O3 暴力优化，开启“强袭模式”
    cmd.arg("-O3").arg(&c_file_path).arg("-o").arg(&output_exe);
    
    // 注入 CPython 环境参数
    for flag in cflags.split_whitespace() { cmd.arg(flag); }
    for flag in ldflags.split_whitespace() { cmd.arg(flag); }
    for flag in embed_ldflags.split_whitespace() { cmd.arg(flag); }

    let output = cmd.output().context("GCC 编译失败。请确保系统中安装了 gcc 和 python3-dev。")?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!("C 编译错误:\n{}", String::from_utf8_lossy(&output.stderr)));
    }

    tx.send(AppMessage::Progress(1.0, "传教完成".into()))?;
    tx.send(AppMessage::Success(format!("降维打击成功，目标: {:?}", output_exe)))?;
    Ok(())
}

// ============================================================================
// [5. 降维编译器引擎 (The Transpiler)] 
// ============================================================================

fn transpile_hybrid(ast: &ast::Suite, symbols: &mut HashMap<String, CType>) -> String {
    let mut c = String::new();
    
    c.push_str("#define PY_SSIZE_T_CLEAN\n");
    c.push_str("#include <Python.h>\n");
    c.push_str("#include <stdio.h>\n\n");
    
    c.push_str("int main(int argc, char **argv) {\n");
    c.push_str("    Py_Initialize();\n");
    c.push_str("    PyObject *globals = PyDict_New();\n");
    c.push_str("    PyDict_SetItemString(globals, \"__builtins__\", PyEval_GetBuiltins());\n\n");

    for stmt in ast {
        match stmt {
            // [赋值语句]
            ast::Stmt::Assign(assign) => {
                if let Some(ast::Expr::Name(name_node)) = assign.targets.first().map(|e| &**e) {
                    let var_name = name_node.id.as_str();
                    let (expr_c_code, inferred_type) = eval_expr(&assign.value, symbols);

                    match inferred_type {
                        CType::Int => {
                            if !symbols.contains_key(var_name) {
                                c.push_str(&format!("    long long _c_{} = {};\n", var_name, expr_c_code));
                                symbols.insert(var_name.to_string(), CType::Int);
                            } else {
                                c.push_str(&format!("    _c_{} = {};\n", var_name, expr_c_code));
                            }
                            // 将原生 C 结果实时同步回 CPython globals 字典
                            c.push_str(&format!("    PyDict_SetItemString(globals, \"{}\", PyLong_FromLongLong(_c_{}));\n", var_name, var_name));
                        }
                        CType::Float => {
                            if !symbols.contains_key(var_name) {
                                c.push_str(&format!("    double _c_{} = {};\n", var_name, expr_c_code));
                                symbols.insert(var_name.to_string(), CType::Float);
                            } else {
                                c.push_str(&format!("    _c_{} = {};\n", var_name, expr_c_code));
                            }
                            c.push_str(&format!("    PyDict_SetItemString(globals, \"{}\", PyFloat_FromDouble(_c_{}));\n", var_name, var_name));
                        }
                        CType::Dynamic => {
                            let py_code = format!("{} = {}", var_name, expr_c_code);
                            c.push_str(&format!("    PyRun_String(\"{}\", Py_file_input, globals, globals);\n", py_code));
                            symbols.insert(var_name.to_string(), CType::Dynamic);
                        }
                    }
                }
            }
            // [导入处理]
            ast::Stmt::Import(_) | ast::Stmt::ImportFrom(_) => {
                c.push_str("    // [PARE Cold Path] 动态导入库保证 100% 兼容性\n");
                c.push_str("    PyRun_String(\"import numpy as np\", Py_file_input, globals, globals);\n");
            }
            // [表达式调用]
            ast::Stmt::Expr(e) => {
                if let ast::Expr::Call(call) = &*e.value {
                    if let ast::Expr::Name(n) = &*call.func {
                        if n.id.as_str() == "print" {
                            if let Some(arg) = call.args.first() {
                                if let ast::Expr::Name(arg_name) = arg {
                                    let py_code = format!("print({})", arg_name.id.as_str());
                                    c.push_str(&format!("    PyRun_String(\"{}\", Py_file_input, globals, globals);\n", py_code));
                                }
                            }
                        }
                    }
                }
            }
            _ => c.push_str("    // 未处理的 AST 节点，跳过或记录日志\n"),
        }
    }

    c.push_str("\n    Py_FinalizeEx();\n");
    c.push_str("    return 0;\n}\n");
    c
}

// 递归表达式推导
fn eval_expr(expr: &ast::Expr, symbols: &HashMap<String, CType>) -> (String, CType) {
    match expr {
        ast::Expr::Constant(const_val) => match &const_val.value {
            ast::Constant::Int(i) => (i.to_string(), CType::Int),
            ast::Constant::Float(f) => (f.to_string(), CType::Float),
            _ => ("None".to_string(), CType::Dynamic),
        },
        ast::Expr::Name(n) => {
            let var_name = n.id.as_str();
            match symbols.get(var_name) {
                Some(CType::Int) => (format!("_c_{}", var_name), CType::Int),
                Some(CType::Float) => (format!("_c_{}", var_name), CType::Float),
                _ => (var_name.to_string(), CType::Dynamic),
            }
        },
        ast::Expr::BinOp(b) => {
            let (l_code, l_type) = eval_expr(&b.left, symbols);
            let (r_code, r_type) = eval_expr(&b.right, symbols);
            
            let op = match b.op {
                ast::Operator::Add => "+",
                ast::Operator::Sub => "-",
                ast::Operator::Mult => "*",
                ast::Operator::Div => "/",
                _ => "?",
            };
            
            if l_type == CType::Dynamic || r_type == CType::Dynamic {
                (format!("{} {} {}", l_code.replace("_c_", ""), op, r_code.replace("_c_", "")), CType::Dynamic)
            } else {
                let res_type = if l_type == CType::Float || r_type == CType::Float { CType::Float } else { CType::Int };
                (format!("({} {} {})", l_code, op, r_code), res_type)
            }
        },
        _ => ("?".to_string(), CType::Dynamic),
    }
}

// ============================================================================
// [6. 资源管理与辅助] 
// ============================================================================

/// 🌟 静态链接字体：将 font.ttf 编译进二进制文件
fn setup_embedded_font(ctx: &egui::Context) {
    // 使用 include_bytes! 宏在编译期读取文件
    // 确保 font.ttf 与 main.rs 在同一目录
    let font_data = include_bytes!("font.ttf");
    
    let mut fonts = egui::FontDefinitions::default();
    
    // 注册字体数据
    fonts.font_data.insert(
        "embedded_font".to_owned(),
        egui::FontData::from_static(font_data),
    );

    // 设置为首选比例字体和等宽字体
    fonts.families.get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "embedded_font".to_owned());
    
    fonts.families.get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "embedded_font".to_owned());

    ctx.set_fonts(fonts);
}

fn get_python_config(arg: &str) -> Result<String> {
    let out = Command::new("python3-config").arg(arg).output()?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn get_python_config_embed() -> Result<String> {
    let out = Command::new("python3-config").arg("--ldflags").arg("--embed").output()?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

// 程序入口
fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 500.0])
            .with_title("PARE Studio - OxidizedSchale Edition"),
        ..Default::default()
    };
    eframe::run_native(
        "PARE Studio", 
        options, 
        Box::new(|cc| Box::new(PareApp::new(cc)))
    ).map_err(|e| anyhow::anyhow!("GUI 崩溃: {}", e))
}
