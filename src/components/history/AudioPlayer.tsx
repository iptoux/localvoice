import { useEffect, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { Play, Pause, Volume2 } from "lucide-react";

interface AudioPlayerProps {
  audioPath: string;
}

export function AudioPlayer({ audioPath }: AudioPlayerProps) {
  const audioRef = useRef<HTMLAudioElement>(null);
  const [playing, setPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [loadError, setLoadError] = useState<string | null>(null);
  const src = convertFileSrc(audioPath);

  useEffect(() => {
    setPlaying(false);
    setCurrentTime(0);
    setDuration(0);
    setLoadError(null);
  }, [audioPath]);

  function togglePlay() {
    const el = audioRef.current;
    if (!el) return;
    if (playing) {
      el.pause();
    } else {
      el.play().catch((e) => setLoadError(String(e)));
    }
  }

  function handleSeek(e: React.ChangeEvent<HTMLInputElement>) {
    const el = audioRef.current;
    if (!el) return;
    el.currentTime = Number(e.target.value);
  }

  function fmt(s: number) {
    if (!isFinite(s)) return "00:00";
    const m = Math.floor(s / 60);
    return `${String(m).padStart(2, "0")}:${String(Math.floor(s % 60)).padStart(2, "0")}`;
  }

  return (
    <div className="flex flex-col gap-1 bg-muted rounded-md px-3 py-2 mt-2">
      <div className="flex items-center gap-2">
        <Volume2 size={12} className="text-muted-foreground shrink-0" />
        <audio
          ref={audioRef}
          src={src}
          onPlay={() => setPlaying(true)}
          onPause={() => setPlaying(false)}
          onEnded={() => setPlaying(false)}
          onTimeUpdate={() => setCurrentTime(audioRef.current?.currentTime ?? 0)}
          onLoadedMetadata={() => setDuration(audioRef.current?.duration ?? 0)}
          onError={(e) => setLoadError(`Cannot load audio (${(e.target as HTMLAudioElement).error?.message ?? "unknown"})`)}
          preload="metadata"
        />
        <button
          onClick={togglePlay}
          className="text-foreground/80 hover:text-foreground transition-colors shrink-0"
          aria-label={playing ? "Pause" : "Play"}
        >
          {playing ? <Pause size={14} /> : <Play size={14} />}
        </button>
        <input
          type="range"
          min={0}
          max={duration || 1}
          step={0.1}
          value={currentTime}
          onChange={handleSeek}
          className="flex-1 h-1 accent-foreground cursor-pointer"
          aria-label="Seek"
        />
        <span className="text-xs text-muted-foreground tabular-nums shrink-0">
          {fmt(currentTime)} / {fmt(duration)}
        </span>
      </div>
      {loadError && (
        <p className="text-xs text-rose-400">{loadError}</p>
      )}
    </div>
  );
}
