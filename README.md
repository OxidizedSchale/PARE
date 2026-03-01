# PARE: PARE's Accelerated Runtime Elimination 🔪🔥

# Warning! The current PARE program is still in the beta version, please do not use our program to handle important work code!
# 警告！目前的 PARE 程序仍处于内测版，请先不要拿我们的程序去处理重要工作代码！
  
                                  
  PARE's Accelerated Runtime Elimination

> **"Speed is a human right. Python's runtime is an obstacle."**
> **“速度是基本人权。Python 的运行时是进化的障碍。”**

---

## 📖 What is PARE? / 什么是 PARE？

**PARE** is a high-performance **Python-to-C Transpiler** engineered in **Rust**. 

Most "Python compilers" are just wrappers. **PARE is different.** We don't just "pack" Python; we **strip it naked**. By parsing Python AST and generating lean, bare-metal C code, PARE eliminates the overhead of the bytecode interpreter and the Global Interpreter Lock (GIL), delivering the raw performance your hardware deserves.

**PARE** 是一个由 **Rust** 驱动的高性能 **Python 转 C 转译器**。

市面上大多数所谓的“Python 编译器”只是简单的打包工具。**PARE 与众不同。** 我们不只是“打包” Python；我们**将其剥离殆尽**。通过解析 Python AST 并生成精简的、裸机级别的 C 代码，PARE 彻底消除了字节码解释器和全局解释器锁（GIL）的开销，释放出硬件应有的原始性能。

---

## 🚀 Key Features / 核心特性

- **🚀 Zero-Interpreter Overhead**: Transpiles Python logic into native C structures. No VM, no PVM.
- 
  **零解释器开销**：将 Python 逻辑转译为原生 C 结构。无虚拟机，无 PVM。
  
- **🦀 Rust-Engineered**: A rock-solid frontend using `rustpython-parser` for lightning-fast analysis.
- 
  **Rust 驱动**：使用 `rustpython-parser` 构建坚如磐石且极速的前端分析引擎。
  
- **🔥 Aggressive Optimization**: Automatically invokes `gcc -O3` to squeeze every drop of CPU juice.
- 
  **激进优化**：自动调用 `gcc -O3` 以榨干 CPU 的每一滴性能。
  
- **⚡ Mold Integration**: Uses the world's fastest linker (`mold`) to make compilation instantaneous.
- 
  **Mold 集成**：集成全球最快的链接器 `mold`，让编译过程瞬间完成。
  
- **⚖️ GPLv3 Powered**: Freedom is mandatory. Open source, forever.
- 
  **GPLv3 驱动**：自由是强制性的。开源，直到永远。

---

## 🛠️ Installation / 安装

### Prerequisites / 前置条件
- **Rust** (Cargo)
- 
- **GCC** (with `-O3` support)

- 
- **Mold** (Optional, for hyper-fast linking)
- 
- **Python 3.10+** (For parsing)

# Clone the mission / 克隆库
git clone https://github.com/OxidizedSchale/PARE-s-Accelerated-Runtime-Elimination.git

cd PARE-s-Accelerated-Runtime-Elimination

# Build the preacher / 编译转换器
cargo build --release

---

## 📖 Usage / 使用方法

Create a Python script `mission.py`:/创建一个名为mission.py的文件，写入

x = 1000000

y = 2000000

z = x * y

print(z)

Convert it to a native beast / 将其转换为原生猛兽:

./target/release/pare --input mission.py --output mission_exe --use-mold


Run it / 运行:

./mission_exe

 //Output: 2000000000000 (at C speeds!)


---

## 🗺️ Roadmap / 开发路线

- [x] Basic Type Inference (Int, Float) / 基础类型推导

- [x] Expression Flattening / 表达式扁平化

- [x] **Pure C Loops** (Removing Python iteration overhead) / 纯 C 循环（消除迭代开销）

- [ ] **Static Scoping** / 静态作用域优化

- [ ] **The "GIL-Killer"** (Native C threading) / “GIL 杀手”（原生 C 多线程）

- [ ] **C-Buffer Strings** / 高性能 C 缓冲区字符串处理

---

## 📜 License / 许可证

Licensed under **GPL-3.0**. 

**PARE** is a tool for the people. If you use it to build the future, the future must remain open.

**PARE** 是属于人民的工具。如果你用它构建未来，那么未来必须保持开放。

---

## 🤝 Join the Cult of Speed / 加入速度神教

If you believe Python is a great language trapped in a slow body, **join us.** Contributions, stars, and bug reports are the holy-fire that keeps this project burning.

如果你相信 Python 是一门被困在缓慢躯体里的伟大语言，**请加入我们。** 你的贡献、Star 和 Bug 报告，都是维系这一项目生生不息的圣火。

**PARE's Accelerated Runtime Elimination - Pure Speed, No Bullshit.**

# 联系我们
作者邮箱：3997101522@qq.com
