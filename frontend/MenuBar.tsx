import { useState } from "react";
import "./MenuBar.css";

interface MenuBarProps {
  setChatId: (chatId: number | null) => void;
  userOwnedChats: { id: number; name: string }[];
  login: () => void;
  logout: () => void;
  setWindowVisible: (visible: boolean) => void;
  syncUserOwnedChats: () => void;
}

const MenuBar: React.FC<MenuBarProps> = ({
  setChatId,
  userOwnedChats,
  login,
  logout, 
  setWindowVisible,
  syncUserOwnedChats,
}) => {
  const [openMenu, setOpenMenu] = useState<
    "file" | "edit" | "window" | "save" | null
  >(null);
  const [showPopup, setShowPopup] = useState(false);

  const toggleMenu = (menu: "file" | "edit" | "window" | "save") => {
    setOpenMenu(openMenu === menu ? null : menu);
  };

  const handleOpenChat = () => {
    syncUserOwnedChats(); // Sync the chats before showing the popup
    setShowPopup(true); // Show the chat selection popup
  };

  const selectChat = (id: number) => {
    setChatId(id);
    setShowPopup(false);
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
              <div className="dropdown-item" onClick={() => setChatId(null)}>
                New
              </div>
              <div className="dropdown-item" onClick={handleOpenChat}>
                Open
              </div>
              <div
                className="dropdown-item"
                onClick={() => {
                  // todo...  ??? why does this exist
                }}
              >
                Save
              </div>
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
              <div
                className="dropdown-item"
                onClick={() => setWindowVisible(false)}
              >
                Close
              </div>
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
              <div className="dropdown-item" onClick={() => login()}>
                Login
              </div>
   <div className="dropdown-item" onClick={() => logout()}>
     Logout
   </div>
            </div>
          )}
        </div>

        {/* Exit Button */}
        <div className="exit-button" onClick={() => setWindowVisible(false)}>
          Exit
        </div>
      </div>

      {/* Popup Window for Chat Selection */}
      {showPopup && (
        <div className="popup-overlay" onClick={() => setShowPopup(false)}>
          <div className="popup-window" onClick={(e) => e.stopPropagation()}>
            <div className="popup-header">
              <span>Select a Chat</span>
              <button
                className="close-button"
                onClick={() => setShowPopup(false)}
              >
                X
              </button>
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
