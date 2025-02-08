import { useState } from "react";
import "./MenuBar.css";

interface MenuBarProps {
  chatId: number | null;
  setChatId: (chatId: number | null) => void;
  userOwnedChats: { id: number; name: string }[];
  setUserOwnedChats: (chats: { id: number; name: string }[]) => void;
  sessToken: string | null;
  login: () => void;
  setWindowVisible: (visible: boolean) => void; // <-- New prop to control visibility
}

const MenuBar: React.FC<MenuBarProps> = ({
  chatId,
  setChatId,
  userOwnedChats,
  sessToken,
  login,
  setWindowVisible,
}) => {
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const [showPopup, setShowPopup] = useState(false);

  const toggleMenu = (menu: string) => {
    setOpenMenu(openMenu === menu ? null : menu);
  };

  const handleOpenChat = () => {
    setShowPopup(true); // Show the chat selection popup
  };

  const selectChat = (id: number) => {
    setChatId(id);
    setShowPopup(false);
  };

  const handleSaveChat = async () => {
    if (!chatId) {
      alert("No chat is open. Open or create a chat first.");
      return;
    }

    const message = prompt("Enter message to save:");
    if (!message) return;

    if (!sessToken) {
      alert("Session token missing. Cannot save message.");
      return;
    }

    const resp = await fetch("/api/chat/messages", {
      method: "POST",
      headers: {
        "X-Session-Token": sessToken,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ chatId, text: message }),
    });

    if (resp.status === 200) {
      console.log(`Message saved to chat ${chatId}`);
    } else {
      console.error("Failed to save message");
    }
  };

  return (
    <>
      <div className={`menu-bar ${showPopup ? "blur" : ""}`}>
        {/* File Menu */}
        <div className="menu-item">
          <div className="menu-button" onClick={() => toggleMenu("file")}>
            File
          </div>
          {openMenu === "file" && (
            <div className="dropdown">
              <div className="dropdown-item" onClick={() => setChatId(null)}>New</div>
              <div className="dropdown-item" onClick={handleOpenChat}>Open</div>
              <div className="dropdown-item" onClick={handleSaveChat}>Save</div>
            </div>
          )}
        </div>

        {/* Edit Menu */}
        <div className="menu-item">
          <div className="menu-button" onClick={() => toggleMenu("edit")}>
            Edit
          </div>
          {openMenu === "edit" && (
            <div className="dropdown">
              <div className="dropdown-item">Undo</div>
              <div className="dropdown-item">Redo</div>
              <div className="dropdown-item">Cut</div>
              <div className="dropdown-item">Copy</div>
              <div className="dropdown-item">Paste</div>
            </div>
          )}
        </div>

        {/* Window Menu */}
        <div className="menu-item">
          <div className="menu-button" onClick={() => toggleMenu("window")}>
            Window
          </div>
          {openMenu === "window" && (
            <div className="dropdown">
              <div className="dropdown-item">Minimize</div>
              <div className="dropdown-item">Maximize</div>
              <div className="dropdown-item" onClick={() => setWindowVisible(false)}>Close</div>
            </div>
          )}
        </div>

        {/* Save Menu */}
        <div className="menu-item">
          <div className="menu-button" onClick={() => toggleMenu("save")}>
            Save
          </div>
          {openMenu === "save" && (
            <div className="dropdown">
              <div className="dropdown-item">Save As...</div>
              <div className="dropdown-item">Export</div>
              <div className="dropdown-item" onClick={() => login()}>Login</div>
            </div>
          )}
        </div>

        {/* Exit Button */}
        <div className="exit-button">Exit</div>
      </div>

      {/* Popup Window for Chat Selection */}
      {showPopup && (
        <div className="popup-overlay" onClick={() => setShowPopup(false)}>
          <div className="popup-window" onClick={(e) => e.stopPropagation()}>
            <div className="popup-header">
              <span>Select a Chat</span>
              <button className="close-button" onClick={() => setShowPopup(false)}>X</button>
            </div>
            <div className="popup-content">
              {userOwnedChats.length > 0 ? (
                userOwnedChats.map((chat) => (
                  <div
                    key={chat.id}
                    className="chat-item"
                    onClick={() => selectChat(chat.id)}
                  >
                    {chat.name}
                  </div>
                ))
              ) : (
                <p>No chats available.</p>
              )}
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default MenuBar;