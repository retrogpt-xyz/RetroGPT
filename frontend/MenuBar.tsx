import { useState } from "react";

import "./MenuBar.css"

interface MenuBarProps {
  chatId: number | null;
  setChatId: (chatId: number | null) => void;
  userOwnedChats: { id: number; name: string }[];
  setUserOwnedChats: (chats: { id: number; name: string }[]) => void;
  sessToken: string | null;
  login: () => void; // <-- Add this
}

const MenuBar: React.FC<MenuBarProps> = ({
  chatId,
  setChatId,
  userOwnedChats,
  setUserOwnedChats,
  sessToken,
  login, // <-- Accept login function
}) => {
  const [openMenu, setOpenMenu] = useState<string | null>(null);

  const toggleMenu = (menu: string) => {
    setOpenMenu(openMenu === menu ? null : menu);
  };

  // Open an existing chat or create a new one
  const handleOpenChat = async () => {
    const chatName = prompt(
      "Enter Chat Name (or type a new one to create):"
    )?.trim();
    if (!chatName) return;

    // Check if chat exists
    const existingChat = userOwnedChats.find((chat) => chat.name === chatName);
    if (existingChat) {
      setChatId(existingChat.id);
      console.log(`Switched to chat: ${existingChat.name}`);
      return;
    }

    // Create new chat if not found
    if (!sessToken) {
      alert("Session token missing. Cannot create chat.");
      return;
    }

    const resp = await fetch("/api/create_chat", {
      method: "POST",
      headers: {
        "X-Session-Token": sessToken,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ name: chatName }),
    });

    if (resp.status === 200) {
      const newChat = await resp.json();
      setUserOwnedChats([...userOwnedChats, newChat]);
      setChatId(newChat.id);
      console.log(`Created and switched to chat: ${chatName}`);
    } else {
      console.error("Failed to create chat");
    }
  };

  // Save a message to the current chat
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
    <div className="menu-bar">
      {/* File Menu */}
      <div className="menu-item">
        <div className="menu-button" onClick={() => toggleMenu("file")}>
          File
        </div>
        {openMenu === "file" && (
          <div className="dropdown">
            <div className="dropdown-item"onClick={() => setChatId(null)}>New</div>
            <div className="dropdown-item"onClick={handleOpenChat}>Open</div>
            <div className="dropdown-item"onClick={handleSaveChat}>Save</div>
            <div className="dropdown-item">Exit</div>
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
            <div className="dropdown-item">Close</div>
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
  );
};

export default MenuBar;
