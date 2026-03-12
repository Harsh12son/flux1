import type { PluginDefinition } from "../types";

// A very small and safe calculator: only digits, + - * / . and parentheses.
function evaluateExpression(expr: string): number | null {
  const cleaned = expr.replace(/[^0-9+\-*/().]/g, "");
  if (!cleaned.trim()) return null;
  // eslint-disable-next-line no-new-func
  const fn = new Function(`return (${cleaned});`);
  try {
    const result = fn();
    if (typeof result === "number" && Number.isFinite(result)) {
      return result;
    }
  } catch {
    return null;
  }
  return null;
}

const calculatorPlugin: PluginDefinition = {
  name: "calculator",
  description: "Evaluate math expressions",
  trigger: "=",
  provideResults(query) {
    const expr = query.slice(1).trim();
    const value = evaluateExpression(expr);
    if (value === null) return [];
    return [
      {
        id: `plugin:calculator:${expr}`,
        title: `${value}`,
        subtitle: expr,
        kind: "plugin",
        icon: "∑",
        hint: "Enter",
        score: 200
      }
    ];
  },
  execute(resultId) {
    const parts = resultId.split(":");
    const expr = decodeURIComponent(parts.slice(2).join(":"));
    const value = evaluateExpression(expr);
    if (value === null) return;
    navigator.clipboard.writeText(String(value)).catch(() => undefined);
  }
};

export default calculatorPlugin;

