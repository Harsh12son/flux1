import React, { KeyboardEvent, useCallback, useEffect } from "react";
import { CommandBar } from "../components/CommandBar";
import { ResultList } from "../components/ResultList";
import { useKeyboardNav } from "../hooks/useKeyboardNav";
import { useSearch } from "../hooks/useSearch";
import { usePlugins } from "../hooks/usePlugins";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const isTauri =
  typeof window !== "undefined" && "__TAURI_IPC__" in window;

const win = isTauri
  ? getCurrentWindow()
  : {
      hide: async () => {},
      show: async () => {},
      setFocus: async () => {},
    };

export const Launcher: React.FC = () => {
  const { query, setQuery, results, executeResult } = useSearch();
  const { pluginResults, executePluginResult } = usePlugins(query);

  const combinedResults = [...pluginResults, ...results];

  const { activeIndex, moveDown, moveUp, reset } =
    useKeyboardNav(combinedResults.length);

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      moveDown();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      moveUp();
    } else if (e.key === "Enter") {
      e.preventDefault();
      const result = combinedResults[activeIndex];

      if (result) {
        if (result.id.startsWith("plugin:")) {
          void executePluginResult(result.id);
        } else {
          void executeResult(result);
        }
      }
    } else if (e.key === "Escape") {
      e.preventDefault();
      win.hide();
    }
  };

  const handleSelect = useCallback(
    (index: number) => {
      const result = combinedResults[index];
      if (!result) return;

      if (result.id.startsWith("plugin:")) {
        void executePluginResult(result.id);
      } else {
        void executeResult(result);
      }
    },
    [combinedResults, executePluginResult, executeResult]
  );

  useEffect(() => {
    if (!isTauri) return;

    const unlistenPromise = listen("show_launcher", async () => {
      reset();
      setQuery("");
      await win.show();
      await win.setFocus();
    });

    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
    };
  }, [reset, setQuery]);

  return (
    <div className="flux-launcher-backdrop">
      <div className="flux-launcher-shell">
        <CommandBar
          value={query}
          onChange={setQuery}
          onKeyDown={handleKeyDown}
        />

        <ResultList
          items={combinedResults}
          activeIndex={activeIndex}
          onSelect={handleSelect}
        />
      </div>
    </div>
  );
};