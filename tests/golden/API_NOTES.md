# iztro v2 API Notes

## Top-level exports

- `astro` - main astrolabe creation module
- `star` - star calculation utilities
- `data` - constants (HEAVENLY_STEMS, EARTHLY_BRANCHES, PALACES, MUTAGEN, etc.)
- `util` - utility functions

## astro methods

### `astro.bySolar(solarDate, timeIndex, gender, fixLeap, language)`

- `solarDate`: string, e.g. `'2000-8-16'`
- `timeIndex`: number 0-12 (maps to 12 two-hour periods, 0=子early,1=丑,2=寅,...,12=子late)
- `gender`: string `'男'` or `'女'`
- `fixLeap`: boolean
- `language`: string e.g. `'zh-CN'`
- Alias: `astro.astrolabeBySolarDate()` (same signature)

### `astro.byLunar(lunarDate, timeIndex, gender, isLeap, fixLeap, language)`

- `lunarDate`: string, e.g. `'2000-7-17'`
- `timeIndex`: number 0-12
- `gender`: string `'男'` or `'女'`
- `isLeap`: boolean - whether the lunar month is a leap month
- `fixLeap`: boolean
- `language`: string e.g. `'zh-CN'`
- Alias: `astro.astrolabeByLunarDate()` (same signature)

## Astrolabe result object

### Top-level fields

| Field                       | Example                    | Type                               |
|-----------------------------|----------------------------|------------------------------------|
| `solarDate`                 | `'2000-8-16'`              | string                             |
| `lunarDate`                 | `'二〇〇〇年七月十七'`              | string                             |
| `chineseDate`               | `'庚辰 甲申 丙午 庚寅'`            | string                             |
| `time`                      | `'寅时'`                     | string                             |
| `timeRange`                 | `'03:00~05:00'`            | string                             |
| `sign`                      | `'狮子座'`                    | string                             |
| `zodiac`                    | `'龙'`                      | string                             |
| `gender`                    | `'女'`                      | string                             |
| `soul`                      | `'破军'`                     | string (star name for soul palace) |
| `body`                      | `'文昌'`                     | string (star name for body palace) |
| `fiveElementsClass`         | `'木三局'`                    | string                             |
| `earthlyBranchOfSoulPalace` | `'午'`                      | string                             |
| `earthlyBranchOfBodyPalace` | `'戌'`                      | string                             |
| `palaces`                   | array of 12 Palace objects | array                              |

### `rawDates` sub-object

```json
{
  "lunarDate": {
    "lunarYear": 2000,
    "lunarMonth": 7,
    "lunarDay": 17,
    "isLeap": false
  },
  "chineseDate": {
    "yearly": [
      "庚",
      "辰"
    ],
    "monthly": [
      "甲",
      "申"
    ],
    "daily": [
      "丙",
      "午"
    ],
    "hourly": [
      "庚",
      "寅"
    ]
  }
}
```

### Palace object (12 palaces, index 0-11)

| Field              | Example                                                | Type     |
|--------------------|--------------------------------------------------------|----------|
| `index`            | `0`                                                    | number   |
| `name`             | `'财帛'`                                                 | string   |
| `isBodyPalace`     | `false`                                                | boolean  |
| `isOriginalPalace` | `false`                                                | boolean  |
| `heavenlyStem`     | `'戊'`                                                  | string   |
| `earthlyBranch`    | `'寅'`                                                  | string   |
| `majorStars`       | array of Star objects                                  | array    |
| `minorStars`       | array of Star objects                                  | array    |
| `adjectiveStars`   | array of Star objects (no brightness/mutagen)          | array    |
| `changsheng12`     | `'绝'`                                                  | string   |
| `boshi12`          | `'飞廉'`                                                 | string   |
| `jiangqian12`      | `'岁驿'`                                                 | string   |
| `suiqian12`        | `'吊客'`                                                 | string   |
| `decadal`          | `{range:[43,52], heavenlyStem:'戊', earthlyBranch:'寅'}` | object   |
| `ages`             | `[9, 21, 33, 45, 57, 69, 81, 93, 105, 117]`            | number[] |

