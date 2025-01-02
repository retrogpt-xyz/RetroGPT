import { useState } from "react";
import { ChatDisplay } from "./ChatDisplay";
import { InputBox } from "./InputBox";

const promptBackend = async (prompt: string): Promise<string> => {
  try {
    const response = await fetch("/api/prompt", {
      method: "POST",
      body: prompt,
    });

    if (!response.ok) {
      return `Error: Server returned status ${response.status}`;
    }

    return await response.text();
  } catch (error) {
    console.error("Error in promptBackend:", error);
    return "Error: Failed to communicate with server";
  }
};

export const MainScreenInner = () => {
  const promptPrefix = "$ ";
  const [msgs, setMsgs] = useState<string[]>([]);
  const handleSubmit = async (txt: string) => {
    setMsgs([...msgs, promptPrefix + txt]);
    const response = await promptBackend(txt);
    setMsgs((prev) => [...prev, response]);
  };
  return (
    <div
      style={{
        width: "80%",
        height: "80%",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        border: "2px solid green",
        borderRadius: "8px",
        padding: "20px",
        background: "linear-gradient(to bottom, #000000, #001700)",
        gap: "20px",
      }}
    >
      <ChatDisplay msgs={msgs} />
      <InputBox onSubmit={handleSubmit} />
    </div>
  );
};
