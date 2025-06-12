- `pre-commit`: 提交时 hooks
配置文件 `.pre-commit-config.yaml`，并执行
```zsh
pre-commit install
```
- `tokei`: 字符统计工具

```zsh
tokei  # 统计当前目录代码
tokei ~/projects/my_app # 统计指定目录代码
tokei --language=Rust,Python # 指定语言
tokei --exclude="*.min.js" --exclude-dir="node_modules" # 排除文件或目录
tokei --output json # 输出格式(json, yaml, csv...)
```
- `protobuf`: `protoc` gRPC代码生成工具，并非 rust 工具。在 `build.rs` 中使用 `tonic-build` 生成，会调用 `protoc`
- `tonic-build`: 构建工具
- `sqlx-cli`
执行前，使用超级用户 `postgres` 赋予权限
```zsh
docker exec -it 9693 psql -U postgres
## 切换到目标数据库
\c reservation
## 授予 public schema 的所有权限
GRANT ALL PRIVILEGES ON SCHEMA public TO shur;
## 允许创建表
GRANT CREATE ON SCHEMA public TO shur;
## 确保未来创建的表自动授予权限
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO shur;
```


```zsh
## 初始化迁移文件
sqlx migrate add init -r
## 运行迁移
sqlx migrate run ## 会生成一个名叫 _sqlx_migrations 的表记录迁移日志
## 回滚迁移
sqlx migrate revert ## sqlx migrate down 完全相同
```
- `pgcli`: `psql` 美化版
