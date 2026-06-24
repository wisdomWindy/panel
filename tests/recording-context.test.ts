import { strict as assert } from 'node:assert';
import test from 'node:test';

import {
  buildRecordingStartOptions,
  canStartRecording
} from '../src/renderer/views/recording/utils/recording-context.ts';

test('canStartRecording rejects missing room context', () => {
  const result = canStartRecording(null);

  assert.equal(result.ok, false);
  assert.match(result.message ?? '', /直播间/);
});

test('buildRecordingStartOptions uses current room context instead of default literal', () => {
  const options = buildRecordingStartOptions(
    {
      roomId: '12345',
      roomName: '测试直播间'
    },
    {
      format: 'mp4',
      quality: 'original',
      segmentSize: 0,
      segmentDuration: 0,
      outputPath: '/tmp/output'
    }
  );

  assert.equal(options.roomId, '12345');
  assert.equal(options.roomName, '测试直播间');
  assert.notEqual(options.roomId, 'default');
});
