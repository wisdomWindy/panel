import { contextBridge, ipcRenderer } from 'electron';

contextBridge.exposeInMainWorld('electronAPI', {
  // IPC 通信
  invoke: (channel: string, ...args: unknown[]) => {
    return ipcRenderer.invoke(channel, ...args);
  },

  on: (channel: string, callback: (...args: unknown[]) => void) => {
    ipcRenderer.on(channel, (_event, ...args) => callback(...args));
  },

  off: (channel: string, callback: (...args: unknown[]) => void) => {
    ipcRenderer.removeListener(channel, callback);
  },

  // 文件系统操作
  checkDirectoryExists: (path: string) => {
    return ipcRenderer.invoke('fs:checkDirectoryExists', path);
  },

  checkFileExists: (path: string) => {
    return ipcRenderer.invoke('fs:checkFileExists', path);
  },

  readFile: (path: string) => {
    return ipcRenderer.invoke('fs:readFile', path);
  },

  getDirectories: (path: string) => {
    return ipcRenderer.invoke('fs:getDirectories', path);
  },

  copyDirectory: (source: string, target: string) => {
    return ipcRenderer.invoke('fs:copyDirectory', source, target);
  },

  deleteDirectory: (path: string) => {
    return ipcRenderer.invoke('fs:deleteDirectory', path);
  }
});

// 类型声明
declare global {
  interface Window {
    electronAPI: {
      invoke: (channel: string, ...args: unknown[]) => Promise<unknown>;
      on: (channel: string, callback: (...args: unknown[]) => void) => void;
      off: (channel: string, callback: (...args: unknown[]) => void) => void;
      checkDirectoryExists: (path: string) => Promise<boolean>;
      checkFileExists: (path: string) => Promise<boolean>;
      readFile: (path: string) => Promise<string>;
      getDirectories: (path: string) => Promise<string[]>;
      copyDirectory: (source: string, target: string) => Promise<void>;
      deleteDirectory: (path: string) => Promise<void>;
    };
  }
}
