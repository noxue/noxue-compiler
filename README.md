```
docker rm $(docker ps -a -q)  删除所有容器


docker pull gcc
docker pull rust
docker pull php:5.6
docker pull php:7.4
docker pull php:8
docker pull golang
docker pull python:2
docker pull python:3
docker pull adityai/jdk13
docker pull ruby
```

### bug

* 修复换行符转义bug

之前使用一次性执行命令的方式，处理 \n 有转义问题，且有可能被拼接命令影响主机安全，现在改成所有输入都通过标准输入文件传入到容器，杜绝任何命令在主机执行的危险。

