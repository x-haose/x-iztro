/**
 * 不装载内核的纯数据入口（`x-iztro/types`）：内核 DTO 的类型与枚举常量。
 *
 * 只渲染排盘结果的共享组件从这里导入：枚举是普通常量对象，类型在编译后擦除，
 * 导入本入口不会取 wasm、不会实例化内核，也不会把内核打进包里。排盘与 `init()` 由各应用自己的入口做，
 * 把 `astrolabe.toJSON()` 之类的 DTO 交给组件。
 */

export * from './enums.js';
export type * from './types.js';
