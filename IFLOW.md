# iFlow 项目上下文文件

## 项目概述

**项目名称**: sphere-n-rs  
**项目类型**: Rust 库  
**主要功能**: 生成 n 维球面上的低差异序列 (Low Discrepancy Sequence)  
**领域**: 数值计算、蒙特卡洛模拟、计算机图形学

这是一个 Rust 库，用于在 n 维球面上生成均匀分布的点序列。它通过数学计算和递归结构实现，利用 VdCorput 序列生成器将数字映射到球面。

## 核心组件

1. **VdCorput 序列生成器**: 生成在 0 到 1 之间均匀分布的数字序列
2. **插值函数**: 将生成的数字映射到球面
3. **SphereGen 特性**: 定义所有球生成器的通用接口
4. **Sphere3 和 SphereN 结构**: 实现三维和 n 维球面的生成逻辑
5. **缓存机制**: 使用 `cached` 属性和 `lazy_static` 来提升性能

## 主要文件结构

- `src/lib.rs`: 库的入口点，导出主要模块和结构
- `src/sphere_n.rs`: 实现 n 维球面生成的核心逻辑
- `src/cylind_n.rs`: 实现圆柱坐标方法生成球面点
- `Cargo.toml`: 项目依赖和元数据
- `README.md`: 项目说明和安装指南

## 依赖项

- `ndarray`: 多维数组操作
- `lazy_static`: 懒初始化
- `interp`: 插值计算
- `lds-rs`: 低差异序列基础库
- `cached`: 缓存机制
- `approx_eq`: 测试中使用近似相等比较

## 使用方法

### 安装

```bash
cargo install sphere-n-rs
```

### 用法示例

```rust
use sphere_n_rs::Sphere3;
use sphere_n_rs::SphereGen;

let mut sgen = Sphere3::new(&[2, 3, 5]);
sgen.reseed(10);
for _i in 0..10 {
    println!("{:?}", sgen.pop());
}
let res = sgen.pop();
```

## 构建和测试

### 构建项目

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

### 构建文档

```bash
cargo doc --open
```

## 开发约定

- 遵循 Rust 编码风格和最佳实践
- 使用 `#[inline]` 优化函数性能
- 通过 `lazy_static` 进行静态初始化
- 实现 `CylindGen` 和 `SphereGen` 特性来提供通用接口
- 使用递归结构来处理不同维度的球面

## 测试

项目包含多个单元测试，验证:
- 三维球面生成 (`test_sphere3`)
- n 维球面生成 (`test_sphere_n`)
- 圆柱坐标方法 (`test_cylind_n`)
- 归一化验证 (`test_normalized`)

## 许可证

双重许可证: MIT 或 Apache-2.0