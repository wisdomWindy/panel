/**
 * 浏览器兼容的 EventEmitter（替代 Node.js events 模块）
 * 用于渲染进程
 */

type Listener = (...args: unknown[]) => void;

export class EventEmitter {
  private events: Map<string, Listener[]> = new Map();

  on(event: string, listener: Listener): this {
    if (!this.events.has(event)) {
      this.events.set(event, []);
    }
    this.events.get(event)!.push(listener);
    return this;
  }

  off(event: string, listener: Listener): this {
    const listeners = this.events.get(event);
    if (listeners) {
      const idx = listeners.indexOf(listener);
      if (idx !== -1) listeners.splice(idx, 1);
    }
    return this;
  }

  emit(event: string, ...args: unknown[]): boolean {
    const listeners = this.events.get(event);
    if (!listeners || listeners.length === 0) return false;
    for (const listener of [...listeners]) {
      listener(...args);
    }
    return true;
  }

  once(event: string, listener: Listener): this {
    const onceWrapper: Listener = (...args) => {
      this.off(event, onceWrapper);
      listener(...args);
    };
    return this.on(event, onceWrapper);
  }

  removeAllListeners(event?: string): this {
    if (event) {
      this.events.delete(event);
    } else {
      this.events.clear();
    }
    return this;
  }

  listenerCount(event: string): number {
    const listeners = this.events.get(event);
    return listeners ? listeners.length : 0;
  }

  listeners(event: string): Listener[] {
    return [...(this.events.get(event) || [])];
  }
}

export default EventEmitter;
