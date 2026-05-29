// src/components/ResultTable/SoftwareTag.tsx

import React from 'react';
import { Tag } from 'antd';

interface SoftwareTagProps {
  name: string | null;
}

const SoftwareTag: React.FC<SoftwareTagProps> = ({ name }) => {
  if (!name) return <Tag>未知</Tag>;

  let color: string;
  switch (name) {
    case 'Windows 系统':
      color = 'red';
      break;
    case '临时文件':
      color = 'green';
      break;
    case '缓存文件':
      color = 'cyan';
      break;
    default:
      color = 'blue';
  }

  return <Tag color={color}>{name}</Tag>;
};

export default SoftwareTag;
