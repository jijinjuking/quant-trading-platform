// src/standards/mod.rs

//! =====================================================
//! AI 工程开发规范（AI Engineering Development Spec）
//!
//! 1. 不破坏老代码，优先扩展/组合/适配/最小重构
//! 2. 受保护目录：src/core/, src/engine/, src/legacy/
//! 3. Rust 额外规范：禁止 unsafe、unwrap/expect，优先使用 Result/Option
//! 4. 编码前必须复述职责、分析冲突、列出方案、选择最小改动
//! 5. 自检清单：改变语义？引入 panic？影响历史调用？需要迁移？
//! =====================================================

// 引入 ai_spec.txt 内容作为常量
pub const AI_SPEC: &str = include_str!("ai_spec.txt");

// 可选：提供一个打印函数，方便 AI 或开发者直接查看
pub fn print_spec() {
    println!("{}", AI_SPEC);
}