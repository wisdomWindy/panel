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
  item('room', '/connection'),
  item('danmaku', '/danmaku-list'),
  item('dashboard', '/dashboard'),
  item('rankings', '/rankings'),
  item('risk', undefined, Object.freeze([
    item('sensitiveWords', '/sensitive-words'),
    item('blacklist', '/blacklist')
  ])),
  item('reminders', undefined, Object.freeze([
    item('captainEffect', '/captain-effect'),
    item('giftReminder', '/gift-reminder'),
    item('giftSound', '/gift-sound')
  ])),
  item('tts', '/tts'),
  item('recording', '/recording'),
  item('plugins', '/plugins'),
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
