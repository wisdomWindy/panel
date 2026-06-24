type Listener = (...args: any[]) => void;

export class EventEmitter {
  private listeners = new Map<string | symbol, Set<Listener>>();
  private onceWrappers = new Map<Listener, Listener>();

  on(event: string | symbol, listener: Listener): this {
    const eventListeners = this.listeners.get(event) ?? new Set<Listener>();
    eventListeners.add(listener);
    this.listeners.set(event, eventListeners);
    return this;
  }

  once(event: string | symbol, listener: Listener): this {
    const wrapper: Listener = (...args) => {
      this.off(event, wrapper);
      this.onceWrappers.delete(listener);
      listener(...args);
    };
    this.onceWrappers.set(listener, wrapper);
    return this.on(event, wrapper);
  }

  off(event: string | symbol, listener: Listener): this {
    const eventListeners = this.listeners.get(event);
    if (!eventListeners) {
      return this;
    }

    const wrapper = this.onceWrappers.get(listener);
    eventListeners.delete(wrapper ?? listener);
    this.onceWrappers.delete(listener);

    if (eventListeners.size === 0) {
      this.listeners.delete(event);
    }

    return this;
  }

  removeAllListeners(event?: string | symbol): this {
    if (typeof event === 'undefined') {
      this.listeners.clear();
      this.onceWrappers.clear();
      return this;
    }

    this.listeners.delete(event);
    for (const [original, wrapper] of this.onceWrappers) {
      if (!this.listeners.size || !this.hasWrapperInAnyEvent(wrapper)) {
        this.onceWrappers.delete(original);
      }
    }
    return this;
  }

  emit(event: string | symbol, ...args: any[]): boolean {
    const eventListeners = this.listeners.get(event);
    if (!eventListeners || eventListeners.size === 0) {
      return false;
    }

    [...eventListeners].forEach(listener => {
      listener(...args);
    });

    return true;
  }

  listenerCount(event: string | symbol): number {
    return this.listeners.get(event)?.size ?? 0;
  }

  eventNames(): (string | symbol)[] {
    return [...this.listeners.keys()];
  }

  private hasWrapperInAnyEvent(wrapper: Listener): boolean {
    for (const listeners of this.listeners.values()) {
      if (listeners.has(wrapper)) return true;
    }
    return false;
  }
}
