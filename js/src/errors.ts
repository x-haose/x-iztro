import type { KnowledgeIssue } from './types.js';

/**
 * 内核报出的错误类别，与 Rust 核心 `IztroError` 的 code 一一对应：
 * - `invalid_date`：日期格式非法、日期不存在（含改历空缺 1582-10-05 至 10-14），或超出 1-9999 年
 * - `invalid_time_index`：时辰索引不在 0-12
 * - `invalid_argument`：其余入参或配置非法，如未知的性别、语言、星耀标识
 * - `internal`：库内部缺陷或 WebAssembly 实例故障（trap）
 */
export type ErrorCode = 'invalid_date' | 'invalid_time_index' | 'invalid_argument' | 'internal';

/** `IztroError` 的构造选项 */
export interface IztroErrorOptions extends ErrorOptions {
  /** 知识包被拒绝时的全部校验问题 */
  issues?: readonly KnowledgeIssue[] | undefined;
}

/**
 * 排盘与查询的失败。`message` 是内核给出的英文原文，`code` 供程序按类别判断。
 *
 * 调用方用错接口（如未初始化、传错参数类型）抛的是标准的 `Error` / `TypeError`，不是本类。
 */
export class IztroError extends Error {
  /** 机器可读的错误类别 */
  readonly code: ErrorCode;
  /** 知识包构造（`KnowledgePack.fromObject` 等）被拒绝时的全部校验问题，含 warning；其余错误为 undefined */
  readonly issues: readonly KnowledgeIssue[] | undefined;

  constructor(code: ErrorCode, message: string, options?: IztroErrorOptions) {
    super(message, options);
    this.name = 'IztroError';
    this.code = code;
    this.issues = options?.issues;
  }
}

/** JS 侧校验失败时的 `IztroError`：code 为 invalid_argument，与内核同类报错同形 */
export function invalidArgument(message: string): IztroError {
  return new IztroError('invalid_argument', message);
}
