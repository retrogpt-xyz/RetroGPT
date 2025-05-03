import React, { useState } from "react";
import "./FileExplorer.css";
React;
interface FileItem {
  name: string;
  type: "folder" | "file";
  children?: FileItem[];
}

interface FileExplorerProps {
  visible: boolean;
  onClose: () => void;
}

const FileExplorer: React.FC<FileExplorerProps> = ({ visible, onClose }) => {
  const [currentPath, setCurrentPath] = useState<string[]>(["Home"]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  const fileStructure: Record<string, FileItem[]> = {
    "Home": [
      { name: "Desktop", type: "folder" },
      { name: "Documents", type: "folder" },
      { name: "Downloads", type: "folder" },
      { name: "Pictures", type: "folder" },
      { name: "Music", type: "folder" },
      { name: "Videos", type: "folder" },
      { name: "Chats", type: "folder" },
    ],
    "Desktop": [
      { name: "Recycle Bin", type: "folder" },
      { name: "My Computer", type: "folder" },
      { name: "Network", type: "folder" },
      { name: "Notes.txt", type: "file" },
      { name: "Resume.doc", type: "file" },
      { name: "Presentation.ppt", type: "file" },
      { name: "Budget.xls", type: "file" },
    ],
    "Documents": [
      { name: "Work", type: "folder" },
      { name: "Personal", type: "folder" },
      { name: "report.pdf", type: "file" },
      { name: "letter.doc", type: "file" },
      { name: "contract.docx", type: "file" },
      { name: "notes.txt", type: "file" },
    ],
    "Chats": [
      { name: "General", type: "folder" },
      { name: "Private", type: "folder" },
      { name: "Archive", type: "folder" },
      { name: "chat_log_0429.txt", type: "file" },
      { name: "settings.ini", type: "file" },
      { name: "backup.zip", type: "file" },
    ],
    "General": [
      { name: "welcome.txt", type: "file" },
      { name: "rules.txt", type: "file" },
      { name: "faq.pdf", type: "file" },
    ],
    "Private": [
      { name: "user1_chat.log", type: "file" },
      { name: "user2_chat.log", type: "file" },
      { name: "meeting_notes.doc", type: "file" },
    ],
  };

  const getCurrentFolder = (): FileItem[] => {
    const currentFolderName = currentPath[currentPath.length - 1];
    return fileStructure[currentFolderName] || [];
  };

  const navigateToFolder = (folderName: string) => {
    setCurrentPath([...currentPath, folderName]);
    setSelectedFile(null);
  };

  const navigateUp = () => {
    if (currentPath.length > 1) {
      setCurrentPath(currentPath.slice(0, -1));
      setSelectedFile(null);
    }
  };

  const handleFileClick = (fileName: string) => {
    setSelectedFile(fileName);
  };

  if (!visible) return null;

  const currentItems = getCurrentFolder();
  const folders = currentItems.filter(item => item.type === "folder");
  const files = currentItems.filter(item => item.type === "file");

  return (
    <div className="popup-overlay">
      <div className="popup-window file-explorer">
        <div className="popup-header">
          <span>File Explorer - {currentPath.join(" > ")}</span>
          <button className="exit-button" onClick={onClose}>
            X
          </button>
        </div>

        <div className="file-explorer-toolbar">
          <button onClick={navigateUp} className="toolbar-button">
            Up
          </button>
          <span className="path-display">{currentPath.join(" > ")}</span>
        </div>

        <div className="file-explorer-content">
          {/* Folders section */}
          <div className="folders-section">
            {folders.map((item) => (
              <div
                key={item.name}
                className={`file-item folder ${
                  selectedFile === item.name ? "selected" : ""
                }`}
                onClick={() => navigateToFolder(item.name)}
              >
                <img
                  src="https://64.media.tumblr.com/3c645dd8a0ef3647d4e7cd76564b2136/ff2e44bf204fefea-47/s540x810/3b2c927cf2ce25f62ca2ed7aae557c4237b307bc.png"
                  alt="Folder"
                  className="file-icon"
                />
                <span>{item.name}</span>
              </div>
            ))}
          </div>

          {/* Files section - horizontal layout */}
          <div className="files-section">
            {files.map((item) => (
              <div
                key={item.name}
                className={`file-item file ${
                  selectedFile === item.name ? "selected" : ""
                }`}
                onClick={() => handleFileClick(item.name)}
              >
                <img
                  src="https://64.media.tumblr.com/18c2a365f1c29f9ac87483aa28bec32a/7278b44044d05aac-19/s540x810/b6652a087de2d49c1a45251d3b0ba18ea07cc965.png"
                  alt="File"
                  className="file-icon"
                />
                <span>{item.name}</span>
              </div>
            ))}
          </div>
        </div>

        <div className="status-bar">
          {selectedFile
            ? `Selected: ${selectedFile}`
            : `${currentItems.length} items (${folders.length} folders, ${files.length} files)`}
        </div>
        <div className="resize-handle"></div>
      </div>
    </div>
  );
};

export default FileExplorer;
