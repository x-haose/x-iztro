/**
 * 首页的十二宫盘预览。
 *
 * 盘面数据取自 `by_solar("2000-8-16", 2, female)` 的真实排盘结果
 * （与 tests/golden 的金标一致），按紫微斗数传统盘型排布：
 * 十二地支绕成一圈，中间四格留作中宫。
 */

type Cell = {
  /** 地支 */
  branch: string;
  /** 宫名 */
  palace: string;
  /** 宫干 */
  stem: string;
  /** 该宫主星，空宫为空数组 */
  stars: { name: string; mutagen?: '禄' | '权' | '科' | '忌' }[];
  /** 是否命宫 */
  soul?: boolean;
  /** 是否身宫 */
  body?: boolean;
};

/** 按盘面从左上到右下的顺序排列，中宫在渲染时单独插入。 */
const CELLS: Cell[] = [
  { branch: '巳', palace: '兄弟', stem: '辛', stars: [{ name: '天机' }] },
  { branch: '午', palace: '命宫', stem: '壬', stars: [{ name: '紫微' }], soul: true },
  { branch: '未', palace: '父母', stem: '癸', stars: [] },
  { branch: '申', palace: '福德', stem: '甲', stars: [{ name: '破军' }] },
  { branch: '辰', palace: '夫妻', stem: '庚', stars: [{ name: '七杀' }] },
  { branch: '酉', palace: '田宅', stem: '乙', stars: [] },
  { branch: '卯', palace: '子女', stem: '己', stars: [{ name: '太阳', mutagen: '禄' }, { name: '天梁' }] },
  { branch: '戌', palace: '官禄', stem: '丙', stars: [{ name: '廉贞' }, { name: '天府' }], body: true },
  { branch: '寅', palace: '财帛', stem: '戊', stars: [{ name: '武曲', mutagen: '权' }, { name: '天相' }] },
  { branch: '丑', palace: '疾厄', stem: '己', stars: [{ name: '天同', mutagen: '忌' }, { name: '巨门' }] },
  { branch: '子', palace: '迁移', stem: '戊', stars: [{ name: '贪狼' }] },
  { branch: '亥', palace: '仆役', stem: '丁', stars: [{ name: '太阴', mutagen: '科' }] },
];

/** 四化各自的着色，与文档正文的约定一致。 */
const MUTAGEN_TONE: Record<string, string> = {
  禄: 'text-emerald-600 dark:text-emerald-400',
  权: 'text-blue-600 dark:text-blue-400',
  科: 'text-amber-600 dark:text-amber-400',
  忌: 'text-rose-600 dark:text-rose-400',
};

/** 十二格在 4×4 网格中的位置，中间 2×2 空出给中宫。 */
const POSITIONS = [
  'col-start-1 row-start-1',
  'col-start-2 row-start-1',
  'col-start-3 row-start-1',
  'col-start-4 row-start-1',
  'col-start-1 row-start-2',
  'col-start-4 row-start-2',
  'col-start-1 row-start-3',
  'col-start-4 row-start-3',
  'col-start-1 row-start-4',
  'col-start-2 row-start-4',
  'col-start-3 row-start-4',
  'col-start-4 row-start-4',
];

function PalaceCell({ cell, position }: { cell: Cell; position: string }) {
  return (
    <div
      className={`${position} relative flex flex-col justify-between overflow-hidden rounded-lg border p-2 transition-colors ${
        cell.soul
          ? 'border-fd-primary/60 bg-fd-primary/5'
          : 'border-fd-border/70 bg-fd-card/40 hover:border-fd-border'
      }`}
    >
      <div className="flex flex-wrap gap-x-1 gap-y-0.5 font-serif-cjk text-sm leading-tight">
        {cell.stars.length > 0 ? (
          cell.stars.map((s) => (
            <span key={s.name} className="whitespace-nowrap">
              {s.name}
              {s.mutagen && (
                <sup className={`ml-0.5 text-[0.6rem] font-medium ${MUTAGEN_TONE[s.mutagen]}`}>
                  {s.mutagen}
                </sup>
              )}
            </span>
          ))
        ) : (
          <span className="text-fd-muted-foreground/50 text-xs">—</span>
        )}
      </div>

      <div className="flex items-end justify-between text-[0.65rem] leading-none">
        <span
          className={
            cell.soul ? 'font-medium text-fd-primary' : 'text-fd-muted-foreground'
          }
        >
          {cell.palace}
          {cell.body && <span className="ml-1 text-fd-muted-foreground/70">身</span>}
        </span>
        <span className="text-fd-muted-foreground/60 tabular-nums">
          {cell.stem}
          {cell.branch}
        </span>
      </div>
    </div>
  );
}

export function ChartPreview({ lang }: { lang: string }) {
  const zh = lang === 'zh';

  return (
    <figure className="w-full max-w-md">
      <div className="grid aspect-square grid-cols-4 grid-rows-4 gap-1.5">
        {CELLS.map((cell, i) => (
          <PalaceCell key={cell.branch} cell={cell} position={POSITIONS[i]} />
        ))}

        <div className="col-start-2 row-start-2 col-span-2 row-span-2 flex flex-col items-center justify-center gap-1 rounded-lg border border-fd-border/50 bg-fd-muted/30 p-3 text-center">
          <span className="font-serif-cjk text-lg">
            {zh ? '紫微斗数' : 'Zi Wei Dou Shu'}
          </span>
          <dl className="mt-1 space-y-0.5 text-[0.65rem] text-fd-muted-foreground">
            <div className="flex gap-1.5">
              <dt>{zh ? '阳历' : 'Solar'}</dt>
              <dd className="tabular-nums">2000-8-16</dd>
            </div>
            <div className="flex gap-1.5">
              <dt>{zh ? '命主' : 'Soul'}</dt>
              <dd>{zh ? '破军' : 'Po Jun'}</dd>
            </div>
            <div className="flex gap-1.5">
              <dt>{zh ? '五行局' : 'Element'}</dt>
              <dd>{zh ? '木三局' : 'Wood 3'}</dd>
            </div>
          </dl>
        </div>
      </div>

      <figcaption className="mt-3 text-xs text-fd-muted-foreground">
        {zh
          ? '真实排盘结果：2000-8-16 寅时女命，与金标数据逐字段一致'
          : 'Real output: female, 2000-8-16, Tiger hour — matching the golden dataset field for field'}
      </figcaption>
    </figure>
  );
}
