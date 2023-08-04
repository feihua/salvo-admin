#!/bin/bash

cargo build -r

#停止服务
docker stop salvo_admin


#删除容器
docker rm -f salvo_admin

#删除镜像
docker rmi -f salvo_admin:v1

#删除none镜像
docker rmi -f $(docker images | grep "none" | awk '{print $3}')

#构建服务
docker build -t salvo_admin:v1 -f Dockerfile .

#启动服务
docker run -itd --net=host --name=salvo_admin salvo_admin:v1
