import type { PluginDefinition } from "../types";

const webSearchPlugin: PluginDefinition = {
  name: "web-search",
  description: "Google search using default browser",
  trigger: "g ",
  provideResults(query) {
    if (!query.toLowerCase().startsWith("g ")) return [];
    const q = query.slice(2).trim();
    if (!q) return [];
    const encoded = encodeURIComponent(q);
    return [
      {
        id: `plugin:web-search:${encoded}`,
        title: `Search Google for "${q}"`,
        subtitle: "Open in default browser",
        kind: "plugin",
        icon: "G",
        hint: "Enter",
        score: 180
      }
    ];
  },
  execute(resultId) {
    const parts = resultId.split(":");
    const encoded = parts.slice(2).join(":");
    const url = `https://www.google.com/search?q=${encoded}`;
    window.open(url, "_blank", "noopener,noreferrer");
  }
};

export default webSearchPlugin;

