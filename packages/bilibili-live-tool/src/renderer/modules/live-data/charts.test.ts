import { describe, expect, it } from 'vitest';
import type { StatTrendPoint } from '@/platform/desktop';
import { buildTrendOption } from './charts';

const point: StatTrendPoint = {
  occurredAt: 1_700_000_000_000,
  onlineCount: 10,
  danmuCount: 20,
  danmuRate: 2.5,
  giftValueMinor: 1234,
  captainCount: 3,
  filteredCount: 4
};

describe('dashboard chart options', () => {
  it('uses Rust trend points without synthesizing history and exposes all five metrics in details', () => {
    const option = buildTrendOption([point], 'giftValueMinor', '礼物收益趋势', '#d97706');
    expect(option.series[0].data).toEqual([12.34]);
    expect(option.xAxis.data).toHaveLength(1);
    const detail = option.tooltip.formatter([{ dataIndex: 0 }]);
    expect(detail).toContain('在线人数 10');
    expect(detail).toContain('弹幕总量 20');
    expect(detail).toContain('弹幕速率 2.5 /s');
    expect(detail).toContain('礼物总价值');
    expect(detail).toContain('舰长总数 3');
  });
});
