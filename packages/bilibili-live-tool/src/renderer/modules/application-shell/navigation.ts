export interface ShellNavigationItem {
  readonly id: string;
  readonly labelKey: string;
  readonly route?: string;
  readonly isDisabled: boolean;
  readonly children?: readonly ShellNavigationItem[];
}

const item = (
  id: string,
  route: string | undefined,
  children?: readonly ShellNavigationItem[],
  isDisabled = false
): ShellNavigationItem => ({ id, labelKey: `shell.navigation.${id}`, route, isDisabled, children });

const freeTree = (): readonly ShellNavigationItem[] => Object.freeze([
  item('liveRoom', '/live'),
  item('danmaku', '/danmaku-list'),
  item('dashboard', '/dashboard'),
  item('rankings', '/rankings'),
  item('risk', undefined, Object.freeze([
    item('sensitiveWords', '/risk/sensitive'),
    item('blacklist', '/risk/blacklist')
  ])),
  item('reminders', undefined, Object.freeze([
    item('giftReminder', '/reminders/gift'),
    item('captainEffect', '/reminders/captain'),
    item('giftSound', '/reminders/sound')
  ])),
  item('tts', '/tts'),
  item('recording', '/recording'),
  item('ai', '/ai'),
  item('settings', '/settings')
]);

export function buildNavigationTree(): readonly ShellNavigationItem[] {
  return freeTree();
}

export function isNavigationRouteAvailable(
  tree: readonly ShellNavigationItem[],
  route: string
): boolean {
  return tree.some(entry =>
    (!entry.isDisabled && entry.route === route) ||
    (entry.children !== undefined && isNavigationRouteAvailable(entry.children, route))
  );
}
