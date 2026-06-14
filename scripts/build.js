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

// Build the main process
async function buildMain() {
  console.log('Building main process...');
  await runCommand('npx', ['tsc', 'src/main/index.ts', 'src/main/ipc-handlers.ts', 'src/main/database.ts', 'src/main/bilibili-api.ts', '--outDir', 'dist/main', '--module', 'CommonJS', '--target', 'ES2020', '--esModuleInterop', 'true', '--skipLibCheck', 'true', '--moduleResolution', 'node']);
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