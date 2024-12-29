import { ChatDisplay } from "./ChatDisplay";
import { InputBox } from "./InputBox";

export const MainScreenInner = () => {
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
      <ChatDisplay />
      <InputBox onSubmit={(msg) => alert(msg)} />
    </div>
  );
}; 