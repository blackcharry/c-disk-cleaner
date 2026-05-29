// src/components/Header/index.tsx

import React from 'react';
import { Layout, Space, Typography, Tag } from 'antd';
import { ScanOutlined } from '@ant-design/icons';

const { Header: AntHeader } = Layout;
const { Title } = Typography;

const AppHeader: React.FC = () => {
  return (
    <AntHeader
      style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        background: '#fff',
        borderBottom: '1px solid #f0f0f0',
        padding: '0 24px',
        height: 56,
      }}
    >
      <Space>
        <ScanOutlined style={{ fontSize: 20, color: '#1677ff' }} />
        <Title level={5} style={{ margin: 0 }}>
          C盘文件管家
        </Title>
        <Tag color="blue" style={{ marginLeft: 8 }}>
          v0.1.0
        </Tag>
      </Space>
      <Typography.Text type="secondary" style={{ fontSize: 12 }}>
        纯信息展示工具 · 不执行清理操作
      </Typography.Text>
    </AntHeader>
  );
};

export default AppHeader;
