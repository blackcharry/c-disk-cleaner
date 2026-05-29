// src/components/ChartPanel/PieChart.tsx

import React from 'react';
import ReactECharts from 'echarts-for-react';
import { useScanStore } from '../../stores/scanStore';
import { RiskColor } from '../../types';

const PieChart: React.FC = () => {
  const result = useScanStore((s) => s.result);
  if (!result) return null;

  const safeEntries = result.entries.filter((e) => e.risk_level === 'Safe');
  const cautionEntries = result.entries.filter((e) => e.risk_level === 'Caution');
  const forbiddenEntries = result.entries.filter((e) => e.risk_level === 'Forbidden');

  const safeSize = safeEntries.reduce((s, e) => s + e.size_bytes, 0);
  const cautionSize = cautionEntries.reduce((s, e) => s + e.size_bytes, 0);
  const forbiddenSize = forbiddenEntries.reduce((s, e) => s + e.size_bytes, 0);

  const option = {
    title: {
      text: 'C盘空间分析',
      left: 'center',
      textStyle: { fontSize: 14 },
    },
    tooltip: {
      trigger: 'item',
      formatter: (params: any) => {
        const gb = (params.value / 1024 / 1024 / 1024).toFixed(2);
        return `${params.name}: ${gb} GB (${params.percent}%)`;
      },
    },
    legend: {
      bottom: 10,
      data: ['🟢 可安全删除', '🟡 谨慎删除', '🔴 禁止删除'],
    },
    series: [
      {
        type: 'pie',
        radius: ['40%', '70%'],
        center: ['50%', '50%'],
        avoidLabelOverlap: false,
        label: { show: false },
        emphasis: {
          label: { show: true, fontSize: 14, fontWeight: 'bold' },
        },
        data: [
          {
            value: safeSize || 1,
            name: '🟢 可安全删除',
            itemStyle: { color: RiskColor.Safe },
          },
          {
            value: cautionSize || 1,
            name: '🟡 谨慎删除',
            itemStyle: { color: RiskColor.Caution },
          },
          {
            value: forbiddenSize || 1,
            name: '🔴 禁止删除',
            itemStyle: { color: RiskColor.Forbidden },
          },
        ],
      },
    ],
  };

  return <ReactECharts option={option} style={{ height: 250 }} />;
};

export default PieChart;
