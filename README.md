# Jimo

基于领域驱动设计（Domain-Driven Design, DDD）架构的 Rust 项目。

## 项目简介

Jimo 是一个采用 DDD 架构风格构建的 Rust 项目，旨在提供一种清晰、可维护的代码组织方式。该项目将业务逻辑分离到不同的层中，使得代码更加模块化、易于理解和扩展。

## 技术栈

- **Rust** - 系统编程语言
- **Cargo** - Rust 包管理工具
- **Serde** - 序列化/反序列化框架
- **Anyhow** - 错误处理
- **Futures** - 异步编程

## 项目结构

```
jimo/
├── src/                      # 主入口
├── jimo-domain/             # 领域层 - 核心业务逻辑
│   └── src/
│       ├── aggregate/       # 聚合根
│       │   ├── user/        # 用户聚合
│       │   ├── user_profile/# 用户资料聚合
│       │   ├── post/        # 文章聚合
│       │   ├── taxonomy/    # 分类聚合
│       │   ├── tag/         # 标签聚合
│       │   ├── role/        # 角色聚合
│       │   └── perm/        # 权限聚合
│       ├── projection/      # 投影查询
│       └── common/          # 公共组件
├── jimo-application/        # 应用层 - 用例编排
│   └── src/
│       └── usecase/
│           ├── post/        # 文章用例
│           ├── taxonomy/    # 分类用例
│           └── iam/         # 身份认证用例
├── jimo-adapter/            # 适配器层 - 接口定义
├── jimo-infrastructure/    # 基础设施层 - 外部服务集成
├── Cargo.toml               # 工作区配置
└── Cargo.lock               # 依赖锁定文件
```

## 架构说明

### 领域层 (jimo-domain)

领域层是项目的核心，包含所有的业务实体、业务规则和领域服务。主要包括：

- **聚合根 (Aggregate Root)**: 每个聚合根管理一组相关的实体，确保业务一致性
  - User: 用户管理
  - Post: 文章/内容管理
  - Taxonomy: 分类体系
  - Tag: 标签管理
  - Role: 角色管理
  - Permission: 权限管理

- **投影 (Projection)**: 用于查询的领域模型，包括 inline、multi 等类型

- **公共组件**: 包括事件处理、快照机制、仓储接口等

### 应用层 (jimo-application)

应用层负责用例的编排和业务流程的实现：

- 接收来自适配器的请求
- 协调领域层完成业务逻辑
- 返回应用结果 DTO

### 适配器层 (jimo-adapter)

适配器层定义对外接口的抽象，用于解耦具体的实现技术。

### 基础设施层 (jimo-infrastructure)

基础设施层负责与外部系统交互，如数据库、缓存、第三方 API 等。

## 快速开始

### 环境要求

- Rust 1.75+
- Cargo

### 构建项目

```bash
# 编译项目
cargo build

# 运行项目
cargo run

# 运行测试
cargo test
```

### 添加依赖

在 `Cargo.toml` 的 `[workspace.dependencies]` 中添加共享依赖，或在各个 crate 的 `[dependencies]` 中添加特定依赖。

## 贡献指南

欢迎提交 Pull Request 或创建 Issue。

## 开源协议

MIT License
