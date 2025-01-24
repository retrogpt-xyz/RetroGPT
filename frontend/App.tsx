import { useState, useEffect } from "react";
import "./App.css";
import MusicPlayer from "./MusicPlayer";

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
  const [_sessToken, setSessToken] = useState("");

  const get_def_sess = async () => {
    const resp = await fetch("/api/get_def_sess", { method: "GET" });
    let body = await resp.text();
    setSessToken(body);
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
      <div>
        <MusicPlayer />
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
        {[
          "https://64.media.tumblr.com/3ea96a37f9c508e9c7ca7f95c2d9e5c6/32f4c776e65ab1bc-a7/s540x810/7e9ac2c7bcb1c31e20ca09649e7d96fb09982fd8.png",
          "https://64.media.tumblr.com/0d181187c50fedc1c60d1a6c3dd2165d/ec299322d93fd773-53/s540x810/afd900c44adfac375f08a490df747be6384c17d6.png",
          "https://64.media.tumblr.com/42e2b6779cbb09f0bf4ec645560be93f/9d46196f98fe3bc0-93/s540x810/6c3f4bf1a3069443c09f0751cb7375e5ebde98a2.png",
          "https://64.media.tumblr.com/ee4555102b26dc11494796658aef2196/2c2dac95a062501a-88/s540x810/14fabdd9ba87d3855cd9e07a4a8e298240c06c32.png",
          "https://64.media.tumblr.com/813967cfcf02a55d9b1d0dfd1aaff757/4dc8e55108cf74d2-d8/s540x810/0108220e0d1be29cd3b35392fe0da2d395e0c2f8.png",
          "https://64.media.tumblr.com/3348cb2690edd69e4abef37e181df74d/a805f4b239e74093-b6/s540x810/18d6a7c2de480930d0a2fc78916458fcc4e25b52.png",
        ].map((url, index) => (
          <div
            className="app-icon"
            key={index}
            style={{
              backgroundImage: `url(${url})`,
              backgroundSize: "cover",
              backgroundPosition: "center",
            }}
          ></div>
        ))}
      </div>
    </div>
  );
}

export default App;
