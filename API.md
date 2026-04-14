# Wuwa Gacha History API

## API URL 

```
POST https://aki-game2.com/gacha/record/query
```


## Frontend URL Parms (GET)
```
https://aki-gm-resources.aki-game.com/aki/gacha/index.html#/record
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

```json 
{
  "playerId": "123456789",
  "serverId": "76402e5b20be2c39f095a152090afddc",
  "cardPoolId": "abcdef1234567890abcdef1234567890",
  "cardPoolType": 1,
  "languageCode": "zh-Hans",
  "recordId": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",

  "size": 20,
  "lastId": null
}
```

| Parameter    | Type          | Description                                               |
| :----------- | :------------ | :-------------------------------------------------------- |
| playerId     | string        | 玩家 UID (9 digits)                                       |
| serverId     | string        | 服务器 ID                                                 |
| cardPoolId   | string        | 卡池 ID, 每期活动不同                                     |
| cardPoolType | number        | 卡池类型编号                                              |
| languageCode | string        | 语言, 如 zh-Hans, en-US                                   |
| recordId     | string        | 鉴权凭证, 1小时有效                                       |
| size         | number        | 每页条数，固定填 20（服务端上限，填其他值无效或报错）     |
| lastId       | string / null | 游标，首次请求填 null，后续填上一页最后一条记录的 id 字段 |

**cardPoolType**

| Value | Description  |
| :---- | :----------- |
| 1     | 限定角色池   |
| 2     | 限定武器池   |
| 3     | 常驻角色池   |
| 4     | 常驻武器池   |
| 5     | 新手换取     |
| 6     | 新手自选换取 |
| 7     | 感恩自选换取 |

**cardPoolId**
| cardPoolType | cardPoolId                             |
| :----------- | :------------------------------------- |
| 3            | "6994d9b2-88d3-4efa-b33e-4c7a297b5d0e" |
| 4            | "2e6e7c2b-d925-42b8-9a2f-1a57c3b6d9e0" |
| 5            | "e0fa20f7-8a2b-4c5b-9de8-8e5a3c2e4d7f" |
| 6            | "d3aa37e3-a8b4-4d5c-8a1e-5e7b9c2d1f3a" |
| 7            | "c4f5d6e7-b1a2-3c4d-5e6f-7a8b9c0d1e2f" |

## Response

```json 
{
  "code": 0,
  "message": "success",
  "data": [
    {
      "id": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
      "cardPoolType": 1,
      "resourceId": 1501,
      "qualityLevel": 5,
      "resourceType": "角色",
      "name": "鉴心",
      "count": 1,
      "time": "2024-05-24 10:32:15"
    },
    {
      "id": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
      "cardPoolType": 1,
      "resourceId": 2011,
      "qualityLevel": 4,
      "resourceType": "武器",
      "name": "远行者长刃·辟路",
      "count": 1,
      "time": "2024-05-24 10:30:01"
    }
  ]
}
```

| Parameter | Type   | Description                      |
| :-------- | :----- | :------------------------------- |
| code      | number | 状态码，0 表示成功，其他表示失败 |
| message   | string | 状态描述                         |
| data      | array  | 抽卡记录列表，按时间倒序排列     |

**data**
| Parameter    | Type   | Description                        |
| :----------- | :----- | :--------------------------------- |
| id           | string | 记录 ID                            |
| cardPoolType | number | 卡池类型，与请求一致               |
| resourceId   | number | 道具的内部 ID                      |
| qualityLevel | number | 品质星级，3/4/5 对应三/四/五星     |
| resourceType | string | 道具类型，"角色" 或 "武器"         |
| name         | string | 道具名称（根据请求的语言返回）     |
| count        | number | 数量，固定为 1                     |
| time         | string | 抽取时间，格式 YYYY-MM-DD HH:mm:ss |