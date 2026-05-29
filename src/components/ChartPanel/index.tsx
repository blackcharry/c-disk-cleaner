// src/components/ChartPanel/index.tsx

import React from 'react';
import { Card, Row, Col, Statistic } from 'antd';
import { useScanStore } from '../../stores/scanStore';
import PieChart from './PieChart';
import { formatSize } from '../../utils/format';

const ChartPanel: React.FC = () => {
  const { result } = useScanStore();
  if (!result) return null;

  const safeSize = result.entries
    .filter((e) => e.risk_level === 'Safe')
    .reduce((s, e) => s + e.size_bytes, 0);
  const forbiddenSize = result.entries
    .filter((e) => e.risk_level === 'Forbidden')
    .reduce((s, e) => s + e.size_bytes, 0);

  return (
    <Card style={{ marginBottom: 16 }}>
      <Row gutter={24}>
        <Col span={16}>
          <PieChart />
        </Col>
        <Col span={8}>
          <Statistic
            title="扫描文件总大小"
            value={formatSize(result.used_size)}
            style={{ marginBottom: 16 }}
          />
          <Statistic
            title="🟢 可安全删除"
            value={formatSize(safeSize)}
            valueStyle={{ color: '#52c41a' }}
            style={{ marginBottom: 16 }}
          />
          <Statistic
            title="🔴 禁止删除（系统）"
            value={formatSize(forbiddenSize)}
            valueStyle={{ color: '#ff4d4f' }}
            style={{ marginBottom: 16 }}
          />
          <Statistic
            title="扫描条目数"
            value={result.entry_count}
            suffix="个"
          />
        </Col>
      </Row>
    </Card>
  );
};

export default ChartPanel;
