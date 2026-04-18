# scripts

辅助脚本集合。目前只有一个 `main.py`（`wiki-art-fetcher`）——用于从库街区鸣潮
Wiki 抓取所有角色与武器的立绘，落盘到 `public/wiki-art/`，供前端静态引用。

本目录是一个独立的 [uv](https://docs.astral.sh/uv/) 工程，跟 Rust / 前端
工作区互不影响。

---

## 前置要求

- [uv](https://docs.astral.sh/uv/) 已安装（`brew install uv` 或 `curl -LsSf
  https://astral.sh/uv/install.sh | sh`）。首次跑 `uv run` 会自动按
  `.python-version` 拉取 CPython 3.12，无需系统 Python。

## 快速使用

都从**仓库根目录**执行：

```bash
# 抓全部角色 + 武器 → public/wiki-art/
uv run --project scripts scripts/main.py

# 自定义输出目录
uv run --project scripts scripts/main.py ./out

# 只抓一个分类
uv run --project scripts scripts/main.py --only characters
uv run --project scripts scripts/main.py --only weapons

# 调并发（默认 8）
uv run --project scripts scripts/main.py -j 16
```

首次运行会自动 `uv sync`（读 `uv.lock`，建 `.venv/`）。重跑是幂等的——
已存在且非空的文件按文件名跳过；要强制重抓删掉对应文件或整个输出目录即可。

落盘结构：

```
public/wiki-art/
├── characters/
│   ├── 琳奈.png
│   ├── 长离.png
│   └── ...      # 当前 53 张
└── weapons/
    ├── 溢彩荧辉.png
    ├── 远行者长刃·辟路.png
    └── ...      # 当前 113 张
```

文件名就是游戏内的角色/武器中文名，前端可以直接按名称拼 URL：

```tsx
<img src={`/wiki-art/characters/${record.name}.png`} />
```

---

## API 逆向说明

Wiki 页面 <https://wiki.kurobbs.com/mc/catalogue/list?fid=1099&sid=1105> 是
Vite SPA，列表数据走后端接口：

```
POST https://api.kurobbs.com/wiki/core/catalogue/item/getPage
Content-Type: application/x-www-form-urlencoded      ← 必须是 form，不是 JSON
Origin:       https://wiki.kurobbs.com
Referer:      https://wiki.kurobbs.com/
wiki_type:    9

body: catalogueId=<sid>&page=1&limit=1000
```

坑一：**Content-Type 必须是 `application/x-www-form-urlencoded`**。发 JSON
body 会被后端静默返回 `{"code":500,"msg":"系统异常"}`，从错误消息看不出这是
Content-Type 的问题。

坑二：`catalogueId` 就是 URL 里的 `sid`，不是 `fid`。当前映射：

| 分类     | catalogueId |
|----------|-------------|
| 共鸣者   | 1105        |
| 武器     | 1106        |

响应结构：

```json
{
  "code": 200,
  "msg": "操作成功",
  "data": {
    "results": {
      "total": 53,
      "records": [
        {
          "id": 18902,
          "name": "琳奈",
          "content": {
            "contentUrl": "https://prod-alicdn-community.kurobbs.com/forum/....png",
            "star": "5",
            "type": "vertical-figure",
            ...
          },
          ...
        }
      ]
    }
  }
}
```

脚本只关心 `name` 和 `content.contentUrl`。`star` / `type` 暂未使用，后续
要做 UI 分级展示可以带上。脚本在 `total > records.length` 时会抛错，避免
将来官方加分页后悄悄截断。

---

## 工程结构 / 元信息

```
scripts/
├── .gitignore          # uv 默认（忽略 .venv/、__pycache__/）
├── .python-version     # 3.12，由 uv init 写入
├── README.md           # 本文件
├── main.py             # 脚本入口
├── pyproject.toml      # 仅 httpx 一个依赖
└── uv.lock             # 锁文件，必须随仓库提交
```

工程是用命令构建的，没有手写 `pyproject.toml`：

```bash
uv init --name wiki-art-fetcher --python 3.12 --app
uv add httpx
```

要新增依赖统一用 `uv add <pkg>` / `uv remove <pkg>`，不要手写
`pyproject.toml`，否则 `uv.lock` 会和声明漂移。

## 常见问题

- **`code: 500` 不是鉴权失败**：99% 是 Content-Type 发错了或忘了送
  `Origin` / `Referer` / `wiki_type` 三个头。脚本里已经都塞好了。
- **下载到的是 HTML 而不是图片**：CDN (`prod-alicdn-community.kurobbs.com`)
  偶尔需要 `User-Agent`；脚本默认带了桌面 Chrome UA，理论上不会触发。
- **角色 / 武器新增了 Wiki 还没同步**：没办法，得等 Wiki 运营方上架。
