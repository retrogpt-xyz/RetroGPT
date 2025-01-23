import { useState, useEffect } from "react";
import "./App.css";

interface DisplayMessage {
  text: string;
  sender: "user" | "ai";
}

interface BackendQueryMessage {
  text: string;
  chatId: number | null;
}

function App() {
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });
  const [displayMessages, setDisplayMessages] = useState<DisplayMessage[]>([]);
  const [inputMessage, setInputMessage] = useState("");
  const [chatId, setChatId] = useState<number | null>(null);

  // TODO: Implement session token (stok) validation
  const [_stok, setStok] = useState("");

  const get_def_sess = async () => {
    const resp = await fetch("/api/get_def_sess", { method: "GET" });
    let body = await resp.text();
    setStok(body);
  };

  useEffect(() => {
    get_def_sess();
  }, []);

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

  const fetchAIResponse = async (msg: BackendQueryMessage) => {
    const response = await fetch("/api/prompt", {
      method: "POST",
      body: JSON.stringify(msg),
    });

    const body = await response.text();
    let parsed: BackendQueryMessage = JSON.parse(body);

    const aiResponse = parsed.text;

    setDisplayMessages((prev) => [...prev, { text: aiResponse, sender: "ai" }]);
    setChatId(parsed.chatId);
  };

  const handleSendMessage = () => {
    if (!inputMessage.trim()) return;

    setDisplayMessages((prev) => [
      ...prev,
      { text: inputMessage, sender: "user" },
    ]);

    const be_query_msg: BackendQueryMessage = {
      text: inputMessage,
      chatId: chatId,
    };

    fetchAIResponse(be_query_msg);

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
              {displayMessages.map((message, index) => (
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
