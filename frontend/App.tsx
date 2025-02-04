import { useState, useEffect } from "react";
import "./App.css";

import MusicPlayer from "./MusicPlayer";
import { googleLogout, useGoogleLogin } from "@react-oauth/google";
import axios from "axios";
import MenuBar from "./MenuBar";

interface DisplayMessage {
  text: string;
  sender: "user" | "ai";
}

interface BackendQueryMessage {
  text: string;
  chatId: number | null;
  sessionToken: string;
}

interface User {
  access_token: string;
}

interface Profile {
  id: string;
  picture: string;
  name: string;
  email: string;
}

function App() {
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });

  const [displayMessages, setDisplayMessages] = useState<DisplayMessage[]>([]);
  const [inputMessage, setInputMessage] = useState("");
  const [chatId, setChatId] = useState<number | null>(null);

  const [userAccessToken, setUserAccessToken] = useState<User | null>(null);
  const [profile, setProfile] = useState<Profile | null>(null);

  const [sessToken, setSessToken] = useState<string | null>(null);
  const [userId, setUserId] = useState<number | null>(null);

  const [userOwnedChats, setUserOwnedChats] = useState<
    { id: number; name: string }[]
  >([]);

  const displayLoginOpts = false;

  useEffect(() => {
    if (!sessToken) {
      setUserOwnedChats([]);
      return;
    }
    if (!userId) {
      setUserOwnedChats([]);
      return;
    }

    fetch("/api/chats", {
      body: userId.toString(),
      method: "POST",
      headers: {
        "X-Session-Token": sessToken,
        "Content-Type": "application/json",
      },
    }).then(async (resp) => {
      if (resp.status != 200) return;

      const body = await resp.json();
      setUserOwnedChats(body);
    });
  }, [userId, sessToken, chatId]);

  const getSessionToken = async () => {
    if (!userId) {
      const resp = await fetch("/api/get_def_sess", { method: "GET" });
      const body = await resp.text();
      setSessToken(body);
      return;
    }

    await fetch("/api/session", {
      method: "POST",
      body: JSON.stringify(userId),
    }).then(async (resp) => {
      const sessionToken = await resp.text();
      console.log(sessionToken);
      setSessToken(sessionToken);
    });
  };

  const login = useGoogleLogin({
    onSuccess: setUserAccessToken,
  });

  const syncMessages = async () => {
    if (!chatId) {
      setDisplayMessages([]);
      return;
    }

    if (!sessToken) {
      setDisplayMessages([]);
      return;
    }

    fetch("/api/chat/messages", {
      method: "POST",
      headers: {
        "X-Session-Token": sessToken,
      },
      body: chatId.toString(),
    }).then(async (resp) => {
      const msgs: DisplayMessage[] = JSON.parse(await resp.text());
      console.log(msgs);
      console.log(displayMessages);
      if (JSON.stringify(msgs) === JSON.stringify(displayMessages)) {
        return;
      }
      console.log("got different messages from the server");
      setDisplayMessages(msgs);
    });
  };

  useEffect(() => {
    syncMessages();
  }, [chatId]);

  useEffect(() => {
    getSessionToken();
  }, [userId]);

  useEffect(() => {
    setDisplayMessages([]);
    setChatId(null);
  }, [sessToken]);

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

  useEffect(() => {
    if (userAccessToken) {
      axios
        .get(
          `https://www.googleapis.com/oauth2/v1/userinfo?access_token=${userAccessToken.access_token}`,
          {
            headers: {
              Authorization: `Bearer ${userAccessToken.access_token}`,
              Accept: "application/json",
            },
          },
        )
        .then(async (res) => {
          console.log(res.data);
          const profile: Profile = res.data;
          setProfile(profile);
          const body = {
            google_id: profile.id,
            email: profile.email,
            name: profile.name,
          };

          await fetch("/api/auth", {
            method: "POST",
            body: JSON.stringify(body),
          }).then(async (resp) => {
            const user_id: number = JSON.parse(await resp.text());
            console.log(user_id);
            setUserId(user_id);
          });
        })
        .catch((err) => console.log(err));
    }
  }, [userAccessToken]);

  const fetchAIResponse = async (msg: BackendQueryMessage) => {
    const headers: HeadersInit = {
      "Content-Type": "application/json",
    };
    if (sessToken) {
      headers["X-Session-Token"] = sessToken;
    }

    const response = await fetch("/api/prompt", {
      method: "POST",
      body: JSON.stringify(msg),
      headers,
    });

    const reader = response.body?.getReader();
    if (!reader) {
      console.error("Failed to get reader from response body.");
      return;
    }
    const decoder = new TextDecoder("utf-8");
    let aiResponse = "";

    // Append a new message for streaming
    setDisplayMessages((prev) => [
      ...prev,
      { text: "...", sender: "ai" as const },
    ]);

    const messageIndex = displayMessages.length + 1;

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      aiResponse += decoder.decode(value, { stream: true });

      // Update the last message with the new data
      setDisplayMessages((prev) => {
        const updatedMessages = [...prev];
        updatedMessages[messageIndex] = {
          text: aiResponse,
          sender: "ai" as const,
        };
        return updatedMessages;
      });
    }

    const chatIdHeader = response.headers.get("X-Chat-ID");

    if (chatId) setTimeout(() => syncMessages(), 1000);

    if (chatIdHeader) {
      setChatId(parseInt(chatIdHeader));
    }
  };

  const handleSendMessage = async () => {
    if (!inputMessage.trim()) return;
    if (!sessToken) return;

    await syncMessages();

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
        <div>
          <MenuBar />
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

      {/* Right column with app icons */}
      <div className="app-column">
        {displayLoginOpts &&
          (profile ? (
            <>
              <p>{profile.name}</p>
              <img src={profile.picture} alt="Profile" />
              <button
                onClick={() => {
                  setProfile(null);
                  setUserId(null);
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
