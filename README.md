# C盘文件管家

纯信息展示工具——扫描 C 盘，告诉你每个大文件/文件夹是什么、属于哪个软件、能不能删，不替你动手。

## 技术栈

- **桌面框架**: Tauri 2.x
- **后端**: Rust（MFT 扫描、注册表读取、指纹库匹配、风险评估）
- **前端**: React + TypeScript + Ant Design + ECharts

## 开发

```bash
# 安装依赖
pnpm install

# 启动开发模式
pnpm tauri dev

# 构建
pnpm tauri build
```

## 许可证

MIT
