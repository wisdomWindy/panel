# 下载

## 选择您的平台

<div class="download-cards">
  <div class="download-card">
    <div class="card-icon">🪟</div>
    <h3>Windows</h3>
    <p>支持 Windows 10 及以上版本</p>
    <a href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手-Setup.exe" class="download-btn windows">
      下载 exe 安装包
    </a>
    <span class="file-info">约 80 MB</span>
  </div>
  
  <div class="download-card">
    <div class="card-icon">🍎</div>
    <h3>macOS</h3>
    <p>支持 macOS 10.15 及以上版本</p>
    <a href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.dmg" class="download-btn macos">
      下载 dmg 安装包
    </a>
    <span class="file-info">约 90 MB</span>
  </div>
  
  <div class="download-card">
    <div class="card-icon">🐧</div>
    <h3>Linux</h3>
    <p>支持 Ubuntu 20.04 及以上版本</p>
    <a href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.AppImage" class="download-btn linux">
      下载 AppImage
    </a>
    <span class="file-info">约 85 MB</span>
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

<style>
.download-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin: 2rem 0;
}

.download-card {
  text-align: center;
  padding: 2rem;
  border-radius: 12px;
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  transition: transform 0.2s, box-shadow 0.2s;
}

.download-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.card-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.download-card h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.25rem;
}

.download-card p {
  color: var(--vp-c-text-2);
  font-size: 0.875rem;
  margin: 0 0 1.5rem 0;
}

.download-btn {
  display: inline-block;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  color: white;
  font-weight: 500;
  text-decoration: none;
  transition: transform 0.2s;
}

.download-btn:hover {
  transform: translateY(-2px);
}

.download-btn.windows {
  background: linear-gradient(135deg, #0078d4, #005a9e);
}

.download-btn.macos {
  background: linear-gradient(135deg, #333, #000);
}

.download-btn.linux {
  background: linear-gradient(135deg, #dd4814, #b33a10);
}

.file-info {
  display: block;
  margin-top: 0.75rem;
  font-size: 0.75rem;
  color: var(--vp-c-text-3);
}
</style>