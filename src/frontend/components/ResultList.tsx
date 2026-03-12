import React from "react";
import { ResultItem, ResultItemData } from "./ResultItem";

interface Props {
  items: ResultItemData[];
  activeIndex: number;
  onSelect: (index: number) => void;
}

export const ResultList: React.FC<Props> = ({
  items,
  activeIndex,
  onSelect
}) => {
  if (items.length === 0) {
    return (
      <div className="px-4 py-6 text-center text-sm text-textSecondary">
        Start typing to search apps, files, and commands.
      </div>
    );
  }

  return (
    <div className="max-h-[420px] overflow-y-auto py-2">
      {items.map((item, index) => (
        <ResultItem
          key={item.id}
          item={item}
          active={index === activeIndex}
          onClick={() => onSelect(index)}
        />
      ))}
    </div>
  );
};

