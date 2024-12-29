import React from "react";

export interface InputBoxProps {
  onSubmit: (input: string) => void;
}

export const InputBox = ({ onSubmit }: InputBoxProps) => {
  const font_size = 16;
  const padding = 10;
  const init_height = 2 * padding + 3 + Math.floor((font_size - 7) / 4);
  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    e.target.style.height = "0px";
    const newHeight = e.target.scrollHeight - font_size;
    const parentHeight = e.target.parentElement?.clientHeight || Infinity;
    e.target.style.height = `${Math.min(newHeight, parentHeight)}px`;
  };
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      onSubmit(e.currentTarget.value);
      e.currentTarget.value = "";
      e.currentTarget.style.height = `${init_height}px`;
    }
  };
  return (
    <div
      style={{
        flex: "2",
        display: "flex",
        justifyContent: "center",
        alignItems: "flex-end",
        width: "100%",
        height: "100%",
      }}
    >
      <textarea
        onChange={handleInputChange}
        onKeyDown={handleKeyDown}
        placeholder=">_"
        style={{
          width: "100%",
          height: `${init_height}px`,
          maxHeight: "100%",
          fontFamily: "JetBrains Mono, monospace",
          fontSize: `${font_size}px`,
          resize: "none",
          backgroundColor: "#000000",
          color: "#007700",
          borderRadius: "4px",
          border: "2px solid #00ff00",
          padding: `${padding}px`,
          outline: "none",
        }}
      />
    </div>
  );
};
