# Wuwa Gacha History API

## API URL

```
POST https://gmserver-api.aki-game2.com/gacha/record/query   # 国服
POST https://gmserver-api.aki-game2.net/gacha/record/query   # 国际服
```

## Frontend URL Params (GET)

抽卡记录 WebView 打开时加载的链接，日志里可以直接抓到：

```
https://aki-gm-resources.aki-game.com/aki/gacha/index.html#/record          # 国服
https://aki-gm-resources-oversea.aki-game.net/aki/gacha/index.html#/record  # 国际服
  ?svr_id=<serverId>
  &player_id=<playerId>
  &lang=<languageCode>
  &gacha_id=<cardPoolId>
  &gacha_type=<cardPoolType>
  &svr_area=<svrArea>
  &record_id=<recordId>
  &resources_id=<resourcesId>
```

## POST Request Body

抓包得到的真实请求（`Content-Type: application/json`）：

```json
{
  "playerId": "117584047",
  "serverId": "76402e5b20be2c39f095a152090afddc",
  "cardPoolId": "15091d1f8565611ec6f2fd4e509ab9e8",
  "cardPoolType": 1,
  "languageCode": "zh-Hans",
  "recordId": "01b6dcb718b18e3fec30e99e466eb550"
}
```

| Parameter    | Type   | Description                            |
| :----------- | :----- | :------------------------------------- |
| playerId     | string | 玩家 UID (9 位数字)                    |
| serverId     | string | 服务器 ID                              |
| cardPoolId   | string | 实测**服务端忽略**，可传空字符串       |
| cardPoolType | number | 卡池类型编号（见下表）                 |
| languageCode | string | 语言，如 `zh-Hans`、`en`               |
| recordId     | string | 鉴权凭证，约 1 小时有效，过期需重新抓取 |

> 注意一：实际游戏请求中**没有** `size` / `lastId` 字段。服务端一次性返回该卡池自建号以来的全部抽卡记录。
>
> 注意二：`cardPoolId` 实测被服务端**完全忽略**——传空串、乱码、其他池的 ID 都返回同样的数据，服务端仅根据 `cardPoolType` 路由。游戏客户端仍会在每次请求里填一个值，复现游戏行为时保留字段即可。

**cardPoolType**

| Value | Description  |
| :---- | :----------- |
| 1     | 限定角色池   |
| 2     | 限定武器池   |
| 3     | 常驻角色池   |
| 4     | 常驻武器池   |
| 5     | 新手唤取     |
| 6     | 新手自选唤取 |
| 7     | 感恩自选唤取 |


## Response

```json
{
  "code": 0,
  "message": "success",
  "data": [
    {
      "cardPoolType": "角色精准调谐",
      "resourceId": 1412,
      "qualityLevel": 5,
      "resourceType": "角色",
      "name": "长离",
      "count": 1,
      "time": "2026-03-19 12:38:37"
    },
    {
      "cardPoolType": "角色精准调谐",
      "resourceId": 21010043,
      "qualityLevel": 3,
      "resourceType": "武器",
      "name": "远行者长刃·辟路",
      "count": 1,
      "time": "2026-03-19 12:38:37"
    }
  ]
}
```

| Parameter | Type   | Description                      |
| :-------- | :----- | :------------------------------- |
| code      | number | 状态码，0 表示成功，其他表示失败 |
| message   | string | 状态描述                         |
| data      | array  | 抽卡记录列表，**按时间倒序**     |

**data[i]**

| Parameter    | Type   | Description                                                          |
| :----------- | :----- | :------------------------------------------------------------------- |
| cardPoolType | string | **本地化字符串**，按请求的 `languageCode` 返回，不是请求中的整数      |
| resourceId   | number | 道具的内部 ID                                                        |
| qualityLevel | number | 品质星级，3 / 4 / 5 对应三 / 四 / 五星                                |
| resourceType | string | 道具类型，如 `角色` / `武器`                                          |
| name         | string | 道具名称（按请求的 `languageCode` 返回）                              |
| count        | number | 数量，目前观察到固定为 1                                              |
| time         | string | 抽取时间，格式 `YYYY-MM-DD HH:MM:SS`，秒级精度                        |

**各 `cardPoolType` 返回的本地化字符串**（实测抓取，对比用）：

| type | `zh-Hans`              | `en`                              |
| :--- | :--------------------- | :-------------------------------- |
| 1    | 角色精准调谐           | Resonators Accurate Modulation    |
| 2    | 武器精准调谐           | —（未验证）                        |
| 3    | 角色调谐（常驻池）     | —（未验证）                        |
| 4    | 武器调谐（常驻池）     | —（未验证）                        |
| 5    | —（此测试玩家无数据）  | —                                  |
| 6    | —（此测试玩家无数据）  | —                                  |
| 7    | —（此测试玩家无数据）  | —                                  |

> 观察：**响应中没有 `id` 字段**，无法用服务端稳定 ID 去重。本项目存库时对每个卡池采用「整池删除 + 重新写入」策略（响应就是该卡池完整的抽卡历史），并用 `{cardPoolType}-{index}` 作为合成主键（`index` 从最早一次抽取开始计数，新增抽取时旧记录的 index 保持稳定）。
