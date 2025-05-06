// CDPlayer.tsx
import React, {
  useState,
  useRef,
  useEffect,
  MouseEvent as ReactMouseEvent,
  ChangeEvent,
} from 'react'
import './MusicPlayer.css'

type Track = { url: string; name: string }

const playlists = ['Retro Vibes', 'Classic Rock', 'Chill Beats']

const CDPlayer: React.FC = () => {
  // audio / track state
  const [tracks, setTracks] = useState<Track[]>([])
  const [currentTrack, setCurrentTrack] = useState(0)
  const [isPlaying, setIsPlaying] = useState(false)
  const [time, setTime] = useState(0)

  // drag state
  const [position, setPosition] = useState({ x: 100, y: 100 })
  const [dragging, setDragging] = useState(false)
  const dragOffset = useRef({ x: 0, y: 0 })

  // menu state (now includes 'Artist' as a possible key)
  const [openMenu, setOpenMenu] = useState<string | null>(null)
  const playerWindowRef = useRef<HTMLDivElement>(null) // Ref for the whole window

  // playlist state
  const [selectedPlaylist, setSelectedPlaylist] =
    useState(playlists[0])

  const audioRef = useRef<HTMLAudioElement>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)

  // wire up audio events
  useEffect(() => {
    const audio = audioRef.current
    if (!audio) return
    const onTimeUpdate = () => setTime(audio.currentTime)
    const onEnded = () => {
      if (currentTrack < tracks.length - 1) {
        setCurrentTrack(i => i + 1)
        setTime(0)
        if (isPlaying) audio.play()
      } else {
        setIsPlaying(false)
      }
    }
    const onError = (e: ErrorEvent) => {
      console.error('Audio playback error:', e)
      setIsPlaying(false)
      // Optionally show an error message to the user
    }
    audio.addEventListener('timeupdate', onTimeUpdate)
    audio.addEventListener('ended', onEnded)
    audio.addEventListener('error', onError)
    return () => {
      audio.removeEventListener('timeupdate', onTimeUpdate)
      audio.removeEventListener('ended', onEnded)
      audio.removeEventListener('error', onError)
    }
  }, [currentTrack, isPlaying, tracks.length])

  // close menu on outside click (using playerWindowRef)
  useEffect(() => {
    const handleDocClick = (e: MouseEvent) => {
      if (
        playerWindowRef.current &&
        !playerWindowRef.current.contains(e.target as Node)
      ) {
        setOpenMenu(null)
      }
    }
    // Use mousedown to catch clicks before button actions might trigger
    document.addEventListener('mousedown', handleDocClick)
    return () =>
      document.removeEventListener('mousedown', handleDocClick)
  }, [])

  // drag handlers
  useEffect(() => {
    const onMouseMove = (e: MouseEvent) => {
      if (!dragging) return
      setPosition({
        x: e.clientX - dragOffset.current.x,
        y: e.clientY - dragOffset.current.y,
      })
    }
    const onMouseUp = () => setDragging(false)
    document.addEventListener('mousemove', onMouseMove)
    document.addEventListener('mouseup', onMouseUp)
    return () => {
      document.removeEventListener('mousemove', onMouseMove)
      document.removeEventListener('mouseup', onMouseUp)
    }
  }, [dragging])

  // Only allow dragging from the main menubar
  const handleDragMouseDown = (
    e: ReactMouseEvent<HTMLDivElement>
  ) => {
    // Check if the click target is the menubar itself or one of its direct children (spans)
    // This prevents starting a drag when clicking inside an open dropdown
    const target = e.target as HTMLElement
    if (
      target.classList.contains('cdplayer-menubar') ||
      target.parentElement?.classList.contains('cdplayer-menubar')
    ) {
      e.preventDefault()
      dragOffset.current = {
        x: e.clientX - position.x,
        y: e.clientY - position.y,
      }
      setDragging(true)
      setOpenMenu(null) // Close menus when starting drag
    }
  }

  // file loading
  const handleEject = () => fileInputRef.current?.click()
  const handleFiles = (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (!files?.length) return
    const newTracks = Array.from(files).map(f => ({
      url: URL.createObjectURL(f),
      name: f.name,
    }))
    // Clean up previous object URLs before replacing them
    tracks.forEach(track => URL.revokeObjectURL(track.url))
    setTracks(newTracks)
    setCurrentTrack(0)
    setTime(0)
    setIsPlaying(false)
  }

  useEffect(() => {
    // existing code...
    return () => {
      tracks.forEach(track => URL.revokeObjectURL(track.url))
    }
  }, [tracks])

  // transport controls
  const handlePlayPause = () => {
    const audio = audioRef.current
    if (!audio || !tracks.length) return
    if (isPlaying) {
      audio.pause()
      setIsPlaying(false)
    } else {
      audio.play()
      setIsPlaying(true)
    }
  }
  const handleStop = () => {
    const audio = audioRef.current
    if (!audio) return
    audio.pause()
    audio.currentTime = 0
    setIsPlaying(false)
    setTime(0)
  }
  const handlePrev = () => {
    if (currentTrack > 0) {
      setCurrentTrack(i => i - 1)
      setTime(0)
      if (isPlaying) audioRef.current?.play()
    }
  }
  const handleNext = () => {
    if (currentTrack < tracks.length - 1) {
      setCurrentTrack(i => i + 1)
      setTime(0)
      if (isPlaying) audioRef.current?.play()
    }
  }

  // format display
  const formatTime = (s: number) => {
    const m = Math.floor(s / 60)
      .toString()
      .padStart(2, '0')
    const secs = Math.floor(s % 60)
      .toString()
      .padStart(2, '0')
    return `${m}:${secs}`
  }
  const displayTrack = tracks.length
    ? `[${String(currentTrack + 1).padStart(2, '0')}] ${formatTime(
        time
      )}`
    : '[00] 00:00'
  const titleLabel = tracks.length
    ? tracks[currentTrack].name
    : 'Please insert an audio compact disc.'
  const trackLabel = tracks.length
    ? String(currentTrack + 1)
    : ''

  // build menu structures
  const discMenu = [
    { label: 'Eject', action: handleEject },
    {
      label: isPlaying ? 'Pause' : 'Play',
      action: handlePlayPause,
    },
    { label: 'Stop', action: handleStop },
  ]
  const viewMenu = [
    { label: 'Display Playlist', action: () => {} },
    { label: 'Display Info', action: () => {} },
  ]
  const optionsMenu = [
    { label: 'Shuffle', action: () => {} },
    { label: 'Repeat', action: () => {} },
  ]
  const helpMenu = [{ label: 'About', action: () => {} }]

  // Combine top menus
  const topMenuItems: Record<string, typeof discMenu> = {
    Disc: discMenu,
    View: viewMenu,
    Options: optionsMenu,
    Help: helpMenu,
  }
  const topMenus = Object.keys(topMenuItems)

  // Playlist menu items
  const artistMenuItems = playlists.map(name => ({
    label: name,
    action: () => setSelectedPlaylist(name),
  }))

  // Generic menu click handler
  const handleMenuClick = (
    e: ReactMouseEvent,
    menuKey: string
  ) => {
    e.stopPropagation() // Prevent closing immediately
    setOpenMenu(openMenu === menuKey ? null : menuKey)
  }

  // Generic menu item click handler
  const handleMenuItemClick = (
    e: ReactMouseEvent,
    action: () => void
  ) => {
    e.stopPropagation()
    action()
    setOpenMenu(null) // Close menu after action
  }

  return (
    <div
      className="cdplayer-window"
      style={{ top: position.y, left: position.x }}
      ref={playerWindowRef} // Add ref here
    >
      {/* Draggable Menubar */}
      <div
        className="cdplayer-menubar"
        onMouseDown={handleDragMouseDown} // Use specific drag handler
      >
        {topMenus.map(menu => (
          <div key={menu} className="menubar-menu">
            <span onClick={e => handleMenuClick(e, menu)}>
              {menu}
            </span>
            {openMenu === menu && (
              <ul className="dropdown">
                {topMenuItems[menu].map(item => (
                  <li
                    key={item.label}
                    onClick={e =>
                      handleMenuItemClick(e, item.action)
                    }
                  >
                    {item.label}
                  </li>
                ))}
              </ul>
            )}
          </div>
        ))}
      </div>

      {/* Display */}
      <div className="cdplayer-display">{displayTrack}</div>

      {/* Controls */}
      <div className="cdplayer-controls">
        <button
          onClick={handlePrev}
          disabled={!tracks.length}
          className="button"
        >
          &#9198;
        </button>
        <button
          onClick={handleStop}
          disabled={!tracks.length}
          className="button"
        >
          &#9632;
        </button>
        <button
          onClick={handlePlayPause}
          disabled={!tracks.length}
          className="button"
        >
          {isPlaying ? '❚❚' : '▶'}
        </button>
        <button
          onClick={handleNext}
          disabled={!tracks.length}
          className="button"
        >
          &#9197;
        </button>
        <button onClick={handleEject} className="button">
          ⏏
        </button>
      </div>

      {/* Artist/Playlist Row - New Style */}
      <div className="cdplayer-info-row">
        <label>Artist:</label>
        <div className="playlist-menu-container">
          {' '}
          {/* Container for positioning */}
          <div className="playlist-menu-trigger">
            <span onClick={e => handleMenuClick(e, 'Artist')}>
              {selectedPlaylist}
            </span>
            {/* Arrow indicator */}
            <span
              className="dropdown-arrow"
              onClick={e => handleMenuClick(e, 'Artist')}
            >
              ▼
            </span>
          </div>
          {openMenu === 'Artist' && (
            <ul className="dropdown playlist-dropdown">
              {' '}
              {/* Specific class if needed */}
              {artistMenuItems.map(item => (
                <li
                  key={item.label}
                  onClick={e => handleMenuItemClick(e, item.action)}
                >
                  {item.label}
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>

      {/* Title Row */}
      <div className="cdplayer-info-row">
        <label>Title:</label>
        <input type="text" value={titleLabel} disabled />
      </div>

      {/* Track Row */}
      <div className="cdplayer-info-row">
        <label>Track:</label>
        <input type="text" value={trackLabel} disabled />
      </div>

      {/* Hidden file input */}
      <input
        type="file"
        accept=".wav"
        multiple
        ref={fileInputRef}
        onChange={handleFiles}
        style={{ display: 'none' }}
      />

      {/* Audio element */}
      <audio
        ref={audioRef}
        src={tracks[currentTrack]?.url}
        preload="metadata"
      />
    </div>
  )
}

export default CDPlayer
