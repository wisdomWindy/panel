import assert from 'node:assert/strict';
import test from 'node:test';

import { buildReleaseNotes, installerAssetName } from './generate-release-notes.mjs';

test('installer names follow the product version platform extension contract', () => {
  assert.equal(
    installerAssetName('0.2.0', 'windows-x64', '.exe'),
    'biliLiverHelper_0.2.0_windows-x64.exe'
  );
  assert.equal(
    installerAssetName('v0.2.0', 'macos-arm64', '.dmg'),
    'biliLiverHelper_0.2.0_macos-arm64.dmg'
  );
  assert.throws(() => installerAssetName('0.2.0', 'linux-x64', '.AppImage'), /Unsupported/);
});

test('release notes contain direct downloads for every supported platform', () => {
  const notes = buildReleaseNotes('0.2.0', 'free-v0.2.0');

  assert.match(notes, /^biliLiverHelper v0\.2\.0\n\n## Downloads\n/m);
  assert.match(
    notes,
    /\[Windows x64\]\(https:\/\/github\.com\/wisdomWindy\/panel\/releases\/download\/free-v0\.2\.0\/biliLiverHelper_0\.2\.0_windows-x64\.exe\)/
  );
  assert.match(notes, /biliLiverHelper_0\.2\.0_macos-arm64\.dmg/);
  assert.match(notes, /biliLiverHelper_0\.2\.0_macos-x64\.dmg/);
  assert.equal(notes.match(/^- \[/gm)?.length, 3);
});
