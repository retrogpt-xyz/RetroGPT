import React from "react";
import "./ChatReference.css";

export interface ChatReferenceProps {
  visible: boolean;
  chats: { id: number; name: string }[];
  onSelectChat: (id: number) => void;
}

const ChatReference: React.FC<ChatReferenceProps> = ({
  visible,
  chats,
  onSelectChat,
}) => {
  if (!visible) return null;

  return (
    <div className="chat-reference">
      {chats.map((chat) => (
        <div
          key={chat.id}
          className="chat-ref-item"
          onClick={() => onSelectChat(chat.id)}
        >
          {/* placeholder “+” icon */}
          <img
            src="https://via.placeholder.com/12x12?text=%2B"
            alt="+"
            className="chat-ref-plus"
          />
          {/* placeholder chat‐bubble icon */}
          <img
            src="https://via.placeholder.com/16x16?text=%F0%9F%92%AC"
            alt="chat"
            className="chat-ref-icon"
          />
          <span className="chat-ref-name">{chat.name}</span>
        </div>
      ))}
    </div>
  );
};

export default ChatReference;
