# wuwa-gacha-history

鸣潮 (Wuthering Waves) 抽卡（调谐）记录本地跟踪工具。Tauri v2 桌面端，
SolidJS 前端 + Rust 后端，数据只落在本机 SQLite，不上传任何云端。

支持 macOS 原生客户端与 Windows 客户端。

---

## 能做什么

- **本地落库** —— 调用鸣潮 `gmserver-api.aki-game2.com/gacha/record/query`
  接口，按卡池把全部历史记录存进 `app_data_dir()/gacha.db`。多账号各占
  独立表，互不污染。
- **三种方式填抓取参数（`playerId` / `serverId` / `languageCode` /
  `recordId`）**：
  1. **手动粘贴 JSON** —— 最稳；从任何途径自己拿到参数直接贴。
  2. **从日志获取** —— 用户选一次鸣潮游戏安装目录（`public/wiki-art/` 会
     记到 `localStorage`），脚本从 `Client/Saved/Logs/Client.log` 或 KRSDK
     WebView 的 `debug.log` 里正则抽最近一次打开「调谐记录」的 URL，解析
     query 参数回填。
  3. **抓包获取** —— 本地起 hudsucker MITM 代理 + rcgen 自签 CA（macOS
     login keychain / Windows HKCU Root，都不需要管理员），把系统代理
     临时切到本机，拦截游戏发出的 `/gacha/record/query` 请求体并推回
     前端。完事自动还原代理。
- **筛选** —— 卡池 / 星级 / 道具名模糊 / 时间范围；分页 20 条 / 页。
- **导出** —— CSV / XLSX / JSON，按扩展名自动选格式。
- **立绘** —— `public/wiki-art/` 下预置了 53 位共鸣者 + 113 把武器的
  官方 Wiki 立绘（见 [scripts/README.md](scripts/README.md)），前端
  可以直接用 `/wiki-art/characters/<名字>.png` 引用。

---

## 快速开始

### 先装工具

```bash
# Bun (前端包管理 + 运行器)
curl -fsSL https://bun.sh/install | bash

# Rust 工具链（stable, edition 2024）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Tauri 原生依赖：见 https://tauri.app/start/prerequisites/
```

### 跑起来

```bash
bun install                # 装前端依赖
bun run tauri dev          # Vite + Tauri 窗口同时起
```

### 打包

```bash
bun run tauri build        # 产出 .dmg / .exe 到 src-tauri/target/release/bundle/
```

### 常用命令

```bash
cargo check --workspace             # Rust 类型检查
cargo test --workspace              # Rust 测试（当前 17 个）
bun run format                      # Prettier 跑 src/
bun run tsc --noEmit                # TS 类型检查
```

---

## 目录结构

```
.
├── Cargo.toml                   # workspace 声明
├── wuwa-gacha-history/          # 核心 Rust 库（域逻辑，纯 no-Tauri）
│   └── src/
│       ├── client/              # API 客户端（reqwest + serde）
│       ├── db.rs                # sqlx + SQLite；按 (player_id, card_pool)
│       │                         整池删 + 重写 + seq 列严格保留响应顺序
│       ├── export.rs            # CSV/XLSX/JSON 导出
│       └── error.rs             # thiserror 错误枚举
├── src-tauri/
│   └── src/
│       ├── lib.rs               # Tauri commands + state 装配
│       ├── log_reader.rs        # Client.log / debug.log 解析
│       └── sniffer/             # 抓包通道
│           ├── mod.rs           # 生命周期 + 事件推送
│           ├── ca.rs            # rcgen 生成 CA + 系统信任（keychain / certutil）
│           ├── proxy.rs         # 系统代理开关（networksetup / winreg）
│           └── interceptor.rs   # hudsucker HttpHandler
├── src/                         # SolidJS + TS 前端
│   ├── components/
│   │   ├── FetchForm.tsx        # 三种填参方式 + 主抓取按钮
│   │   ├── Sidebar.tsx          # 用户切换 + 卡池分组
│   │   ├── ContentArea.tsx      # 主列表 + 分页 + 过滤
│   │   ├── FilterPanel.tsx      # 星级 / 名称 / 时间筛选
│   │   ├── RecordTable.tsx      # 表格
│   │   ├── ExportDialog.tsx     # 导出格式 + 路径选择
│   │   └── AddUserDialog.tsx
│   └── lib/                     # 类型 + invoke 包装
├── public/wiki-art/             # 角色 + 武器立绘（166 张 PNG，进 git）
├── scripts/                     # uv 工程：wiki 立绘抓取
│   └── main.py                  # → public/wiki-art/
├── API.md                       # 鸣潮调谐接口逆向文档
├── CLAUDE.md                    # 给 Claude Code 用的项目上下文
└── README.md                    # 就是你正在看的这份
```

---

## 数据与隐私

- 所有数据只落在 `app_data_dir()/gacha.db`（macOS 在
  `~/Library/Application Support/com.tangxiangong.wuwa-gacha-history/`，
  Windows 在 `%APPDATA%\com.tangxiangong.wuwa-gacha-history\`）。
- 抓包模式会启用一个**本机自签 CA**（证书 + 私钥落在上面同目录的
  `mitm/`）和本机 HTTP 代理（只监听 `127.0.0.1`），仅拦 `aki-game2.com`
  / `aki-game2.net` 域名的 `/gacha/record/query` 请求；其他流量不解密不
  落盘。取消抓包时自动还原系统代理。
- 不会把任何数据发到第三方（包括我自己）。

---

## 相关文档

- [API.md](API.md) —— 调谐查询接口的逆向说明：实际 URL、Content-Type、
  本地化 `cardPoolType` 字符串、为什么我们不用服务端 `id`、seq 列怎么保持
  游戏内顺序等。
- [scripts/README.md](scripts/README.md) —— Wiki 立绘抓取脚本的用法 +
  Kurobbs 接口细节。
- [CLAUDE.md](CLAUDE.md) —— 项目的架构摘要，给 Claude Code / 新贡献者当
  入门地图。

---

## 许可证

MIT OR Apache-2.0。数据来源：游戏本体 API 与官方 Wiki（库街区）。本项目
与 Kuro Games / 库街区无关。