Note: Palace index 0 is NOT always 命宫. Palaces are arranged by earthly branch position (寅=0, 卯=1, 辰=2, ..., 丑=11).
The 命宫 could be at any index.

### Star object (majorStars / minorStars)

| Field        | Example    | Type                                                             |
|--------------|------------|------------------------------------------------------------------|
| `name`       | `'武曲'`     | string                                                           |
| `type`       | `'major'`  | string: major, minor, soft, tough, tianma, lucun, flower, helper |
| `scope`      | `'origin'` | string: origin, decadal, yearly, monthly, daily, hourly          |
| `brightness` | `'得'`      | string (empty string if none)                                    |
| `mutagen`    | `'权'`      | string (empty string or undefined if none)                       |

### Adjective Star object

| Field   | Example    | Type   |
|---------|------------|--------|
| `name`  | `'解神'`     | string |
| `type`  | `'helper'` | string |
| `scope` | `'origin'` | string |

(No brightness or mutagen fields)

## Horoscope (流运/运限)

Call: `astrolabe.horoscope(solarDate, timeIndex)`

- Returns horoscope object for the given date/time

### Horoscope result object

| Field       | Type                               |
|-------------|------------------------------------|
| `lunarDate` | string                             |
| `solarDate` | string                             |
| `decadal`   | HoroscopeItem                      |
| `age`       | AgeItem                            |
| `yearly`    | HoroscopeItem (with yearlyDecStar) |
| `monthly`   | HoroscopeItem                      |
| `daily`     | HoroscopeItem                      |
| `hourly`    | HoroscopeItem                      |

### HoroscopeItem (decadal/yearly/monthly/daily/hourly)

| Field           | Example                            | Type                                            |
|-----------------|------------------------------------|-------------------------------------------------|
| `index`         | `2`                                | number                                          |
| `name`          | `'大限'`                             | string                                          |
| `heavenlyStem`  | `'庚'`                              | string                                          |
| `earthlyBranch` | `'辰'`                              | string                                          |
| `palaceNames`   | `['夫妻','兄弟','命宫',...]`             | string[] (12 items, indexed by palace position) |
| `mutagen`       | `['太阳','武曲','太阴','天同']`            | string[] (4 items: 禄权科忌)                        |
| `stars`         | array of 12 arrays of Star objects | Star[][]                                        |

### Yearly-specific: `yearlyDecStar`

```json
{
  "suiqian12": [
    "病符",
    "岁建",
    "晦气",
    "丧门",
    "贯索",
    "官符",
    "小耗",
    "大耗",
    "龙德",
    "白虎",
    "天德",
    "吊客"
  ],
  "jiangqian12": [
    "亡神",
    "将星",
    "攀鞍",
    "岁驿",
    "息神",
    "华盖",
    "劫煞",
    "灾煞",
    "天煞",
    "指背",
    "咸池",
    "月煞"
  ]
}
```

### AgeItem

| Field           | Example             | Type     |
|-----------------|---------------------|----------|
| `index`         | `9`                 | number   |
| `nominalAge`    | `24`                | number   |
| `name`          | `'小限'`              | string   |
| `heavenlyStem`  | `'丁'`               | string   |
| `earthlyBranch` | `'亥'`               | string   |
| `palaceNames`   | string[] (12 items) | string[] |
| `mutagen`       | string[] (4 items)  | string[] |

## Time index mapping (0-12)

- 0: 早子时 (23:00~01:00 early)
- 1: 丑时 (01:00~03:00)
- 2: 寅时 (03:00~05:00)
- 3: 卯时 (05:00~07:00)
- 4: 辰时 (07:00~09:00)
- 5: 巳时 (09:00~11:00)
- 6: 午时 (11:00~13:00)
- 7: 未时 (13:00~15:00)
- 8: 申时 (15:00~17:00)
- 9: 酉时 (17:00~19:00)
- 10: 戌时 (19:00~21:00)
- 11: 亥时 (21:00~23:00)
- 12: 晚子时 (23:00~01:00 late)

## Palace names (12 palaces)

命宫, 兄弟, 夫妻, 子女, 财帛, 疾厄, 迁移, 仆役, 官禄, 田宅, 福德, 父母
