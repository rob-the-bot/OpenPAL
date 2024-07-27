# 仙剑开源版

![Windows](https://img.shields.io/github/actions/workflow/status/rob-the-bot/OpenPAL/ci-windows.yml?branch=master&style=flat-square&label=Windows&logo=windows)
![Linux](https://img.shields.io/github/actions/workflow/status/rob-the-bot/OpenPAL/ci-linux.yml?branch=master&style=flat-square&label=Linux&logo=linux)
![macOS](https://img.shields.io/github/actions/workflow/status/rob-the-bot/OpenPAL/ci-macos.yml?branch=master&style=flat-square&label=macOS&logo=apple)
![Android](https://img.shields.io/github/actions/workflow/status/rob-the-bot/OpenPAL/ci-android.yml?branch=master&style=flat-square&label=Android&logo=android)
![PSVita](https://img.shields.io/github/actions/workflow/status/rob-the-bot/OpenPAL/ci-psvita.yml?branch=master&style=flat-square&label=PlayStation%20Vita&logo=playstation)


> 云对雨，雪对风，仙剑对妖弓。

《仙剑奇侠传三》和《仙剑奇侠传四》开源实现。
仙剑三剧情开发已完成。
仙剑四开源版仍处于早期开发阶段，存在很多未实现的功能及 Bug，暂无完善的游戏体验。

## 📢 声明

- 开源版不附带任何游戏数据，因此您必须拥有《仙剑奇侠传三》的[正版拷贝](https://store.steampowered.com/app/1536070)或者《仙剑奇侠传三》的[正版拷贝](https://store.steampowered.com/app/1536070)才可以正常运行。
- 仙剑开源版并非软星公司或大宇集团的官方作品。

## 🏡 社区

欢迎加入 QQ 群 636662894

## 运行

**首次运行前请手动修改 `openpal3.toml`，将《仙剑奇侠传三》游戏目录填入：**
**（请注意反斜杠需要重复两次）**

```
# PAL3.exe 所在的目录
# The folder where PAL3.exe is
asset_path = "E:\\CubeLibrary\\apps\\1000039"
```

之后运行 `openpal3.exe` 即可。如果运行时提示 OpenAL 出错，[请下载并安装 OpenAL](http://www.openal.org/downloads/oalinst.zip)。

### 操作

- 空格键：对话框下一句
- A/D键：调整视角
- Esc键：跳过过场动画
- F键：互动
- 方向键：跑
- 1/2/3/4：存档至第1、2、3、4号存档位

## Building

目前OpenPAL支持 Windows, Linux, PS Vita, macOS and Android作为目标平台。
See the build files under [workflows](.github/workflows).

### Toolchain and dependencies

- [Rust](https://www.rust-lang.org/) **nightly** toolchain
- [OpenAL](https://www.openal.org)
- [Latest Vulkan SDK](https://www.lunarg.com/vulkan-sdk/)
- LLVM + clang
  - [on Windows](https://releases.llvm.org/download.html)
  - on Linux please install `libclang-dev` and `clang`
- cmake
  - [on Windows](https://cmake.org/download/)
- vcpkg on Windows


### 构建步骤


```bash
cd openpal3
cargo build --release
```

构建 Android 平台安装包需要先安装`cargo-apk`，并设置好 NDK 开发环境

```bash
cd openpal3 && cargo apk build --release --lib
```

For building instructions

### 常见问题

#### MacOS

由于 nightly 工具链的一个[bug](https://github.com/rust-lang/rust/issues/91372),构建时可用下面命令规避链接错误的问题：

```bash
MACOSX_DEPLOYMENT_TARGET=11.0 cargo build --release
```

#### PS Vita

Requires VitaSDK, see [ci-psvita.yml](.github/workflows/ci-psvita.yml).

## Contributing

非常感谢一同参与 OpenPAL3 的开发！请参看 [CONTRIBUTING](CONTRIBUTING.md) 来了解参与项目的要求与步骤。

## Miscellaneous

- [@zhangboyang/PAL3patch](https://github.com/zhangboyang/PAL3patch) 提供了简单有效的脱壳工具
- [仙剑三高难度吧](https://tieba.baidu.com/f?kw=%E4%BB%99%E5%89%913%E9%AB%98%E9%9A%BE%E5%BA%A6) 有一些关于仙剑三数据文件的目录结构与内容的讨论
- [这个转帖](https://tieba.baidu.com/p/5381666939?red_tag=0041464978) 分析了 `pol` 文件的部分结构
- [看雪论坛的这篇帖子](https://bbs.pediy.com/thread-157228.htm) 分析了 `cpk` 与 `sce` 的文件格式
