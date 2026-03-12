import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import type { ResultItemData } from "../components/ResultItem";

export interface SearchResult extends ResultItemData {
  score: number;
  payload?: Record<string, unknown>;
}

const isTauri =
  typeof window !== "undefined" && "__TAURI_IPC__" in window;

export const useSearch = () => {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  const lastQueryRef = useRef("");
  const timeoutRef = useRef<number | null>(null);

  useEffect(() => {
    if (timeoutRef.current) {
      window.clearTimeout(timeoutRef.current);
    }

    if (!query.trim()) {
      setResults([]);
      return;
    }

    timeoutRef.current = window.setTimeout(async () => {
      try {
        if (!isTauri) return;

        setIsSearching(true);
        lastQueryRef.current = query;

        const start = performance.now();

        const raw = await invoke<SearchResult[]>("search_index", {
          query,
        });

        const end = performance.now();

        console.debug(
          `Search "${query}" ${(end - start).toFixed(2)}ms`
        );

        if (lastQueryRef.current === query) {
          setResults(raw);
        }
      } catch (err) {
        console.error("search_index error", err);
      } finally {
        setIsSearching(false);
      }
    }, 35);

    return () => {
      if (timeoutRef.current) {
        window.clearTimeout(timeoutRef.current);
      }
    };
  }, [query]);

  const executeResult = async (result: SearchResult) => {
    try {
      if (!isTauri) return;

      await invoke("execute_result", {
        id: result.id,
      });
    } catch (err) {
      console.error("execute_result error", err);
    }
  };

  return {
    query,
    setQuery,
    results,
    isSearching,
    executeResult,
  };
};