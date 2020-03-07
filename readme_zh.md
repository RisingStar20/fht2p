# fht2p
[![Actions Status](https://github.com/biluohc/fht2p/workflows/CI/badge.svg)](https://github.com/biluohc/fht2p/actions)

fht2p 是使用 Rust 开发的，跨平台的 HTTP 静态文件服务器，项目 CI 会测试 Linux, MacOS 和 Windows 三大平台。

## 特点
- 可靠： 使用 Rust 实现，集成测试，安全可靠
- 通用： 完全跨平台，Unix-like 和 Windows 系统均可用
- 方便： 静态链接，不依赖 openssl 等外部动态库，下载即可用
- 极速： 异步多线程，基于 tokio 和 hyper 打造，极速响应，海量并发
- 功能： 文件下载上传，目录浏览，断点续传，代理功能，丰富选项，应有尽有

### 功能
1. 多路径分享
1. 文件断点续传
1. 可关闭的 目录浏览（以及排序等功能）
1. 缓存功能
1. 文件上传，目录新建
1. HTTPS（tokio-rustls, 不依赖外部动态库）
1. Proxy（隧道代理, 普通代理)
1. 基本认证（Basic Authentication）
1. 跨域资源共享（CORS）
1. 目录页面压缩（GZIP)
1. 命令行参数
1. 配置文件(格式是json5，类似json但支持注释等)
1. 终端日志可选级别
1. 启动时输出服务的网址，可选输出二维码

### 截图

![snapshot.png](https://raw.githubusercontent.com/biluohc/fht2p/master/config/assets/snapshot.png)

### 安装
```sh
    cargo install --git https://github.com/biluohc/fht2p fht2p -f

    # running fht2p --help(-h) to get help.

    fht2p -h
```
#### 或者
```sh
    git clone https://github.com/biluohc/fht2p
    # cargo  install --path fht2p/ fht2p -f

    cd fht2p
    cargo build --release

    ./target/release/fht2p --help
```

### 查看帮助信息，配置文件可以参考项目下的 config 目录
```sh
fht2p 0.9.3 (8d85ce30-modified@master rustc1.41.1 2020-03-04~15:35:48UTC)
Wspsxing <biluohc@qq.com>
A HTTP Server for Static File written with Rust

USAGE:
    fht2p [FLAGS] [OPTIONS] [PATH]...

FLAGS:
    -F, --config-print     Print the content of default config file
    -d, --disable-index    Disable index(directory) view(will return 403)
    -f, --follow-links     Enable follow links
    -h, --help             Prints help information
    -k, --keepalive        Close HTTP keep alive
    -m, --mkdir            Enable mkdir function
    -Q, --qr-code          Show URL's QR code at startup
    -r, --redirect-html    Redirect dir to `index.html` or `index.htm` if it exists
    -s, --show-hider       Show entries starts with '.'
    -u, --upload           Enable upload function
    -V, --version          Prints version information
    -v, --verbose          Increases logging verbosity each use(warn0_info1_debug2_trace3+)

OPTIONS:
    -a, --auth <auth>                  Set the username:password for authorization
    -S, --cache-secs <cache-secs>      Set the secs of cache(use 0 to close) [default: 60]
    -C, --cert <cert>                  Set the cert for https,  public_cert_file:private_key_file
        --compress <compress>          Set the level for index compress, should between 0~9, use 0 to close [default: 5]
    -c, --config <config>              Set the path to use a custom config file
                                       default path: ~/.config/fht2p/fht2p.json
    -i, --ip <ip>                      Set listenning ip address [default: 127.0.0.1]
    -M, --magic-limit <magic-limit>    The size limit for detect file ContenType(use 0 to close) [default: 10485760]
    -p, --port <port>                  Set listenning port [default: 8000]
    -P, --proxy <proxy>                Enable http proxy(Regular for allowed domains, empty string can allow all)

ARGS:
    <PATH>...    Set the paths to share [Default: "."]
```

### 注意事项
1. 关于选项和配置文件的优先级

    默认配置文件位于 `$HOME/.config/fht2p/fht2p.json`, 如果没有，可以新建

    选项总体可以分为四种:
- 第一种是 --help， --version 和 --config-print 程序会很快退出不需要考虑优先级，
- 第二种是 --verbose 和 --qr-code 这种优先级忽略的，和其它选项无冲突
- 第三种是 --config 指定配置文件，就会忽略第四种选项
- 第四种是 其他选项和参数，一旦有了就会忽略默认的配置文件（之所以这样是为了防止优先级太复杂）

2. 关于安全和 HTTPS

- HTTP是基于TCP的明文协议，完全没有安全可言，如果需要安全性，一定要使用 HTTPS
- 为了安全，默认监听的是本机回环地址（`127.0.0.1`）, 本机外要访问可监听 `0.0.0.0`或 特定的地址并注意防火墙
- 程序默认监听当前目录，请不要分享家目录或者根目录到网络上，除非你明白在干什么