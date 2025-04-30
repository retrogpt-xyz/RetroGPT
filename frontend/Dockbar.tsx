import { useEffect, useState } from "react";
import "./Dockbar.css";

interface DockItem {
  id: number;
  icon: string;
  label: string;
}

const Dock = () => {
  const [selectedApp] = useState<number | null>(null);
  const [currentTime, setCurrentTime] = useState(new Date());

  // Update time every minute
  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date());
    }, 60000);

    return () => clearInterval(timer);
  }, []);

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString("en-US", {
      hour: "numeric",
      minute: "2-digit",
      hour12: true,
    });
  };

  const formatDate = (date: Date) => {
    return date.toLocaleDateString("en-US", {
      month: "numeric",
      day: "numeric",
      year: "2-digit",
    });
  };


  const dockItems: DockItem[] = [
    {
      id: 1,
      icon: "https://64.media.tumblr.com/3ea96a37f9c508e9c7ca7f95c2d9e5c6/32f4c776e65ab1bc-a7/s540x810/7e9ac2c7bcb1c31e20ca09649e7d96fb09982fd8.png",
      label: "Get Started",
    },
    {
      id: 2,
      icon: "https://64.media.tumblr.com/0d181187c50fedc1c60d1a6c3dd2165d/ec299322d93fd773-53/s540x810/afd900c44adfac375f08a490df747be6384c17d6.png",
      label: "About Us",
    },
    {
      id: 3,
      icon: "https://64.media.tumblr.com/42e2b6779cbb09f0bf4ec645560be93f/9d46196f98fe3bc0-93/s540x810/6c3f4bf1a3069443c09f0751cb7375e5ebde98a2.png",
      label: "Files",
    },
  ];

  return (
    <div className="dock-container">
      <div className="dock">
        {dockItems.map((item) => (
          <div
            key={item.id}
            className={`dock-item ${selectedApp === item.id ? "selected" : ""}`}
          >
            <div className="icon">
              <img src={item.icon} alt={`App ${item.id}`} />
            </div>
            <span className="dock-label">{item.label}</span>
          </div>
        ))}
        <div className="dock-clock">
          <span>{formatTime(currentTime)}</span>
          <span>{formatDate(currentTime)}</span>
        </div>
      </div>
    </div>
  );
};

export default Dock;
