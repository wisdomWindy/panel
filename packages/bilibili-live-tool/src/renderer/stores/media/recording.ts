import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DesktopError,
  type FfmpegSource,
  type HighlightId,
  type HighlightSummary,
  type PathGrantId,
  type PreviewSession,
  type RecordingConfig,
  type RecordingId,
  type RecordingStatus,
  type RecordingSummary,
  type RoomId
} from '@/platform/desktop';
import {
  adaptHighlightSummaries,
  adaptHighlightSummary,
  adaptRecordingConfig,
  adaptRecordingStatus,
  adaptRecordingSummaries,
  adaptRecordingSummary
} from '@/modules/media/adapters';
import { isUserCancellation } from '@/modules/common/error-text';

export const useMediaRecordingStore = defineStore('media-recording', () => {
  const config = ref<RecordingConfig | null>(null);
  const status = ref<RecordingStatus | null>(null);
  const recordings = ref<readonly RecordingSummary[]>([]);
  const highlights = ref<readonly HighlightSummary[]>([]);
  const pending = ref(false);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  let generation = 0;
  let loadPromise: Promise<void> | null = null;
  let temporaryDirectoryGrant: PathGrantId | null = null;
  const operations = new Map<string, Promise<unknown>>();

  function runOperation<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    const localGeneration = generation;
    error.value = null;
    pendingOperations.value = new Set([...pendingOperations.value, key]);
    const current = effect()
      .catch(value => {
        const mapped = normalizeDesktopError(value);
        // A dismissed dialog is not a failure worth an error banner.
        if (localGeneration === generation && !isUserCancellation(mapped.code)) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (operations.get(key) === current) operations.delete(key);
        if (localGeneration === generation) {
          const next = new Set(pendingOperations.value);
          next.delete(key);
          pendingOperations.value = next;
        }
      });
    operations.set(key, current);
    return current;
  }

  function read(force: boolean): Promise<void> {
    if (!force && loadPromise) return loadPromise;
    const localGeneration = ++generation;
    pending.value = true;
    error.value = null;
    const current = Promise.all([
      invokeDesktop('recording_get_config', undefined),
      invokeDesktop('recording_status', undefined),
      invokeDesktop('recording_list', undefined),
      invokeDesktop('highlight_list', undefined)
    ])
      .then(([configValue, statusValue, recordingValues, highlightValues]) => {
        const adapted = {
          config: adaptRecordingConfig(configValue),
          status: adaptRecordingStatus(statusValue),
          recordings: adaptRecordingSummaries(recordingValues),
          highlights: adaptHighlightSummaries(highlightValues)
        };
        if (localGeneration === generation) {
          config.value = adapted.config;
          status.value = adapted.status;
          recordings.value = adapted.recordings;
          highlights.value = adapted.highlights;
        }
      })
      .catch(value => {
        const mapped = normalizeDesktopError(value);
        // A dismissed dialog is not a failure worth an error banner.
        if (localGeneration === generation && !isUserCancellation(mapped.code)) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (loadPromise === current) loadPromise = null;
        if (localGeneration === generation) pending.value = false;
      });
    loadPromise = current;
    return current;
  }

  function load(): Promise<void> {
    return read(false);
  }

  function refresh(): Promise<void> {
    return read(true);
  }

  /**
   * Status-only refresh, used once per second while a recording runs so the
   * elapsed time, file size and remaining space stay live.
   */
  function refreshStatus(): Promise<void> {
    return runOperation('status', async () => {
      replaceStatus(await invokeDesktop('recording_status', undefined));
    });
  }

  function saveConfig(value: RecordingConfig): Promise<RecordingConfig> {
    return runOperation('saveConfig', async () => {
      const adapted = adaptRecordingConfig(await invokeDesktop('recording_save_config', { config: value }));
      config.value = adapted;
      if (adapted.directory?.grantId === temporaryDirectoryGrant) temporaryDirectoryGrant = null;
      return adapted;
    });
  }

  function selectDirectory(): Promise<NonNullable<RecordingConfig['directory']>> {
    return runOperation('selectDirectory', async () => {
      const grant = await invokeDesktop('path_select', { purpose: 'recordingDirectory' });
      const previous = temporaryDirectoryGrant;
      temporaryDirectoryGrant = grant.grantId;
      if (previous) await invokeDesktop('path_revoke', { grantId: previous }).catch(() => undefined);
      return grant;
    });
  }

  async function discardDirectorySelection(): Promise<void> {
    const grantId = temporaryDirectoryGrant;
    temporaryDirectoryGrant = null;
    if (grantId) await invokeDesktop('path_revoke', { grantId }).catch(() => undefined);
  }

  /** Installs FFmpeg into the app's own data directory (no PATH changes). */
  function installFfmpeg(): Promise<string> {
    return runOperation('installFfmpeg', async () => {
      const path = await invokeDesktop('ffmpeg_install', undefined);
      const adapted = adaptRecordingConfig(
        await invokeDesktop('recording_set_ffmpeg_source', {
          source: 'installed',
          grantId: null
        })
      );
      config.value = adapted;
      return path;
    });
  }

  function setFfmpegSource(source: FfmpegSource): Promise<RecordingConfig> {
    return runOperation('setFfmpegSource', async () => {
      let grantId: PathGrantId | null = null;
      if (source === 'local') {
        grantId = (await invokeDesktop('path_select', { purpose: 'localFfmpegExecutable' })).grantId;
      }
      try {
        const adapted = adaptRecordingConfig(
          await invokeDesktop('recording_set_ffmpeg_source', { source, grantId })
        );
        config.value = adapted;
        return adapted;
      } finally {
        if (grantId) await invokeDesktop('path_revoke', { grantId }).catch(() => undefined);
      }
    });
  }

  function commitStatus(key: string, effect: () => Promise<RecordingStatus>): Promise<RecordingStatus> {
    return runOperation(key, async () => {
      const adapted = adaptRecordingStatus(await effect());
      status.value = adapted;
      return adapted;
    });
  }

  function start(roomId: RoomId): Promise<RecordingStatus> {
    return commitStatus('start', () => invokeDesktop('recording_start', { roomId }));
  }

  function stop(): Promise<RecordingStatus> {
    return commitStatus('stop', () => invokeDesktop('recording_stop', undefined));
  }

  function markHighlight(): Promise<HighlightSummary> {
    return runOperation('markHighlight', async () => {
      const adapted = adaptHighlightSummary(await invokeDesktop('highlight_mark', undefined));
      highlights.value = Object.freeze([adapted, ...highlights.value.filter(item => item.id !== adapted.id)]);
      return adapted;
    });
  }

  function previewRecording(recordingId: RecordingId): Promise<PreviewSession> {
    return runOperation(`previewRecording:${recordingId}`, () => invokeDesktop('recording_preview', { recordingId }));
  }

  function previewHighlight(highlightId: HighlightId): Promise<PreviewSession> {
    return runOperation(`previewHighlight:${highlightId}`, () => invokeDesktop('highlight_preview', { highlightId }));
  }

  function renameRecording(recordingId: RecordingId, fileNameStem: string): Promise<RecordingSummary> {
    return runOperation(`renameRecording:${recordingId}`, async () => {
      const adapted = adaptRecordingSummary(
        await invokeDesktop('recording_rename', { recordingId, fileNameStem })
      );
      recordings.value = Object.freeze(recordings.value.map(item => item.id === adapted.id ? adapted : item));
      return adapted;
    });
  }

  function deleteRecording(recordingId: RecordingId): Promise<void> {
    return runOperation(`deleteRecording:${recordingId}`, async () => {
      await invokeDesktop('recording_delete', { recordingId });
      recordings.value = Object.freeze(recordings.value.filter(item => item.id !== recordingId));
    });
  }

  function exportHighlight(highlightId: HighlightId): Promise<string> {
    return runOperation(`exportHighlight:${highlightId}`, async () => {
      const grant = await invokeDesktop('path_select', { purpose: 'highlightExport' });
      try {
        const result = await invokeDesktop('highlight_export', { highlightId, grantId: grant.grantId });
        return result.displayPath;
      } finally {
        await invokeDesktop('path_revoke', { grantId: grant.grantId }).catch(() => undefined);
      }
    });
  }

  function deleteHighlight(highlightId: HighlightId): Promise<void> {
    return runOperation(`deleteHighlight:${highlightId}`, async () => {
      await invokeDesktop('highlight_delete', { highlightId });
      highlights.value = Object.freeze(highlights.value.filter(item => item.id !== highlightId));
    });
  }

  function replaceStatus(value: unknown): void {
    try {
      status.value = adaptRecordingStatus(value);
    } catch (value) {
      error.value = normalizeDesktopError(value);
    }
  }

  function acceptCompleted(value: unknown): void {
    try {
      const adapted = adaptRecordingSummary(value);
      recordings.value = Object.freeze([adapted, ...recordings.value.filter(item => item.id !== adapted.id)]);
    } catch (value) {
      error.value = normalizeDesktopError(value);
    }
  }

  function acceptHighlight(value: unknown): void {
    try {
      const adapted = adaptHighlightSummary(value);
      highlights.value = Object.freeze([adapted, ...highlights.value.filter(item => item.id !== adapted.id)]);
    } catch (value) {
      error.value = normalizeDesktopError(value);
    }
  }

  function dispose(): void {
    generation += 1;
    loadPromise = null;
    operations.clear();
    pending.value = false;
    pendingOperations.value = new Set();
    void discardDirectorySelection();
  }

  return {
    config,
    status,
    recordings,
    highlights,
    pending,
    pendingOperations,
    error,
    load,
    refresh,
    refreshStatus,
    saveConfig,
    selectDirectory,
    discardDirectorySelection,
    installFfmpeg,
    setFfmpegSource,
    start,
    stop,
    markHighlight,
    previewRecording,
    previewHighlight,
    renameRecording,
    deleteRecording,
    exportHighlight,
    deleteHighlight,
    replaceStatus,
    acceptCompleted,
    acceptHighlight,
    dispose
  };
});
