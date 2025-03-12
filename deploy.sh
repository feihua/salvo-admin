#!/bin/bash
 script
# 该函数用于构建并部署一个名为 `salvo-admin` 的 Docker 容器。
# 它首先使用 Cargo 构建项目，然后停止并删除现有的容器和镜像，
# 最后构建新的 Docker 镜像并运行容器。

# 使用 Cargo 构建项目，`-r` 参数表示以 release 模式构建
cargo build -r

# 停止名为 `salvo-admin` 的 Docker 容器
docker stop salvo-admin

# 强制删除名为 `salvo-admin` 的 Docker 容器
docker rm -f salvo-admin

# 强制删除名为 `salvo-admin:v1` 的 Docker 镜像
docker rmi -f salvo-admin:v1

# 删除所有标记为 `<none>` 的 Docker 镜像
docker rmi -f $(docker images | grep "none" | awk '{print $3}')

# 使用当前目录下的 `Dockerfile` 构建名为 `salvo-admin:v1` 的 Docker 镜像
docker build -t salvo-admin:v1 -f Dockerfile .

# 以交互式后台模式运行 `salvo-admin:v1` 镜像，容器命名为 `salvo-admin`，并使用主机网络模式
docker run -itd --net=host --name=salvo-admin salvo-admin:v1

