import React, { KeyboardEvent } from "react";

interface CommandBarProps {
  value: string;
  onChange: (value: string) => void;
  onKeyDown: (event: KeyboardEvent<HTMLInputElement>) => void;
}

export const CommandBar: React.FC<CommandBarProps> = ({
  value,
  onChange,
  onKeyDown
}) => {
  return (
    <div className="flex items-center gap-2 border-b border-surfaceAlt bg-surface">
      <div className="px-3 text-textSecondary text-sm font-medium">⌘</div>
      <input
        autoFocus
        className="flux-command-input"
        placeholder="Type a command or search…"
        value={value}
        spellCheck={false}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={onKeyDown}
      />
      <div className="flex items-center gap-2 pr-3 text-[11px] text-textSecondary">
        <span className="flux-kbd">↑↓</span>
        <span className="flux-kbd">Enter</span>
        <span className="flux-kbd">Esc</span>
      </div>
    </div>
  );
};

