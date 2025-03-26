// Dock.tsx
import { useState } from "react";
import "./Dockbar.css";

interface DockItem {
  id: number;
  icon: string;
}

interface Props {
  onAppSelect: (appName: string) => void;
}

const Dock = ({ onAppSelect }: Props) => {
  const [selectedApp, setSelectedApp] = useState<number | null>(null);

  const dockItems: DockItem[] = [
    {
      id: 1,
      icon: "https://64.media.tumblr.com/3ea96a37f9c508e9c7ca7f95c2d9e5c6/32f4c776e65ab1bc-a7/s540x810/7e9ac2c7bcb1c31e20ca09649e7d96fb09982fd8.png"
    },
    {
      id: 2,
      icon: "https://64.media.tumblr.com/0d181187c50fedc1c60d1a6c3dd2165d/ec299322d93fd773-53/s540x810/afd900c44adfac375f08a490df747be6384c17d6.png"
    },
    {
      id: 3,
      icon: "https://64.media.tumblr.com/42e2b6779cbb09f0bf4ec645560be93f/9d46196f98fe3bc0-93/s540x810/6c3f4bf1a3069443c09f0751cb7375e5ebde98a2.png"
    },
    // Add more dock items as needed fr
  ];

  const handleAppClick = (id: number, icon: string) => {
    setSelectedApp(id);
    onAppSelect(icon);
  };

  return (
    <div className="dock-container">
      <div className="dock">
        {dockItems.map((item) => (
          <div
            key={item.id}
            className={`dock-item ${selectedApp === item.id ? "selected" : ""}`}
          >
            <div className="icon">
              <img src={item.icon} />
            </div>
            <span className="label"></span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Dock;
