- `pre-commit`: 提交时 hooks
配置文件 `.pre-commit-config.yaml`，并执行
```bash
pre-commit install
```
- `tokei`: 字符统计工具

```bash
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
```bash
docker exec -it 9693 psql -U postgres
## 切换到目标数据库
\c reservation
## 授予 rsvp schema 的所有权限
GRANT ALL PRIVILEGES ON SCHEMA rsvp TO shur;
## 授予创建数据库的权限(for test)
ALTER USER shur CREATEDB;
## 授予数据库的所有权限
GRANT ALL PRIVILEGES ON DATABASE reservation TO shur;
## 允许在 schema 中创建(including tables, functions, indices, etc.)
GRANT CREATE ON SCHEMA rsvp TO shur;
## 确保未来创建的表自动授予权限
ALTER DEFAULT PRIVILEGES IN SCHEMA rsvp GRANT ALL ON TABLES TO shur;
```


```bash
## 初始化迁移文件
sqlx migrate add init -r
## 运行迁移
sqlx migrate run ## 会生成一个名叫 _sqlx_migrations 的表记录迁移日志
## 回滚迁移
sqlx migrate revert ## sqlx migrate down 完全相同
```
- `pgcli`: `psql` 美化版
