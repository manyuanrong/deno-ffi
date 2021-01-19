import { DataType } from "./lib.ts";

export function convertValue(value: any, type: DataType) {
  switch (type) {
    case DataType.i64:
    case DataType.f32:
      return value.toString();
  }
  return value;
}

export function parseValue(value: any, type?: DataType) {
  if (!type) return value;
  switch (type) {
    case DataType.i64:
      return BigInt(value);
  }
  return value;
}
