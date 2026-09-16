const currency = new Intl.NumberFormat('zh-CN', {
  style: 'currency',
  currency: 'CNY',
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
});

const integer = new Intl.NumberFormat('zh-CN', { maximumFractionDigits: 0 });

export function formatMoneyMinor(value: number): string {
  return currency.format(value / 100);
}

export function formatCount(value: number): string {
  return integer.format(value);
}

export function formatDanmuRate(value: number): string {
  return `${value.toFixed(value % 1 === 0 ? 0 : 1)} /s`;
}

export function formatTimestamp(value: number): string {
  return new Intl.DateTimeFormat('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  }).format(value);
}
