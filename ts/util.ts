import { prepare } from "../deps.ts";
import { FFI_OPS } from "./types.ts";

// @ts-ignore
export const DenoCore = Deno.core as {
  ops: () => { [key: string]: number };
  dispatch(rid: number, ...buf: ArrayBufferView[]): Uint8Array | undefined;
};

const PLUGIN_NAME = "deno_ffi";
let pluginId = 0;
let pluginUrl = "";
const encoder = new TextEncoder();
const decoder = new TextDecoder();

export function setPluginUrl(url: string) {
  pluginUrl = url;
}

export async function initPlugin() {
  const releaseUrl = pluginUrl;
  if (!pluginId) {
    const options = {
      name: PLUGIN_NAME,
      urls: {
        darwin: `${releaseUrl}/lib${PLUGIN_NAME}.dylib`,
        windows: `${releaseUrl}/${PLUGIN_NAME}.dll`,
        linux: `${releaseUrl}/lib${PLUGIN_NAME}.so`,
      },
    };
    pluginId = await prepare(options);
  }
}

export function closePlugin() {
  if (pluginId) {
    Deno.close(pluginId);
    pluginId = 0;
  }
}

export function dispatch(op: FFI_OPS, ...data: Uint8Array[]) {
  const ops = DenoCore.ops() as { [key in keyof typeof FFI_OPS]: number };
  const result = DenoCore.dispatch(ops[op], ...data);
  return result;
}

export function encodeString(str: string): Uint8Array {
  return encoder.encode(str);
}

export function decodeString(buf: Uint8Array): string {
  return decoder.decode(buf);
}
