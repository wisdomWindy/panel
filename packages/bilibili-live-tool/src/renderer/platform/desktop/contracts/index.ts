import type { BaseCommandRequestMap, BaseCommandResultMap, BaseEventPayloadMap } from './base';
import type { AiCommandRequestMap, AiCommandResultMap, AiEventPayloadMap } from './ai';
import type { LiveDataCommandRequestMap, LiveDataCommandResultMap, LiveDataEventPayloadMap } from './live-data';
import type { MediaCommandRequestMap, MediaCommandResultMap, MediaEventPayloadMap } from './media';
import type { MemberCommandRequestMap, MemberCommandResultMap, MemberEventPayloadMap } from './member';
import type { ReleaseCommandRequestMap, ReleaseCommandResultMap, ReleaseEventPayloadMap } from './release';
import type { ShellCommandRequestMap, ShellCommandResultMap, ShellEventPayloadMap } from './shell';

export * from './base';
export * from './ai';
export * from './live-data';
export * from './media';
export * from './member';
export * from './release';
export * from './shell';

export type CommandRequestMap = BaseCommandRequestMap & AiCommandRequestMap & LiveDataCommandRequestMap & MediaCommandRequestMap & MemberCommandRequestMap & ReleaseCommandRequestMap & ShellCommandRequestMap;
export type CommandResultMap = BaseCommandResultMap & AiCommandResultMap & LiveDataCommandResultMap & MediaCommandResultMap & MemberCommandResultMap & ReleaseCommandResultMap & ShellCommandResultMap;
export type EventPayloadMap = BaseEventPayloadMap & AiEventPayloadMap & LiveDataEventPayloadMap & MediaEventPayloadMap & MemberEventPayloadMap & ReleaseEventPayloadMap & ShellEventPayloadMap;
export type CommandName = keyof CommandRequestMap;
export type EventName = keyof EventPayloadMap;
