import { useState, useEffect } from "react";
import "./MenuBar.css";

const MenuBar = () => {
  // State to track which dropdown is open
  const [openMenu, setOpenMenu] = useState<string | null>(null);

  // Toggle dropdown visibility
  const toggleMenu = (menu: string) => {
    console.log("Clicked:", menu);
    setOpenMenu(openMenu === menu ? null : menu);
  };

  // Close dropdown when clicking outside
  const handleClickOutside = (e: MouseEvent): void => {
    if (!(e.target as HTMLElement).closest(".menu-item")) {
      setOpenMenu(null);
    }
  };

  useEffect(() => {
    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  }, []);

  return (
    <div className="menu-bar">
      {/* File Menu */}
      <div className="menu-item">
        <div className="menu-button" onClick={() => toggleMenu("file")}>
          File
        </div>
        {openMenu === "file" && (
          <div className="dropdown">
            <div className="dropdown-item">New</div>
            <div className="dropdown-item">Open</div>
            <div className="dropdown-item">Save</div>
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
          </div>
        )}
      </div>

      {/* Exit Button */}
      <div className="exit-button">Exit</div>
    </div>
  );
};

export default MenuBar;
