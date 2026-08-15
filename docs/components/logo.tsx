/** 导航栏站点标识：北极（紫微）星符号 + 库名。 */
export function Logo() {
  return (
    <span className="inline-flex items-center gap-2 font-semibold">
      <svg viewBox="0 0 24 24" width="20" height="20" aria-hidden="true">
        <path
          d="M12 2.5 13.9 9.2 20.5 11 13.9 12.8 12 19.5 10.1 12.8 3.5 11 10.1 9.2Z"
          fill="currentColor"
        />
      </svg>
      x-iztro
    </span>
  );
}
