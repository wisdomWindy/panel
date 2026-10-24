import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');

describe('dashboard page contract', () => {
  it('renders every summary value with readable text on its white card', () => {
    const page = source('./index.vue');
    const valueClasses = [...page.matchAll(/<dd class="([^"]+)"/g)].map(match => match[1]);

    expect(valueClasses).toHaveLength(5);
    expect(valueClasses.every(value => value.includes('text-gray-900'))).toBe(true);
  });
});
