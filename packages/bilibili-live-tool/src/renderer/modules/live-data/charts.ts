import type { StatTrendPoint } from '@/platform/desktop';
import { formatCount, formatDanmuRate, formatMoneyMinor, formatTimestamp } from './formatters';

export type TrendMetric = 'onlineCount' | 'danmuRate' | 'giftValueMinor';

function metricValue(point: StatTrendPoint, metric: TrendMetric): number {
  return metric === 'giftValueMinor' ? point[metric] / 100 : point[metric];
}

export function buildTrendOption(
  points: readonly StatTrendPoint[],
  metric: TrendMetric,
  label: string,
  color: string
) {
  return {
    animation: false,
    color: [color],
    grid: { left: 48, right: 22, top: 24, bottom: 38 },
    tooltip: {
      trigger: 'axis',
      formatter(parameters: unknown): string {
        const first = Array.isArray(parameters) ? parameters[0] : parameters;
        const index =
          typeof first === 'object' && first !== null && 'dataIndex' in first
            ? Number((first as { dataIndex: unknown }).dataIndex)
            : 0;
        const point = points[index];
        if (!point) return '';
        return [
          formatTimestamp(point.occurredAt),
          `在线人数 ${formatCount(point.onlineCount)}`,
          `弹幕总量 ${formatCount(point.danmuCount)}`,
          `弹幕速率 ${formatDanmuRate(point.danmuRate)}`,
          `礼物总价值 ${formatMoneyMinor(point.giftValueMinor)}`,
          `舰长总数 ${formatCount(point.captainCount)}`
        ].join('<br>');
      }
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: points.map(point => formatTimestamp(point.occurredAt)),
      axisLabel: { color: '#6b7280' }
    },
    yAxis: {
      type: 'value',
      axisLabel: { color: '#6b7280' },
      splitLine: { lineStyle: { color: '#e5e7eb' } }
    },
    series: [
      {
        name: label,
        type: 'line',
        showSymbol: points.length < 30,
        smooth: true,
        data: points.map(point => metricValue(point, metric))
      }
    ]
  };
}
