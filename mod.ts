export { DataType, loadLibrary } from "./ts/lib.ts";
export type { ApiDefine } from "./ts/lib.ts";
export { setPluginUrl } from "./ts/util.ts";
export const VERSION = "v0.1.0";
export const RELEASE_URL =
  `https://github.com/manyuanrong/deno-ffi/releases/download/${VERSION}`;

import { setPluginUrl } from "./ts/util.ts";
setPluginUrl(RELEASE_URL);
