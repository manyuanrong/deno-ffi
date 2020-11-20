import { prepare } from "../deps.ts";
import { PluginConfig } from "../mod.ts";
import { FFI_OPS } from "./types.ts";

// @ts-ignore
export const DenoCore = Deno.core as {
  ops: () => { [key: string]: number };
  setAsyncHandler(rid: number, handler: Function): void;
  dispatch(
    rid: number,
    msg: any,
    ...buf: ArrayBufferView[]
  ): Uint8Array | undefined;
};

const PLUGIN_NAME = "deno-ffi";
let ffiOps:
  | { [key in keyof typeof FFI_OPS]: number }
  | undefined;

export async function initPlugin({ releaseUrl }: PluginConfig) {
  if (!ffiOps) {
    const options = {
      name: PLUGIN_NAME,
      urls: {
        darwin: `${releaseUrl}/lib${PLUGIN_NAME}.dylib`,
        windows: `${releaseUrl}/${PLUGIN_NAME}.dll`,
        linux: `${releaseUrl}/lib${PLUGIN_NAME}.so`,
      },
    };
    await prepare(options);
    ffiOps = DenoCore.ops() as { [key in keyof typeof FFI_OPS]: number };
  }
  return ffiOps;
}
