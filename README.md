## 背景
使用Rust语言编写的网关后台API

## 快速开始
```
cargo run
```

## 接口
```shell
# 列表
localhost:7776/rgroutes/namespace/<namespace>

# 单条
localhost:7776/rgroutes/namespace/<namespace>/name/<name>

# 创建
localhost:7776/rgroutes

# 删除
localhost:7776/rgroutes/namespace/<namespace>

# 更新
localhost:7776/rgroutes/namespace/<namespace>
```

## 技术栈
- rustc 1.66.0-nightly (bf15a9e52 2022-10-14)
- VsCode
- Rocket