# AdySec CF拉平镜像站
<a href="https://github.com/adysec/mirror/stargazers"><img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/adysec/mirror?color=yellow&logo=riseup&logoColor=yellow&style=flat-square"></a>
<a href="https://github.com/adysec/mirror/network/members"><img alt="GitHub forks" src="https://img.shields.io/github/forks/adysec/mirror?color=orange&style=flat-square"></a>
<a href="https://github.com/adysec/mirror/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues/adysec/mirror?color=red&style=flat-square"></a>

站点地址：<https://mirror.adysec.com/>

项目地址：<https://github.com/adysec/mirror>

官方源可信度和稳定性最高，但众所周知的原因国内访问速度较慢，因此产生了清华源、中科大源、阿里源之类的一系列开源镜像站，有些资源需要换n个源才能把系统和软件更新完成，第三方源相对官方源软件版本更低，维护者也可能会夹带私货，洁癖患者难以忍受。

**由于docker hub新增流控策略，可能存在`permission denied`、`需要登录`等情况，现改为rust代理程序方案，仍使用cloudflare cdn**

本站使用Cloudflare Workers Free Plan，免费计划慷慨地给了每日10w个请求，可承受每日1000+正常用户使用，带宽至少10Tbps。

|          | Requests        | Duration               | CPU time                                   |
| -------- | --------------- | ---------------------- | ------------------------------------------ |
| **Free** | 100,000 per day | No charge for duration | 10 milliseconds of CPU time per invocation |

本地自助测速地址：https://speed.cloudflare.com/

## 系统镜像

配置文档可能存在描述有误的情况，请参考官方文档修改替换镜像源地址

| 系统         | 配置文档                                      | 下载地址                                       | 同步来源                                     |
| ------------ | --------------------------------------------- | ---------------------------------------------- | -------------------------------------------- |
| ubuntu       | https://mirror.adysec.com/system/ubuntu       | https://mirrors.adysec.com/system/ubuntu       | http://archive.ubuntu.com/ubuntu   |
| centos       | https://mirror.adysec.com/system/centos       | https://mirrors.adysec.com/system/centos       | http://mirror.webhostingghana.com/centos     |
| epel         | https://mirror.adysec.com/system/epel         | https://mirrors.adysec.com/system/epel         | http://mirrors.kernel.org/fedora-epel        |
| deepin       | https://mirror.adysec.com/system/deepin       | https://mirrors.adysec.com/system/deepin       | https://community-packages.deepin.com/deepin |
| kali         | https://mirror.adysec.com/system/kali         | https://mirrors.adysec.com/system/kali         | http://http.kali.org/kali                    |
| debian       | https://mirror.adysec.com/system/debian       | https://mirrors.adysec.com/system/debian       | http://ftp.debian.org/debian                 |
| manjaro      | https://mirror.adysec.com/system/manjaro      | https://mirrors.adysec.com/system/manjaro      | http://ftp.tsukuba.wide.ad.jp/manjaro        |
| GNU          | https://mirror.adysec.com/system/gnu          | https://mirrors.adysec.com/system/gnu          | https://lists.gnu.org/archive/html           |
| openwrt      | https://mirror.adysec.com/system/openwrt      | https://mirrors.adysec.com/system/openwrt      | https://archive.openwrt.org                  |
| kaos         | https://mirror.adysec.com/system/KaOS         | https://mirrors.adysec.com/system/KaOS         | https://ca.kaosx.cf                          |
| arch4edu     | https://mirror.adysec.com/system/arch4edu     | https://mirrors.adysec.com/system/arch4edu     | https://arch4edu.org                         |
| archlinux    | https://mirror.adysec.com/system/archlinux    | https://mirrors.adysec.com/system/archlinux    | https://mirror.pkgbuild.com                  |
| bioarchlinux | https://mirror.adysec.com/system/bioarchlinux | https://mirrors.adysec.com/system/bioarchlinux | https://repo.bioarchlinux.org                |
| archlinuxcn  | https://mirror.adysec.com/system/archlinuxcn  | https://mirrors.adysec.com/system/archlinuxcn  | https://repo.archlinuxcn.org                 |
| archlinuxarm | https://mirror.adysec.com/system/archlinuxarm | https://mirrors.adysec.com/system/archlinuxarm | http://dk.mirror.archlinuxarm.org            |
| fedora       | https://mirror.adysec.com/system/fedora       | https://mirrors.adysec.com/system/fedora       | https://ap.edge.kernel.org/fedora            |
| OpenBSD      | https://mirror.adysec.com/system/openbsd      | https://mirrors.adysec.com/system/OpenBSD      | https://cdn.openbsd.org/pub/OpenBSD          |
| opensuse     | https://mirror.adysec.com/system/opensuse     | https://mirrors.adysec.com/system/opensuse     | http://download.opensuse.org                 |
| freebsd      | https://mirror.adysec.com/system/freebsd      | https://mirrors.adysec.com/system/freebsd      | https://download.freebsd.org                 |
| freedos      | https://mirror.adysec.com/system/freedos      | https://mirrors.adysec.com/system/freedos      | https://mirror.math.princeton.edu/pub/freeDOS|
| kylin        | https://mirror.adysec.com/system/kylin        | https://mirrors.adysec.com/system/kylin        | http://archive.kylinos.cn/kylin              |

## 编程语言

配置文档可能存在描述有误的情况，请参考官方文档修改替换镜像源地址

| 语言 | 配置文档                                | 下载地址                                 | 同步来源                |
| ---- | --------------------------------------- | ---------------------------------------- | ----------------------- |
| pypi | https://mirror.adysec.com/language/pypi | https://mirrors.adysec.com/language/pypi | https://pypi.org/simple |
| rust | https://mirror.adysec.com/language/rust | https://mirrors.adysec.com/language/rust | https://static.rust-lang.org |
| npm  | https://mirror.adysec.com/language/npm  | https://mirrors.adysec.com/language/npm  | https://registry.npmjs.org   |

## 容器

配置文档可能存在描述有误的情况，请参考官方文档修改替换镜像源地址

| 容器       | 配置文档                                       | 下载地址                                       | 同步来源                     |
| ---------- | ---------------------------------------------- | ---------------------------------------------- | ---------------------------- |
| docker-ce  | https://mirror.adysec.com/container/docker-ce  | https://mirrors.adysec.com/container/docker-ce | https://download.docker.com/ |
| Docker Hub | https://mirror.adysec.com/container/docker-hub | https://docker.adysec.com                      | https://registry-1.docker.io |
| Quay       | https://mirror.adysec.com/container/docker-hub | https://quay.adysec.com                        | https://quay.io              |
| GCR        | https://mirror.adysec.com/container/docker-hub | https://gcr.adysec.com                         | https://gcr.io               |
| k8s GCR    | https://mirror.adysec.com/container/docker-hub | https://k8s-gcr.adysec.com                     | https://k8s.gcr.io           |
| k8s        | https://mirror.adysec.com/container/docker-hub | https://k8s.adysec.com                         | https://registry.k8s.io      |
| ghcr       | https://mirror.adysec.com/container/docker-hub | https://ghcr.adysec.com                        | https://ghcr.io              |
| Cloudsmith | https://mirror.adysec.com/container/docker-hub | https://cloudsmith.adysec.com  
