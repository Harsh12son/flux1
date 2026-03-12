import React from "react";
import clsx from "classnames";

export type ResultKind = "app" | "file" | "command" | "plugin";

export interface ResultItemData {
  id: string;
  title: string;
  subtitle?: string;
  kind: ResultKind;
  icon?: string;
  hint?: string;
}

interface Props {
  item: ResultItemData;
  active: boolean;
  onClick: () => void;
}

export const ResultItem: React.FC<Props> = ({ item, active, onClick }) => {
  const iconLetter =
    item.icon && item.icon.length > 0 ? item.icon[0].toUpperCase() : "⋯";

  return (
    <div
      className={clsx("flux-result-row", {
        "flux-result-row-active": active
      })}
      onMouseDown={(e) => {
        e.preventDefault();
        onClick();
      }}
    >
      <div className="flex h-8 w-8 items-center justify-center rounded-xl bg-accentSoft text-sm font-semibold text-white">
        {iconLetter}
      </div>
      <div className="flex-1 min-w-0">
        <div className="flex items-center justify-between gap-2">
          <div className="truncate text-sm font-medium">{item.title}</div>
          {item.hint && (
            <span className="flux-kbd shrink-0">{item.hint}</span>
          )}
        </div>
        {item.subtitle && (
          <div className="truncate text-[11px] text-textSecondary">
            {item.subtitle}
          </div>
        )}
      </div>
      <span className="text-[10px] uppercase text-textSecondary">
        {item.kind}
      </span>
    </div>
  );
};

