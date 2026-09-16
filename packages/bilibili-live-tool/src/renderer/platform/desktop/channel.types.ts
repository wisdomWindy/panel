import type { Component, Ref } from 'vue';
import type { RouteRecordRaw } from 'vue-router';
import type { ShellNavigationItem } from '@/modules/application-shell/navigation';
import type { MemberStatus } from './contracts/member';

export interface MemberChannel {
  readonly routes: readonly RouteRecordRaw[];
  readonly toolbarComponent: Component | null;
  readonly navigationMessages: Readonly<Record<'zh-CN' | 'en-US', Readonly<Record<string, string>>>>;
  readonly deniedRoute: string;
  buildNavigation(common: readonly ShellNavigationItem[]): readonly ShellNavigationItem[];
  useStatus(): Readonly<Ref<MemberStatus>>;
  canAccess(path: string): boolean;
  start(): Promise<void>;
  stop(): void;
}
