# 下载

## 选择您的平台

<div class="my-8 grid grid-cols-[repeat(auto-fit,minmax(250px,1fr))] gap-6">
  <div class="download-card windows">
    <div class="mb-4"><el-icon :size="16"><Monitor /></el-icon></div>
    <h3 class="text-lg font-semibold text-[var(--vp-c-text-1)]">Windows</h3>
    <p class="mb-6 text-sm text-[var(--vp-c-text-2)]">支持 Windows 10 及以上版本</p>
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手-Setup.exe"
      class="download-btn windows-btn"
    >
      下载 exe 安装包
    </a>
    <span class="mt-3 block text-xs text-[var(--vp-c-text-3)]">约 80 MB</span>
  </div>

  <div class="download-card macos">
    <div class="mb-4"><el-icon :size="16"><Platform /></el-icon></div>
    <h3 class="text-lg font-semibold text-[var(--vp-c-text-1)]">macOS</h3>
    <p class="mb-6 text-sm text-[var(--vp-c-text-2)]">支持 macOS 10.15 及以上版本</p>
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.dmg"
      class="download-btn macos-btn"
    >
      下载 dmg 安装包
    </a>
    <span class="mt-3 block text-xs text-[var(--vp-c-text-3)]">约 90 MB</span>
  </div>

  <div class="download-card linux">
    <div class="mb-4"><el-icon :size="16"><Terminal /></el-icon></div>
    <h3 class="text-lg font-semibold text-[var(--vp-c-text-1)]">Linux</h3>
    <p class="mb-6 text-sm text-[var(--vp-c-text-2)]">支持 Ubuntu 20.04 及以上版本</p>
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.AppImage"
      class="download-btn linux-btn"
    >
      下载 AppImage
    </a>
    <span class="mt-3 block text-xs text-[var(--vp-c-text-3)]">约 85 MB</span>
  </div>
</div>

## 安装说明

### Windows

1. 下载 `Bilibili直播助手-Setup.exe` 安装包
2. 双击运行安装程序
3. 按照提示完成安装
4. 从开始菜单或桌面快捷方式启动应用

### macOS

1. 下载 `Bilibili直播助手.dmg` 安装包
2. 双击打开 DMG 文件
3. 将应用拖动到「应用程序」文件夹
4. 从启动台或应用程序文件夹启动应用

::: tip macOS 安全提示
首次打开应用时，可能会提示「无法打开，因为无法验证开发者」。请前往「系统偏好设置」->「安全性与隐私」->「通用」，点击「仍要打开」。
:::

### Linux

1. 下载 `Bilibili直播助手.AppImage` 文件
2. 添加执行权限：`chmod +x Bilibili直播助手.AppImage`
3. 双击运行或在终端中执行：`./Bilibili直播助手.AppImage`

## 系统要求

| 平台 | 最低版本 | 推荐配置 |
|------|----------|----------|
| Windows | Windows 10 | 4GB RAM, 2GB 可用磁盘空间 |
| macOS | macOS 10.15 | 4GB RAM, 2GB 可用磁盘空间 |
| Linux | Ubuntu 20.04 | 4GB RAM, 2GB 可用磁盘空间 |

## 其他版本

访问 [GitHub Releases](https://github.com/wisdomWindy/panel/releases) 页面查看所有版本。

## 从源码构建

如果您想从源码构建，请参考 [安装配置](/guide/installation) 指南。
