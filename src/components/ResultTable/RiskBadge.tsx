// src/components/ResultTable/RiskBadge.tsx

import React from 'react';
import { Tag } from 'antd';
import type { RiskLevel } from '../../types';

const riskConfig: Record<RiskLevel, { color: string; label: string }> = {
  Safe: { color: 'success', label: '🟢 安全' },
  Caution: { color: 'warning', label: '🟡 谨慎' },
  Forbidden: { color: 'error', label: '🔴 禁止' },
};

interface RiskBadgeProps {
  level: RiskLevel;
}

const RiskBadge: React.FC<RiskBadgeProps> = ({ level }) => {
  const config = riskConfig[level] || { color: 'default', label: level };
  return <Tag color={config.color}>{config.label}</Tag>;
};

export default RiskBadge;
