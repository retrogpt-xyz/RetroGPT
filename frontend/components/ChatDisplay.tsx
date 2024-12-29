import chatlogo from "../assets/chatlogo.png";
import { Typewriter } from "./TypeWriter";

export interface ChatDisplayProps {
  msgs: string[];
}

export const ChatDisplay = ({ msgs }: ChatDisplayProps) => {
  return (
    <div
      style={{
        flex: "8",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        flexDirection: "column",
        height: "100%",
        width: "100%",
        overflow: "hidden",
      }}
    >
      {msgs.length === 0 ? <LogoDisplay /> : <MessageList msgs={msgs} />}
    </div>
  );
};

const MessageList = ({ msgs }: { msgs: string[] }) => {
  return (
    <div
      style={{
        width: "100%",
        height: "100%",
        padding: "1rem",
        overflowY: "auto",
      }}
    >
      {msgs.map((msg, index) => (
        <div
          key={index}
          style={{
            marginBottom: "1rem",
            whiteSpace: "pre-wrap",
            wordBreak: "break-word",
          }}
        >
          <Typewriter text={msg}/>
        </div>
      ))}
    </div>
  );
};

const LogoDisplay = () => {
  return (
    <div
      style={{
        width: "100%",
        height: "100%",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        overflow: "hidden",
      }}
    >
      <img
        src={chatlogo}
        style={{
          width: "100%",
          height: "100%",
          objectFit: "contain",
        }}
      />
    </div>
  );
};
