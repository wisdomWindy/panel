import assert from 'node:assert/strict';
import test from 'node:test';

import { buildReleaseNotes, installerAssetName } from './generate-release-notes.mjs';

test('installer names follow the product version platform extension contract', () => {
  assert.equal(
    installerAssetName('0.2.0', 'Windows_x64-setup', '.exe'),
    'biliLiverHelper_0.2.0_Windows_x64-setup.exe'
  );
  assert.equal(
    installerAssetName('v0.2.0', 'macOS_AppleSilicon', '.dmg'),
    'biliLiverHelper_0.2.0_macOS_AppleSilicon.dmg'
  );
  assert.throws(() => installerAssetName('0.2.0', 'linux-x64', '.AppImage'), /Unsupported/);
});

test('release notes contain direct downloads for every supported platform', () => {
  const notes = buildReleaseNotes(
    '0.2.0',
    'free-v0.2.0',
    'Changes:\n\n- Removes the plugin center.'
  );

  assert.match(notes, /^Changes:\n\n- Removes the plugin center\.\n\nDownloads:\n/m);
  assert.match(
    notes,
    /Windows x64: \[`biliLiverHelper_0\.2\.0_Windows_x64-setup\.exe`\]\(https:\/\/github\.com\/wisdomWindy\/panel\/releases\/download\/free-v0\.2\.0\/biliLiverHelper_0\.2\.0_Windows_x64-setup\.exe\)/
  );
  assert.match(notes, /biliLiverHelper_0\.2\.0_macOS_Intel\.dmg/);
  assert.match(notes, /biliLiverHelper_0\.2\.0_macOS_AppleSilicon\.dmg/);
  assert.equal(notes.match(/^- (?:Windows|macOS)/gm)?.length, 3);
  assert.match(notes, /not Authenticode-signed/);
  assert.match(notes, /not Apple Developer signed or notarized/);
  assert.match(notes, /FFmpeg licensing and source information/);
});

test('release notes reject missing change details', () => {
  assert.throws(() => buildReleaseNotes('0.2.0', 'free-v0.2.0', ''), /non-empty Changes/);
});
