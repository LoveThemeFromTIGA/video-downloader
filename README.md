# Video Downloader

## 简介

![index](./docs/imgs/index.png)

**基于[rust](https://www.rust-lang.org/) + [tauri](https://github.com/tauri-apps/tauri) 开发的一款跨平台的多线程视频下载器。**

支持平台:
- windows
- mac
- linux

技术栈:
- [rust](https://www.rust-lang.org/)
- [tokio](https://tokio.rs/)
- [tauri](https://tauri.app/) 
- [typescript](https://www.typescriptlang.org/) 
- [vue](https://vuejs.org/)
- [Element Plus](https://element-plus.org/zh-CN/)


多线程下载实现:

- 通过异步运行时tokio开启多个协程, 分别下载视频文件的分片(http range)。




## 功能

### 抖音

#### 单个视频下载

- 通过解析抖音分享的视频链接下载无水印视频。例如分享链接:  https://v.douyin.com/jpLRaaf/

![单视频下载](./docs/imgs/douyin_single_download.png)

#### 用户主页视频批量下载

- 通过用户主页链接搜索用户视频进行下载。 例如:  https://v.douyin.com/j3XPKMg/, https://www.douyin.com/user/MS4wLjABAAAAWiOs23d6NtmiUg98zONd6wQhmPsy1WLwZn0jEaCbDL8

![用户主页视频下载](./docs/imgs/douyin_muplit_download.png)

### B站

- 暂未完成