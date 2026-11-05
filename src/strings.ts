import rawStrings from "./strings/en.json" with { type: "json" };

export type StringKey = Exclude<keyof typeof rawStrings, "_meta">;

/**
 * Returns the localized string for `key`, replacing `{param}` tokens.
 * Throws if a key does not exist or if a placeholder is missing a value.
 */
export function t(key: StringKey, params?: Record<string, string | number>): string {
  const template = (rawStrings as Record<string, unknown>)[key];
  if (typeof template !== "string") {
    throw new Error(`Missing string key: ${key}`);
  }

  const result = template.replace(/\{([a-zA-Z0-9_]+)\}/g, (match, paramName: string) => {
    if (!params || !(paramName in params)) {
      throw new Error(`Missing required placeholder "${paramName}" for key "${key}" in template "${template}"`);
    }
    return String(params[paramName]);
  });

  if (/\{[a-zA-Z0-9_]+\}/.test(result)) {
    throw new Error(`Unresolved placeholder remains for key "${key}": "${result}"`);
  }

  return result;
}
