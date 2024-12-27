import { useState } from 'react';
import openai_anim from './assets/openai_anim.mp4'
import secondclip from './assets/secondclip.mp4'
import logo from './assets/chatlogo.png'

function TextBox() {
  const [text, setText] = useState('');

  const handleSubmit = () => {
    if (text.trim()) {
      alert(text);
      setText('');
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter') {
      if (!e.shiftKey) {
        e.preventDefault();
        handleSubmit();
      }
    }
  };

  return (
    <textarea 
      value={text}
      onChange={(e) => setText(e.target.value)}
      style={{
        width: '100%',
        minHeight: '40px',
        maxHeight: '100%',
        backgroundColor: '#000000',
        color: '#ffffff',
        borderRadius: '4px',
        padding: '10px',
        border: '2px solid #00ff00',
        outline: 'none',
        fontFamily: 'JetBrains Mono, monospace',
        fontSize: '16px',
        resize: 'none',
        overflow: 'auto'
      }}
      placeholder="Type your message..."
      rows={1}
      onKeyDown={handleKeyDown}
      onInput={(e) => {
        const textarea = e.target as HTMLTextAreaElement;
        textarea.style.height = 'auto';
        textarea.style.height = `${Math.min(textarea.scrollHeight, textarea.parentElement?.clientHeight || Infinity)}px`;
      }}
    />
  );
}

function MainScreen() {
  return (
    <div style={{ 
      width: '100%',
      height: '100%',
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
    }}>
      <div style={{
        width: '80%',
        height: '80%',
        border: '2px solid green',
        borderRadius: '8px',
        display: 'flex',
        flexDirection: 'column',
        padding: '20px',
        background: 'linear-gradient(to bottom, #000000, #001700)',
        gap: '20px'
      }}>
        <div style={{
          flex: '8',
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center'
        }}>
          <img 
            src={logo} 
            alt="Chat Logo"
            style={{
              maxWidth: '100%',
              maxHeight: '100%',
              objectFit: 'contain'
            }}
          />
        </div>
        <div style={{
          flex: '2',
          width: '100%',
          display: 'flex',
          alignItems: 'flex-end'
        }}>
          <TextBox />
        </div>
      </div>
    </div>
  );
}

function App() {
  const [componentIdx, setComponentIdx] = useState(0);

  const nextComponent = () => {
    setComponentIdx((current) => (current + 1));
  };

  return (
    <div style={{
      width: '100vw',
      height: '100vh',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      overflow: 'hidden'
    }}>
      {componentIdx === 0 && <VideoPlayer src={openai_anim} onEnded={nextComponent} />}
      {componentIdx === 1 && <VideoPlayer src={secondclip} onEnded={nextComponent} />}
      {componentIdx === 2 && <MainScreen />}
    </div>
  );
}

interface VideoPlayerProps {
  src: string;
  onEnded: () => void;
}

function VideoPlayer({ src, onEnded }: VideoPlayerProps) {
  return (
    <video 
      src={src} 
      autoPlay 
      muted 
      onEnded={onEnded} 
      style={{
        maxWidth: '95%',
        maxHeight: '95%',
        objectFit: 'contain'
      }}
    ></video>
  );
}

export default App;
