type Listener = (...args: any[]) => void;

export class EventEmitter {
  private listeners = new Map<string | symbol, Set<Listener>>();

  on(event: string | symbol, listener: Listener): this {
    const eventListeners = this.listeners.get(event) ?? new Set<Listener>();
    eventListeners.add(listener);
    this.listeners.set(event, eventListeners);
    return this;
  }

  off(event: string | symbol, listener: Listener): this {
    const eventListeners = this.listeners.get(event);
    if (!eventListeners) {
      return this;
    }

    eventListeners.delete(listener);
    if (eventListeners.size === 0) {
      this.listeners.delete(event);
    }

    return this;
  }

  removeAllListeners(event?: string | symbol): this {
    if (typeof event === 'undefined') {
      this.listeners.clear();
      return this;
    }

    this.listeners.delete(event);
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
}
