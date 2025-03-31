import { useState, useEffect } from "react";
import { googleLogout, useGoogleLogin } from "@react-oauth/google";
import { get_api_host } from "./request";

import "./App.css";
import MenuBar from "./MenuBar";
import Dock from "./Dockbar";
import RightClick from "./RightClickMenu";
import { auth } from "./auth";
import { format_api_request_url } from "./request";

interface DisplayMessage {
  text: string;
  sender: "user" | "ai";
}

interface BackendQueryMessage {
  text: string;
  chatId: number | null;
  sessionToken: string;
}

function App() {
  const [windowVisible, setWindowVisible] = useState(true);

  const [displayMessages, setDisplayMessages] = useState<DisplayMessage[]>([]);
  const [inputMessage, setInputMessage] = useState("");
  const [chatId, setChatId] = useState<number | null>(null);

  const [sessToken, setSessToken] = useState<string>("__default__");
  const [userId, setUserId] = useState<number | null>(null);

  const [userOwnedChats, setUserOwnedChats] = useState<
    { id: number; name: string }[]
  >([]);

  const displayLoginOpts = false;

  const syncUserOwnedChats = async () => {
    if (sessToken == "__default__" || !userId) {
      setUserOwnedChats([]);
      return;
    }

    const resp = await fetch(format_api_request_url("v0.0.1/user_chats"), {
      method: "POST",
      body: JSON.stringify({ user_id: userId }),
      headers: {
        "X-Session-Token": sessToken,
        "Content-Type": "application/json",
      },
    });
    if (resp.status !== 200) return;
    const body = await resp.json();
    setUserOwnedChats(body);
  };

  useEffect(() => {
    syncUserOwnedChats();
  }, [userId, sessToken, chatId]);

  const login = useGoogleLogin({
    onSuccess: async (user_access_token) => {
      const authResult = await auth(user_access_token);
      if (authResult) {
        setSessToken(authResult.sessionToken);
        setUserId(authResult.userId);
      }
    },
  });

  const syncMessages = async () => {
    if (!chatId) {
      setDisplayMessages([]);
      return;
    }

    fetch(format_api_request_url("v0.0.1/chat_msgs"), {
      method: "POST",
      headers: {
        "X-Session-Token": sessToken,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ chat_id: chatId }),
    }).then(async (resp) => {
      const msgs: DisplayMessage[] = JSON.parse(await resp.text());
      console.log(msgs);
      console.log(displayMessages);
      if (JSON.stringify(msgs) === JSON.stringify(displayMessages)) {
        return;
      }
      console.log("got different messages from the server");
      let chid = resp.headers.get("X-Chat-ID");
      if (chid) {
        if (JSON.parse(chid) == chatId) setDisplayMessages(msgs);
      }
    });
  };

  useEffect(() => {
    syncMessages();
  }, [chatId]);

  useEffect(() => {
    setDisplayMessages([]);
    setChatId(null);
  }, [sessToken]);

  const fetchAIResponse = async (msg: BackendQueryMessage) => {
    setDisplayMessages((prev) => [
      ...prev,
      { text: "...", sender: "ai" as const },
    ]);

    const headers: HeadersInit = {
      "Content-Type": "application/json",
    };
    headers["X-Session-Token"] = sessToken;

    // Step 1: Create the response stream via the semver'd prompt endpoint
    const response = await fetch(format_api_request_url("v0.0.1/prompt"), {
      method: "POST",
      body: JSON.stringify({
        text: msg.text,
        chat_id: msg.chatId,
      }),
      headers,
    });

    if (!response.ok) {
      console.error("Failed to create prompt stream");
      return;
    }

    const { chat_id, attach_token } = await response.json();

    // Build WebSocket URL with protocol, host, endpoint, and session token
    const wsProtocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsHost = get_api_host();
    const wsEndpoint = `/api/v0.0.1/attach/${attach_token}`;
    const wsQuery = `?token=${encodeURIComponent(sessToken)}`;
    const ws_url = `${wsProtocol}//${wsHost}${wsEndpoint}${wsQuery}`;

    const ws = new WebSocket(ws_url);

    const messageIndex = displayMessages.length + 1;
    let aiResponse = "";

    ws.onmessage = (event) => {
      const chunk = event.data;
      aiResponse += chunk;

      // Update the last message with the new data
      setDisplayMessages((prev) => {
        const updatedMessages = [...prev];
        updatedMessages[messageIndex] = {
          text: aiResponse,
          sender: "ai" as const,
        };
        return updatedMessages;
      });
    };

    ws.onclose = async () => {
      // Step 3: Append the complete response to chat
      await fetch(format_api_request_url("v0.0.1/append_to_chat"), {
        method: "POST",
        body: JSON.stringify({
          sender: "ai",
          body: aiResponse,
          chat_id: chat_id,
        }),
        headers,
      });

      await syncUserOwnedChats();
      setChatId(chat_id);
    };

    ws.onerror = (error) => {
      console.error("WebSocket error:", error);
    };
  };

  const handleSendMessage = async () => {
    if (!inputMessage.trim()) return;

    setDisplayMessages((prev) => [
      ...prev,
      { text: inputMessage, sender: "user" },
    ]);

    const be_query_msg: BackendQueryMessage = {
      text: inputMessage,
      chatId: chatId,
      sessionToken: sessToken,
    };

    fetchAIResponse(be_query_msg);

    setInputMessage("");
  };

  return (
    <div className="retro-wrapper">
      <div>
        <Dock />
      </div>

      <div>
        <RightClick />
      </div>

      {/* Center window - Chat Interface */}
      {windowVisible && ( // <-- Conditionally render main window
        <div className="main-window">
          <div>
            <MenuBar
              setChatId={setChatId}
              userOwnedChats={userOwnedChats} // Pass the state value, not the setter
              login={login}
              setWindowVisible={setWindowVisible}
              syncUserOwnedChats={syncUserOwnedChats}
            />
          </div>
          <div className="header-bar">WELCOME TO RETROGPT</div>
          <div className="header-under">How can I help?</div>
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
                  onKeyDown={(e) => {
                    if (e.key === "Enter") {
                      handleSendMessage();
                    }
                  }}
                  placeholder="Type your message..."
                />
                <button onClick={handleSendMessage}>Send</button>
              </div>
            </div>
          </div>
        </div>
      )}
      {/* Right column with app icons */}
      <div className="app-column">
        {displayLoginOpts &&
          (sessToken != "__default__" ? (
            <>
              <button
                onClick={() => {
                  setSessToken("__default__");
                  googleLogout();
                }}
              >
                {"logout"}
              </button>
            </>
          ) : (
            <button onClick={() => login()}>login</button>
          ))}

        {displayLoginOpts && (
          <button onClick={() => setChatId(null)}>new chat</button>
        )}

        <div>
          {displayLoginOpts &&
            userOwnedChats.map(({ id: id, name: name }) => (
              <button key={id} onClick={() => setChatId(id)}>
                {name}
              </button>
            ))}
        </div>
        <div className="app-column">
          {[
            {
              url: "https://64.media.tumblr.com/3ea96a37f9c508e9c7ca7f95c2d9e5c6/32f4c776e65ab1bc-a7/s540x810/7e9ac2c7bcb1c31e20ca09649e7d96fb09982fd8.png",
              name: "Music",
            },
            {
              url: "https://64.media.tumblr.com/0d181187c50fedc1c60d1a6c3dd2165d/ec299322d93fd773-53/s540x810/afd900c44adfac375f08a490df747be6384c17d6.png",
              name: "RetroGPT",
              onClick: () => {
                setWindowVisible((prev) => !prev); // Toggle window visibility
              },
            },
            {
              url: "https://64.media.tumblr.com/42e2b6779cbb09f0bf4ec645560be93f/9d46196f98fe3bc0-93/s540x810/6c3f4bf1a3069443c09f0751cb7375e5ebde98a2.png",
              name: "Pages",
            },
            {
              url: "https://64.media.tumblr.com/ee4555102b26dc11494796658aef2196/2c2dac95a062501a-88/s540x810/14fabdd9ba87d3855cd9e07a4a8e298240c06c32.png",
              name: "Reader",
            },
            {
              url: "https://64.media.tumblr.com/813967cfcf02a55d9b1d0dfd1aaff757/4dc8e55108cf74d2-d8/s540x810/0108220e0d1be29cd3b35392fe0da2d395e0c2f8.png",
              name: "Print",
            },
            {
              url: "https://64.media.tumblr.com/3348cb2690edd69e4abef37e181df74d/a805f4b239e74093-b6/s540x810/18d6a7c2de480930d0a2fc78916458fcc4e25b52.png",
              name: "Finder",
            },
          ].map(({ url, name, onClick }, index) => (
            <button
              key={index}
              className="app-icon"
              onClick={onClick} // Add the click handler
              style={{ cursor: "pointer", border: "none", background: "none" }} // Remove default button styles
            >
              <div
                style={{
                  backgroundImage: `url(${url})`,
                  backgroundSize: "cover",
                  backgroundPosition: "center",
                  width: "75px",
                  height: "75px",
                }}
              ></div>
              <span className="app-icon-label">{name}</span>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;
