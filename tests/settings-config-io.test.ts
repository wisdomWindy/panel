import { strict as assert } from 'node:assert';
import test from 'node:test';

import {
  buildSettingsExportConfig,
  createSettingsImportPatches
} from '../src/renderer/views/settings/utils/config-io.ts';

test('buildSettingsExportConfig groups core and feature configs', () => {
  const exported = buildSettingsExportConfig({
    core: { language: 'en-US', enableAutoSend: true },
    recording: { format: 'mp4', outputPath: '/tmp/output' },
    tts: { enabled: true, engine: 'edge-tts' },
    asr: { autoStart: true, engine: 'whisper-cpp' },
    ai: { enabled: true, provider: 'openai' },
    moderation: { enabled: true, autoMuteEnabled: true }
  });

  assert.equal(exported.core.language, 'en-US');
  assert.equal(exported.recording.outputPath, '/tmp/output');
  assert.equal(exported.tts.enabled, true);
  assert.equal(exported.asr.autoStart, true);
  assert.equal(exported.ai.enabled, true);
  assert.equal(exported.moderation.enabled, true);
});

test('createSettingsImportPatches only returns provided domains', () => {
  const patches = createSettingsImportPatches({
    core: { language: 'en-US' },
    ai: { enabled: true }
  });

  assert.deepEqual(patches.core, { language: 'en-US' });
  assert.deepEqual(patches.ai, { enabled: true });
  assert.equal('recording' in patches, false);
  assert.equal('tts' in patches, false);
});
