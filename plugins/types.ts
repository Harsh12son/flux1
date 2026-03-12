import type { SearchResult } from "@frontend/hooks/useSearch";

export interface PluginDefinition {
  name: string;
  description: string;
  trigger: string;
  provideResults: (query: string) => SearchResult[];
  execute: (id: string) => void | Promise<void>;
}

