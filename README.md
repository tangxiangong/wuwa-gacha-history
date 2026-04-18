# wuwa-gacha-history

鸣潮抽卡记录查询工具。Tauri v2 + SolidJS + Rust，支持 macOS / Windows。

## 功能

- 查询并存储调谐记录到本地 SQLite
- 按卡池、星级、名称、时间范围筛选
- 导出 CSV / XLSX / JSON
- 三种参数获取方式：手动粘贴 JSON、读取游戏日志、MITM 抓包

## 参数获取

调用 `gmserver-api.aki-game2.com/gacha/record/query` 需要 `playerId`、
`serverId`、`languageCode`、`recordId` 四个字段，其中 `recordId` 是约
一小时过期的鉴权凭证。

| 方式 | 说明 |
| --- | --- |
| 手动 JSON | 从其他途径拿到参数后直接粘贴 |
| 日志读取 | 指定鸣潮安装目录后，从 `Client/Saved/Logs/Client.log` 或 KRSDK WebView 的 `debug.log` 提取最近一次抽卡记录页面的 URL |
| MITM 抓包 | 本地启动 hudsucker 代理，使用 rcgen 签发的自签 CA 解密 TLS，拦截游戏发出的请求体。关闭时自动还原系统代理 |

## 开发

```bash
bun install
bun run tauri dev
bun run tauri build
```

依赖：bun、Rust（edition 2024）、Tauri 原生工具链
（详见 <https://tauri.app/start/prerequisites/>）。

检查与测试：

```bash
cargo check --workspace
cargo test --workspace
bun run tsc --noEmit
bun run format
```

## 目录结构

```
wuwa-gacha-history/     核心域逻辑：API 客户端、SQLite、导出
src-tauri/              Tauri 后端，sniffer/ 为抓包实现
src/                    SolidJS + TypeScript 前端
public/wiki-art/        角色与武器立绘（53 + 113，从库街区 Wiki 抓取）
scripts/                uv 工程，包含立绘抓取脚本
```

## 数据位置

- 数据库：`app_data_dir()/gacha.db`
  - macOS: `~/Library/Application Support/com.tangxiangong.wuwa-gacha-history/`
  - Windows: `%APPDATA%\com.tangxiangong.wuwa-gacha-history\`
- MITM 自签 CA：同目录下 `mitm/ca.pem` + `ca.key.pem`

## 文档

- [API.md](API.md) —— 抽卡接口请求/响应结构与已知行为
- [scripts/README.md](scripts/README.md) —— 立绘抓取脚本
- [CLAUDE.md](CLAUDE.md) —— 架构概览

## License

MIT OR Apache-2.0
