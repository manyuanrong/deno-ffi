import { setPluginUrl } from "./ts/util.ts";
export const VERSION = "v0.13.0";
export const RELEASE_URL = `https://github.com/manyuanrong/deno_mongo/releases/download/${VERSION}`;
export { loadLibrary } from "./ts/lib.ts";
export { initPlugin } from "./ts/util.ts";
setPluginUrl(RELEASE_URL);
