import React, { useState, useEffect } from "react";
import "./App.css";

function App() {
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });
  const [messages, setMessages] = useState<{ text: string; sender: "user" | "ai" }[]>([]);
  const [inputMessage, setInputMessage] = useState("");

  const handleMouseMove = (event: MouseEvent) => {
    setMousePosition({
      x: event.clientX,
      y: event.clientY,
    });
  };

  useEffect(() => {
    window.addEventListener("mousemove", handleMouseMove);
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
    };
  }, []);

  const handleSendMessage = () => {
    if (!inputMessage.trim()) return;

    setMessages((prev) => [...prev, { text: inputMessage, sender: "user" }]);

    // Placeholder for backend call
    const fetchAIResponse = async () => {
      const aiResponse = `AI response to "${inputMessage}"`; // For riley to replace with backend API call
      setMessages((prev) => [...prev, { text: aiResponse, sender: "ai" }]);
    };

    fetchAIResponse();
    setInputMessage("");
  };

  return (
    <div className="retro-wrapper">
      {/* Invisible custom cursor */}
      <div
        className="custom-cursor-layer"
        style={{
          transform: `translate(${mousePosition.x}px, ${mousePosition.y}px)`,
        }}
      ></div>

      {/* Leftside music player */}
      <div className="music-player">
        <h2>Music Player</h2>
        <p>Now Playing: [Track Placeholder]</p>
        <button>Play</button>
      </div>

      {/* Center window - Chat Interface */}
      <div className="main-window">
        <div className="header-bar"></div>
        <div className="content-area">
          <div className="chat-window">
            <div className="chat-messages">
              {messages.map((message, index) => (
                <div
                  key={index}
                  className={`chat-message ${
                    message.sender === "user" ? "user-message" : "ai-message"
                  }`}
                >
                  {message.text}
                </div>
              ))}
            </div>
            <div className="chat-input">
              <input
                type="text"
                value={inputMessage}
                onChange={(e) => setInputMessage(e.target.value)}
                placeholder="Type your message..."
              />
              <button onClick={handleSendMessage}>Send</button>
            </div>
          </div>
        </div>
      </div>

      {/* Right column with app icons */}
      <div className="app-column">
        {Array.from({ length: 6 }).map((_, index) => (
          <div className="app-icon" key={index}></div>
        ))}
      </div>
    </div>
  );
}

export default App;