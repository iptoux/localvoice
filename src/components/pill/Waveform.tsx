import { useEffect, useRef } from "react";
import { useAppStore } from "../../stores/app-store";

const BAR_COUNT = 12;
const BAR_WIDTH = 3;
const BAR_GAP = 2;
const MAX_HEIGHT = 20;
const MIN_HEIGHT = 3;

/**
 * Simple audio level waveform visualization.
 *
 * Renders animated bars whose height responds to the current audio RMS level.
 * Each bar has a slight random offset for visual variety. Uses requestAnimationFrame
 * for smooth animation — no external dependencies.
 */
export function Waveform() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const levelRef = useRef(0);

  // Track the latest audio level in a ref for the animation loop.
  const audioLevel = useAppStore((s) => s.audioLevel);
  useEffect(() => {
    levelRef.current = audioLevel;
  }, [audioLevel]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const totalWidth = BAR_COUNT * (BAR_WIDTH + BAR_GAP) - BAR_GAP;
    canvas.width = totalWidth;
    canvas.height = MAX_HEIGHT + 4;

    let animId: number;
    const offsets = Array.from({ length: BAR_COUNT }, () => Math.random() * 0.5 + 0.5);

    const draw = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      const level = Math.min(levelRef.current * 4, 1); // Amplify for visibility

      for (let i = 0; i < BAR_COUNT; i++) {
        const barLevel = level * offsets[i];
        const height = MIN_HEIGHT + barLevel * (MAX_HEIGHT - MIN_HEIGHT);
        const x = i * (BAR_WIDTH + BAR_GAP);
        const y = (canvas.height - height) / 2;

        ctx.fillStyle = `rgba(255, 255, 255, ${0.5 + barLevel * 0.5})`;
        ctx.beginPath();
        ctx.roundRect(x, y, BAR_WIDTH, height, 1.5);
        ctx.fill();
      }

      // Slowly rotate offsets for organic feel.
      for (let i = 0; i < BAR_COUNT; i++) {
        offsets[i] += (Math.random() - 0.5) * 0.15;
        offsets[i] = Math.max(0.3, Math.min(1, offsets[i]));
      }

      animId = requestAnimationFrame(draw);
    };

    draw();
    return () => cancelAnimationFrame(animId);
  }, []);

  return (
    <canvas
      ref={canvasRef}
      data-tauri-drag-region
      className="flex-shrink-0"
      style={{ width: BAR_COUNT * (BAR_WIDTH + BAR_GAP) - BAR_GAP, height: MAX_HEIGHT + 4 }}
    />
  );
}
