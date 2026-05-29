// src/App.tsx

import React from 'react';
import { Layout, ConfigProvider, theme } from 'antd';
import zhCN from 'antd/locale/zh_CN';
import AppHeader from './components/Header';
import ScanPanel from './components/ScanPanel';
import ChartPanel from './components/ChartPanel';
import ResultTable from './components/ResultTable';

const { Content } = Layout;

const App: React.FC = () => {
  return (
    <ConfigProvider
      locale={zhCN}
      theme={{
        algorithm: theme.defaultAlgorithm,
        token: {
          colorPrimary: '#1677ff',
        },
      }}
    >
      <Layout style={{ minHeight: '100vh' }}>
        <AppHeader />
        <Content style={{ padding: 24, background: '#f5f5f5' }}>
          <ScanPanel />
          <ChartPanel />
          <ResultTable />
        </Content>
      </Layout>
    </ConfigProvider>
  );
};

export default App;
