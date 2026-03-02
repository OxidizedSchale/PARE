# PARE: PARE's Accelerated Runtime Elimination 🔪🔥
# ⚠️ WARNING / 警告
The current PARE program is in BETA. Do not use it for critical production code! 目前的 PARE 程序仍处于内测版，请勿直接用于处理核心业务代码！
> "Speed is a human right. Python's runtime is an obstacle." > “速度是基本人权。Python 的运行时是进化的障碍。”
> 
# 📖 What is PARE? / 什么是 PARE？
PARE is a high-performance Hybrid Python-to-C Transpiler engineered in Rust.

PARE 是一个由 Rust 驱动的高性能混合型 Python 转 C 转译器。

Unlike traditional wrappers, PARE performs a "Lowering Operation" on your code. By parsing the Python AST, PARE identifies high-performance paths (Integers, Floats) and transpiles them into Native C, while keeping the CPython C-API as a robust fallback for complex logic and ecosystem compatibility.

与传统的打包工具不同，PARE 对代码进行**“降维打击”。通过解析 Python AST，PARE 识别出高性能路径（整数、浮点数）并将其转译为原生 C**，同时保留 CPython C-API 作为复杂逻辑和生态兼容性的坚实后盾。
# 🚀 Key Features / 核心特性
 * ⚡ Hybrid Acceleration (CPython Strike): Seamlessly blends native C speed with Python's ecosystem.
 * 
   混合强袭加速：无缝融合原生 C 的速度与 Python 的生态兼容性。
   
 * 🦀 Rust-Injected Frontend: Uses rustpython-parser for lightning-fast, rock-solid AST analysis.

   Rust 注入前端：使用 rustpython-parser 进行极速且坚如磐石的 AST 分析。
   
 * 🧬 State Synchronization: Bi-directional variable syncing between native C and Python globals.
   
   状态同步机制：实现原生 C 变量与 Python 全局空间（globals）的双向实时同步。
   
 * 🎨 Embedded Resources: The Studio edition features static-linked fonts for a zero-dependency GUI.

   内置资源管理：Studio 版集成静态链接字体，实现零依赖的图形化界面。
   
 * 🔥 Aggressive zig cc Optimization: Automatically invokes zig cc -O3 to squeeze every drop of CPU juice.

   激进 zig CC 优化：自动调用 zig cc -O3 以榨干 CPU 的每一滴性能。
# 🛠️ Requirements / 环境要求
 * Rust (Cargo)
 * zig (with python3-dev headers)
 * CPython 3.10+
# 📖 Usage / 使用方法
 * Launch PARE Studio: Run the binary and witness the power.
   
   启动 PARE Studio：运行程序，见证降维打击的力量。
 * Select Mission: Pick your .py script.
   
   选择任务：选取你的 Python 脚本。
  
 * Execute Transpilation: Hit the "Nuitka Killer" button.
   
   执行转译：点击 “Nuitka Killer” 强袭编译按钮。
PARE will generate a standalone executable that runs logic at C speeds while maintaining 100% compatibility with libraries like numpy.

PARE 将生成一个独立的可执行文件，在保持与 numpy 等库 100% 兼容的同时，以 C 语言的速度运行逻辑。

# 🗺️ Roadmap / 开发路线

 * [x] Hybrid Type Inference (Int, Float, Dynamic)

 * [x] GUI Mission Control (egui powered)

 * [x] Resource Embedding (Static fonts)
  
 * [ ] #![no_std] Support (The ultimate compatibility leap)/无std模式
  
 * [x] Pure C Loops (Deep elimination of iteration overhead)/纯C循环

 * [ ] The "GIL-Killer" (Native C multi-threading)/GIL解释器解除（原生C线程）
# 📜 License / 许可证
Licensed under GPL-3.0./GPL3.0许可

PARE is a tool for the people. If you use it to build the future, the future must remain open.

PARE 是属于人民的工具。如果你用它构建未来，那么未来必须保持开放。
# 🤝 Join the Cult of Speed / 加入速度神教
If you believe Python is a great language trapped in a slow body, join us. 如果你相信 Python 是一门被困在缓慢躯体里的伟大语言，请加入我们。

PARE's Accelerated Runtime Elimination - Pure Speed, No Bullshit.

# 📩 Contact / 联系我们
 * Author: OxidizedSchale
   
 * Email: 3997101522@qq.com
