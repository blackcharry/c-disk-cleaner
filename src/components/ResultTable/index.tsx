// src/components/ResultTable/index.tsx

import React, { useMemo } from 'react';
import { Table, Input, Select, Space, Typography, Button } from 'antd';
import { SearchOutlined, SortAscendingOutlined } from '@ant-design/icons';
import { useScanStore } from '../../stores/scanStore';
import RiskBadge from './RiskBadge';
import SoftwareTag from './SoftwareTag';
import { formatSize, shortenPath } from '../../utils/format';
import type { RiskLevel, FileEntry } from '../../types';

const ResultTable: React.FC = () => {
  const {
    result,
    riskFilter,
    searchKeyword,
    sortBy,
    sortOrder,
    setRiskFilter,
    setSearchKeyword,
    setSortBy,
    toggleSortOrder,
    getFilteredEntries,
  } = useScanStore();

  const filtered = useMemo(() => getFilteredEntries(), [
    result,
    riskFilter,
    searchKeyword,
    sortBy,
    sortOrder,
    getFilteredEntries,
  ]);

  if (!result) {
    return (
      <div style={{ textAlign: 'center', padding: 40, color: '#999' }}>
        <Typography.Text type="secondary">
          点击「开始扫描」查看 C 盘文件分析
        </Typography.Text>
      </div>
    );
  }

  const columns = [
    {
      title: '风险',
      dataIndex: 'risk_level',
      width: 100,
      render: (level: RiskLevel) => <RiskBadge level={level} />,
    },
    {
      title: '路径',
      dataIndex: 'path',
      ellipsis: true,
      render: (path: string) => (
        <Typography.Text
          copyable
          style={{ fontSize: 13 }}
          title={path}
        >
          {shortenPath(path, 80)}
        </Typography.Text>
      ),
      sorter: true,
    },
    {
      title: '大小',
      dataIndex: 'size_bytes',
      width: 110,
      render: (size: number) => (
        <Typography.Text strong>{formatSize(size)}</Typography.Text>
      ),
      sorter: true,
    },
    {
      title: '软件',
      dataIndex: 'software_name',
      width: 120,
      render: (name: string | null) => <SoftwareTag name={name} />,
    },
    {
      title: '用途说明',
      dataIndex: 'description',
      width: 180,
      ellipsis: true,
      render: (d: string | null) =>
        d ? <Typography.Text type="secondary">{d}</Typography.Text> : null,
    },
    {
      title: '清理建议',
      dataIndex: 'cleanable_advice',
      width: 200,
      render: (a: string | null) =>
        a ? (
          <Typography.Text type={a.includes('禁止') ? 'danger' : 'secondary'}>
            {a}
          </Typography.Text>
        ) : null,
    },
  ];

  return (
    <div>
      {/* 过滤栏 */}
      <Space style={{ marginBottom: 16 }} size="middle" wrap>
        <Input
          prefix={<SearchOutlined />}
          placeholder="搜索路径或软件名..."
          value={searchKeyword}
          onChange={(e) => setSearchKeyword(e.target.value)}
          style={{ width: 220 }}
          allowClear
        />
        <Select
          value={riskFilter}
          onChange={setRiskFilter}
          style={{ width: 120 }}
          options={[
            { label: '全部', value: 'all' },
            { label: '🟢 安全', value: 'Safe' },
            { label: '🟡 谨慎', value: 'Caution' },
            { label: '🔴 禁止', value: 'Forbidden' },
          ]}
        />
        <Select
          value={sortBy}
          onChange={setSortBy}
          style={{ width: 100 }}
          options={[
            { label: '按大小', value: 'size' },
            { label: '按名称', value: 'name' },
            { label: '按风险', value: 'risk' },
          ]}
        />
        <Button
          icon={<SortAscendingOutlined rotate={sortOrder === 'asc' ? 0 : 180} />}
          onClick={toggleSortOrder}
        >
          {sortOrder === 'desc' ? '降序' : '升序'}
        </Button>
      </Space>

      <Table
        columns={columns}
        dataSource={filtered}
        rowKey="path"
        size="small"
        pagination={{
          pageSize: 50,
          showSizeChanger: true,
          showTotal: (total) => `共 ${total} 个条目`,
        }}
        scroll={{ x: 900 }}
        onChange={(_pagination, _filters, sorter: any) => {
          if (sorter.field === 'size_bytes') setSortBy('size');
          else if (sorter.field === 'path') setSortBy('name');
        }}
      />
    </div>
  );
};

export default ResultTable;
