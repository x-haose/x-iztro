import { defineCloudflareConfig } from '@opennextjs/cloudflare';

/**
 * OpenNext 的 Cloudflare 适配配置。
 *
 * 文档站的页面全部在构建期生成，运行期只有搜索与 LLM 端点是动态的，
 * 因此不需要增量缓存或标签失效的额外后端。
 */
export default defineCloudflareConfig();
