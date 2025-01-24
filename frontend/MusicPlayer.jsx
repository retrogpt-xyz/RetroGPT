import React from "react";
import "./MusicPlayer.css";

const SoundCloudPlayer = () => {
  return (
    <div className="soundcloud-container">
      {/* SoundCloud iframe */}
      <iframe
        className="soundcloud-player"
        title="SoundCloud Player"
        width="100%"
        height="300"
        scrolling="no"
        frameBorder="no"
        allow="autoplay"
        src="https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/playlists/68014678&color=%23666e7c&auto_play=true&hide_related=false&hide_comments=false&hide_user=false&hide_reposts=false&hide_teaser=false&visual=false"
      ></iframe>

      {/* Description and links */}
      <div className="soundcloud-info">
        <a
          //href="https://soundcloud.com/windows-98-1"
          title="Windows 98の"
          target="_blank"
          rel="noopener noreferrer"
          className="soundcloud-link"
        >
        </a>{" "}
        ·{" "}
        <a
          //href="https://soundcloud.com/windows-98-1/sets/our27bsakoe2"
          title="ブルースクリーン"
          target="_blank"
          rel="noopener noreferrer"
          className="soundcloud-link"
        >
        </a>
      </div>
    </div>
  );
};

export default SoundCloudPlayer;