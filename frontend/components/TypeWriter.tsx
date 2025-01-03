import { useState, useEffect } from "react";

type TypewriterProps = {
  text: string;
  speed?: number;
};

export const Typewriter = ({ text, speed = 5 }: TypewriterProps) => {
  const [displayedText, setDisplayedText] = useState("");

  useEffect(() => {
    let index = 0;
    const intervalId = setInterval(() => {
      setDisplayedText((_) => text.substring(0, index));
      index++;
      if (index >= text.length) {
        clearInterval(intervalId);
      }
    }, speed);
    return () => clearInterval(intervalId);
  }, [text, speed]);

  return (
    <span style={{ fontFamily: '"Static JetBrains Mono", monospace' }}>
      {displayedText}
    </span>
  );
};
