import { useState } from "react";
import { ChatDisplay } from "./ChatDisplay";
import { InputBox } from "./InputBox";

const queryChat = (_: string) => {
  return "some response from chatgpt";
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
