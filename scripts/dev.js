import { spawn } from 'child_process';

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

// Build the main process for development
async function buildMain() {
  console.log('Building main process for development...');
  await runCommand('npx', ['tsc', 'src/main/index.ts', '--outDir', 'dist/main', '--module', 'nodenext', '--target', 'ES2020', '--esModuleInterop', 'true', '--skipLibCheck', 'true']);
}

// Build the preload script for development
async function buildPreload() {
  console.log('Building preload script for development...');
  await runCommand('npx', ['tsc', 'src/preload/index.ts', '--outDir', 'dist/preload', '--module', 'CommonJS', '--target', 'ES2020', '--esModuleInterop', 'true', '--skipLibCheck', 'true']);
}

// Start the renderer in development mode
const renderer = spawn('npx', ['vite', '--config', './vite.config.ts', '--host'], {
  stdio: 'inherit',
  shell: true
});

renderer.on('close', (code) => {
  console.log(`Renderer process exited with code ${code}`);
  process.exit(code);
});

// Handle process termination
process.on('SIGINT', () => {
  renderer.kill('SIGINT');
  process.exit();
});

// Build main and preload processes
buildMain().then(() => {
  buildPreload();
}).catch(error => {
  console.error('Build failed:', error);
  process.exit(1);
});

// Wait for a moment, then start Electron
setTimeout(() => {
  const electron = spawn('npx', ['electron', 'dist/main/index.js'], {
    stdio: 'inherit',
    shell: true
  });

  electron.on('close', (code) => {
    console.log(`Electron process exited with code ${code}`);
    process.exit(code);
  });

  electron.on('error', (error) => {
    console.error('Failed to start Electron:', error);
    process.exit(1);
  });
}, 1000);