import { spawn, exec } from 'child_process';
import { existsSync, rmSync } from 'fs';
import { join } from 'path';

// Function to run a command and return a promise
function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const process = spawn(command, args, { stdio: 'inherit', shell: true });

    process.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Command failed with exit code ${code}`));
      }
    });

    process.on('error', (error) => {
      reject(error);
    });
  });
}

// Kill process by name
function killProcess(name) {
  return new Promise((resolve) => {
    const isWin = process.platform === 'win32';
    const cmd = isWin ? `taskkill /F /IM ${name}` : `pkill -f ${name}`;
    
    exec(cmd, (error) => {
      // Ignore errors if process not found
      resolve();
    });
  });
}

// Clean release directory
async function cleanReleaseDir() {
  const releaseDir = join(process.cwd(), 'release');
  
  if (existsSync(releaseDir)) {
    console.log('Cleaning release directory...');
    
    // Try to kill running processes
    await killProcess('start.exe');
    await killProcess('start');
    await killProcess('Bilibili 直播助手');
    
    // Wait a bit for processes to terminate
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    try {
      rmSync(releaseDir, { recursive: true, force: true });
      console.log('Release directory cleaned.');
    } catch (error) {
      console.warn('Warning: Could not fully clean release directory:', error.message);
    }
  }
}

// Build the main process
async function buildMain() {
  console.log('Building main process...');
  await runCommand('npx', ['tsc', 'src/main/index.ts', 'src/main/ipc-handlers.ts', 'src/main/database.ts', 'src/main/bilibili-api.ts', '--outDir', 'dist/main', '--module', 'nodenext', '--target', 'ES2020', '--esModuleInterop', 'true', '--skipLibCheck', 'true', '--moduleResolution', 'nodenext']);
}

// Build the preload script
async function buildPreload() {
  console.log('Building preload script...');
  await runCommand('npx', ['tsc', 'src/preload/index.ts', '--outDir', 'dist/preload', '--module', 'CommonJS', '--target', 'ES2020', '--esModuleInterop', 'true', '--skipLibCheck', 'true']);
}

// Build the renderer
async function buildRenderer() {
  console.log('Building renderer...');
  await runCommand('npx', ['vite', 'build', '--config', 'vite.config.ts']);
}

// Main build function
async function buildAll() {
  try {
    await cleanReleaseDir();
    await buildMain();
    await buildPreload();
    await buildRenderer();
    console.log('Build completed successfully!');
  } catch (error) {
    console.error('Build failed:', error);
    process.exit(1);
  }
}

buildAll();