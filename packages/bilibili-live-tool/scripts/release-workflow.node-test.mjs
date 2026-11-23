import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

const workflowUrl = new URL('../../../.github/workflows/release-tauri.yml', import.meta.url);

/** Windows checkouts carry CRLF; the contracts below match on LF. */
async function readWorkflow() {
  return (await readFile(workflowUrl, 'utf8')).replace(/\r\n/g, '\n');
}

test('release build matrix contains only Windows x64 and both macOS architectures', async () => {
  const workflow = await readWorkflow();
  const matrix = workflow.match(/\n  build:\n[\s\S]*?\n    runs-on:/)?.[0];

  assert.ok(matrix, 'release build matrix is present');
  assert.equal(matrix.match(/\n          - platform:/g)?.length, 3);
  assert.match(matrix, /platform: windows-latest[\s\S]*?args: --bundles nsis/);
  assert.match(
    matrix,
    /platform: macos-latest[\s\S]*?--target aarch64-apple-darwin --bundles app,dmg/
  );
  assert.match(
    matrix,
    /platform: macos-15-intel[\s\S]*?--target x86_64-apple-darwin --bundles app,dmg/
  );
  assert.doesNotMatch(matrix, /platform: (?:ubuntu|windows-11-arm)/);
  assert.match(matrix, /platform: windows-latest[\s\S]*?asset_platform: Windows_x64-setup/);
  assert.match(matrix, /platform: macos-latest[\s\S]*?asset_platform: macOS_AppleSilicon/);
  assert.match(matrix, /platform: macos-15-intel[\s\S]*?asset_platform: macOS_Intel/);
});

test('release workflow publishes only the free channel', async () => {
  const workflow = await readWorkflow();
  const matrix = workflow.match(/\n  build:\n[\s\S]*?\n    runs-on:/)?.[0];

  assert.ok(matrix, 'release build matrix is present');
  assert.match(matrix, /channel: \[free\]/);
  assert.doesNotMatch(matrix, /channel: \[[^\]]*member/);
  assert.doesNotMatch(workflow, /for CHANNEL in free member/);
  assert.doesNotMatch(workflow, /gh release edit "member-v\$VERSION"/);
  assert.match(
    workflow,
    /releaseAssetNamePattern: liver_\[version\]_\$\{\{ matrix\.target\.asset_platform \}\}\[ext\]/
  );
  assert.match(workflow, /generate-release-notes\.mjs/);
  assert.match(workflow, /--changes-file "\$APP_PATH\/release-notes\/\$VERSION\.md"/);
});
