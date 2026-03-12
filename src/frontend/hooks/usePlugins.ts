import { useEffect, useMemo, useState } from "react";
import type { SearchResult } from "./useSearch";
import type { PluginDefinition } from "@plugins/types";
import calculator from "@plugins/calculator";
import webSearch from "@plugins/web-search";
import clipboard from "@plugins/clipboard";

const PLUGINS: PluginDefinition[] = [calculator, webSearch, clipboard];

export const usePlugins = (query: string) => {
  const [plugins] = useState<PluginDefinition[]>(PLUGINS);

  const pluginResults: SearchResult[] = useMemo(() => {
    if (!query.trim()) return [];
    const all: SearchResult[] = [];
    for (const plugin of plugins) {
      if (!query.startsWith(plugin.trigger)) continue;
      const res = plugin.provideResults(query);
      all.push(...res);
    }
    return all;
  }, [plugins, query]);

  const executePluginResult = async (id: string) => {
    const match = plugins.find((p) => id.startsWith(`plugin:${p.name}:`));
    if (!match) return;
    await match.execute(id);
  };

  return {
    pluginResults,
    executePluginResult
  };
};

