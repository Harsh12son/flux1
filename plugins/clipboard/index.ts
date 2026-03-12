import type { PluginDefinition } from "../types";

const history: string[] = [];

// Track clipboard contents in this session when called explicitly via the plugin.
async function readClipboard(): Promise<string | null> {
  try {
    const text = await navigator.clipboard.readText();
    if (!text) return null;
    if (!history.includes(text)) {
      history.unshift(text);
      if (history.length > 20) history.pop();
    }
    return text;
  } catch {
    return null;
  }
}

const clipboardPlugin: PluginDefinition = {
  name: "clipboard",
  description: "Session clipboard history",
  trigger: "clip ",
  provideResults(query) {
    if (!query.toLowerCase().startsWith("clip")) return [];
    // Synchronous snapshot; reading clipboard is explicitly triggered by execution.
    return history.map((entry, index) => ({
      id: `plugin:clipboard:${index}`,
      title: entry.length > 60 ? `${entry.slice(0, 57)}…` : entry,
      subtitle: "Clipboard item",
      kind: "plugin",
      icon: "⎘",
      hint: "Enter",
      score: 150 - index
    }));
  },
  async execute(resultId) {
    const indexStr = resultId.split(":")[2];
    const index = Number.parseInt(indexStr, 10);
    if (Number.isNaN(index)) {
      await readClipboard();
      return;
    }
    const value = history[index];
    if (!value) return;
    await navigator.clipboard.writeText(value);
  }
};

export default clipboardPlugin;

