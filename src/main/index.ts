import { app, BrowserWindow, Tray, Menu, globalShortcut, nativeImage } from 'electron';
import path from 'path';
import { fileURLToPath } from 'url';
import { registerIpcHandlers } from './ipc-handlers.js';
import { initRecordingModule, cleanupRecordingModule } from './modules/recording/index.js';
import { initGiftSoundModule, cleanupGiftSoundModule } from './modules/gift-sound/index.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

let mainWindow: BrowserWindow | null = null;
let tray: Tray | null = null;

function createTray() {
  // 创建 16x16 的简单图标
  const icon = nativeImage.createEmpty();
  tray = new Tray(icon);

  const contextMenu = Menu.buildFromTemplate([
    {
      label: '显示主窗口',
      click: () => {
        if (mainWindow) {
          mainWindow.show();
          mainWindow.focus();
        }
      }
    },
    {
      label: '隐藏主窗口',
      click: () => {
        if (mainWindow) {
          mainWindow.hide();
        }
      }
    },
    { type: 'separator' },
    {
      label: '退出',
      click: () => {
        app.quit();
      }
    }
  ]);

  tray.setToolTip('Bilibili 直播助手');
  tray.setContextMenu(contextMenu);

  // 双击托盘图标显示窗口
  tray.on('double-click', () => {
    if (mainWindow) {
      mainWindow.show();
      mainWindow.focus();
    }
  });
}

function registerGlobalShortcuts() {
  // Ctrl+Shift+D: 显示/隐藏窗口
  globalShortcut.register('CommandOrControl+Shift+D', () => {
    if (mainWindow) {
      if (mainWindow.isVisible()) {
        mainWindow.hide();
      } else {
        mainWindow.show();
        mainWindow.focus();
      }
    }
  });

  // F9: 标记高光时刻
  globalShortcut.register('F9', () => {
    if (mainWindow) {
      mainWindow.webContents.send('recording:markHighlightHotkey');
    }
  });
}

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      preload: path.join(__dirname, '../preload/index.js'),
      contextIsolation: true,
      nodeIntegration: false
    },
    title: 'Bilibili 直播助手'
  });

  mainWindow.loadFile(path.join(__dirname, '../renderer/index.html'));

  if (process.env.NODE_ENV === 'development') {
    mainWindow.webContents.openDevTools();
  }

  // 点击关闭按钮时不退出，只隐藏到托盘
  mainWindow.on('close', (event) => {
    if (!isQuitting) {
      event.preventDefault();
      mainWindow?.hide();
    }
  });
}

app.whenReady().then(async () => {
  // 注册所有 IPC 处理器（包括数据库初始化）
  await registerIpcHandlers();

  createWindow();
  createTray();
  registerGlobalShortcuts();

  // 初始化录制模块
  if (mainWindow) {
    initRecordingModule(mainWindow);
  }

  // 初始化礼物音效模块
  if (mainWindow) {
    initGiftSoundModule(mainWindow);
  }

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    } else if (mainWindow) {
      mainWindow.show();
    }
  });
});

// 退出标志
let isQuitting = false;
app.on('before-quit', () => {
  isQuitting = true;
  // 清理录制模块
  cleanupRecordingModule();
  // 清理礼物音效模块
  cleanupGiftSoundModule();
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('will-quit', () => {
  // 注销所有全局快捷键
  globalShortcut.unregisterAll();
});

// 通知渲染进程断开连接
app.on('before-quit', () => {
  if (mainWindow && !mainWindow.isDestroyed()) {
    mainWindow.webContents.send('app-before-quit');
  }
});
