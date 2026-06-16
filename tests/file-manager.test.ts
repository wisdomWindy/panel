let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

// ------- 文件工具函数测试 -------
// 注意：由于 file-manager 依赖 electron，这里只测试纯工具函数

console.log('=== 文件大小格式化 ===');
{
  // 复制格式化函数进行测试
  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + units[i];
  }
  
  eq(formatFileSize(0), '0 B', '0 bytes');
  eq(formatFileSize(1024), '1 KB', '1 KB');
  eq(formatFileSize(1024 * 1024), '1 MB', '1 MB');
  eq(formatFileSize(1024 * 1024 * 1024), '1 GB', '1 GB');
  eq(formatFileSize(1500), '1.46 KB', '1.46 KB');
  eq(formatFileSize(1024 * 1024 * 1.5), '1.5 MB', '1.5 MB');
}

console.log('=== 时长格式化 ===');
{
  function formatDuration(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  
  eq(formatDuration(0), '00:00:00', '0秒');
  eq(formatDuration(61), '00:01:01', '61秒');
  eq(formatDuration(3661), '01:01:01', '3661秒');
  eq(formatDuration(3600), '01:00:00', '1小时');
  eq(formatDuration(86400), '24:00:00', '24小时');
}

console.log('=== 文件路径生成 ===');
{
  function generateRecordingPath(roomId: string, format: string, outputPath?: string): string {
    const baseDir = outputPath || '/default/path/recordings';
    const now = new Date();
    const dateStr = now.toISOString().replace(/[:.]/g, '-').slice(0, 19);
    const fileName = `${roomId}_${dateStr}.${format}`;
    return `${baseDir}/${fileName}`;
  }
  
  const path1 = generateRecordingPath('12345', 'mp4');
  ok(path1.includes('12345'), '路径包含房间ID');
  ok(path1.endsWith('.mp4'), '路径以.mp4结尾');
  ok(path1.includes('/default/path/recordings'), '使用默认路径');
  
  const path2 = generateRecordingPath('67890', 'flv', '/custom/path');
  ok(path2.startsWith('/custom/path'), '使用自定义路径');
  ok(path2.includes('67890'), '路径包含房间ID');
  ok(path2.endsWith('.flv'), '路径以.flv结尾');
}

console.log('=== 高光片段路径生成 ===');
{
  function generateClipPath(recordingPath: string, highlightId: string): string {
    const lastSlash = recordingPath.lastIndexOf('/');
    const dir = recordingPath.substring(0, lastSlash);
    const fileName = recordingPath.substring(lastSlash + 1);
    const lastDot = fileName.lastIndexOf('.');
    const baseName = fileName.substring(0, lastDot);
    const ext = fileName.substring(lastDot);
    return `${dir}/${baseName}_highlight_${highlightId}${ext}`;
  }
  
  const clipPath = generateClipPath('/recordings/room1_2026-01-01.mp4', 'highlight123');
  ok(clipPath.includes('highlight_highlight123'), '路径包含高光ID');
  ok(clipPath.endsWith('.mp4'), '路径以.mp4结尾');
  ok(clipPath.startsWith('/recordings/'), '路径在同目录下');
}

console.log('=== 录制文件扩展名检查 ===');
{
  function isRecordingFile(fileName: string): boolean {
    const recordingExtensions = ['.mp4', '.flv', '.mkv', '.avi'];
    const lastDot = fileName.lastIndexOf('.');
    if (lastDot === -1) return false;
    const ext = fileName.substring(lastDot).toLowerCase();
    return recordingExtensions.includes(ext);
  }
  
  ok(isRecordingFile('test.mp4'), 'mp4是录制文件');
  ok(isRecordingFile('test.flv'), 'flv是录制文件');
  ok(isRecordingFile('test.mkv'), 'mkv是录制文件');
  ok(isRecordingFile('test.avi'), 'avi是录制文件');
  ok(!isRecordingFile('test.txt'), 'txt不是录制文件');
  ok(!isRecordingFile('test'), '无扩展名不是录制文件');
  ok(isRecordingFile('test.MP4'), '大写MP4是录制文件');
}

console.log('');
console.log(`--- ${p} passed, ${f} failed, ${p + f} total ---`);
