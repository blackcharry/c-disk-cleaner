// src/components/ScanPanel/index.tsx

import React from 'react';
import { Card, Button, Space, Select, InputNumber, Typography, Spin, Alert } from 'antd';
import { PlayCircleOutlined } from '@ant-design/icons';
import { useScanStore } from '../../stores/scanStore';

const ScanPanel: React.FC = () => {
  const { isScanning, result, error, doStartScan } = useScanStore();

  return (
    <Card style={{ marginBottom: 16 }}>
      <Space direction="vertical" size="middle" style={{ width: '100%' }}>
        <Space>
          <Button
            type="primary"
            size="large"
            icon={<PlayCircleOutlined />}
            onClick={doStartScan}
            loading={isScanning}
          >
            开始扫描
          </Button>
          <Typography.Text type="secondary">盘符：C:</Typography.Text>
        </Space>

        {isScanning && (
          <Space>
            <Spin size="small" />
            <Typography.Text type="secondary">
              正在扫描 C 盘，请稍候...
            </Typography.Text>
          </Space>
        )}

        {error && (
          <Alert
            message="扫描失败"
            description={error}
            type="error"
            showIcon
            closable
          />
        )}

        {result && !isScanning && (
          <Space size="large" wrap>
            <Typography.Text>
              扫描完成，耗时{' '}
              <strong>{(result.scan_duration_ms / 1000).toFixed(1)}s</strong>
            </Typography.Text>
            <Typography.Text>
              共 <strong>{result.entry_count}</strong> 个条目
            </Typography.Text>
            <Typography.Text>
              C盘已用{' '}
              <strong>
                {(result.used_size / 1024 / 1024 / 1024).toFixed(1)} GB
              </strong>
            </Typography.Text>
          </Space>
        )}
      </Space>
    </Card>
  );
};

export default ScanPanel;
