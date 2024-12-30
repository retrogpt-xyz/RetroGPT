import { useState } from "react";
import { ChatDisplay } from "./ChatDisplay";
import { InputBox } from "./InputBox";

const queryChat = (prompt: string): string => {
  const xhr = new XMLHttpRequest();
  xhr.open("POST", "/api/gpt", false); // false makes it a blocking request
  xhr.setRequestHeader("Content-Type", "text/plain");
  try {
    xhr.send(prompt);
    if (xhr.status !== 200) {
      throw new Error("Network response was not ok");
    }
    return xhr.responseText;
  } catch (error) {
    console.error("Error querying GPT:", error);
    throw error;
  }
};

export const MainScreenInner = () => {
  const promptPrefix = "$ ";
  const [msgs, setMsgs] = useState<string[]>([]);
  const handleSubmit = (txt: string) => {
    setMsgs([...msgs, promptPrefix + txt, queryChat(txt)]);
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
