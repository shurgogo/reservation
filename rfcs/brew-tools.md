- tokei: 字符统计工具
- protobuf: protoc gRPC代码生成工具
- tonic-build: 构建工具
- sqlx-cli
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
