import { useState, useEffect, useRef } from "react";

const RightClick = () => {
  const [contextMenu, setContextMenu] = useState<{
    visible: boolean;
    x: number;
    y: number;
  }>({ visible: false, x: 0, y: 0 });

  const contextMenuRef = useRef<HTMLDivElement>(null);

  // Handle right-click event globally
  useEffect(() => {
    const handleContextMenu = (e: MouseEvent) => {
      e.preventDefault(); // Prevent the default browser context menu

      setContextMenu({
        visible: true,
        x: e.clientX,
        y: e.clientY,
      });
    };

    // Add the event listener to the document
    document.addEventListener("contextmenu", handleContextMenu);

    // Clean up the event listener
    return () => {
      document.removeEventListener("contextmenu", handleContextMenu);
    };
  }, []);

  // Close the context menu when clicking outside
  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (
        contextMenuRef.current &&
        !contextMenuRef.current.contains(e.target as Node)
      ) {
        setContextMenu({ visible: false, x: 0, y: 0 });
      }
    };

    document.addEventListener("click", handleClickOutside);
    return () => {
      document.removeEventListener("click", handleClickOutside);
    };
  }, []);

  return (
    <div>

      {/* Context Menu */}
      {contextMenu.visible && (
        <div
          ref={contextMenuRef}
          style={{
            position: "fixed", // Use fixed positioning to ensure it appears at the correct location
            top: contextMenu.y,
            left: contextMenu.x,
            backgroundColor: "#f0f0f0",
            border: "1px solid #ccc",
            boxShadow: "2px 2px 5px rgba(0, 0, 0, 0.2)",
            padding: "5px 0",
            zIndex: 1000,
          }}
        >
          <div
            className="context-menu-item"
          >
            About
          </div>
          <div
            className="context-menu-item"
          >
            Support
          </div>
          <div
            className="context-menu-item"
          >
            Inspect Element
          </div>
        </div>
      )}
    </div>
  );
};

export default RightClick;
