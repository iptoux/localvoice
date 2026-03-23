import {
  MousePointerClick,
  Keyboard,
  Radio,
  ClipboardCopy,
  BookOpen,
  Scissors,
  Timer,
  HardDrive,
  Cpu,
  History,
  Languages,
  BellRing,
  LogIn,
  LayoutPanelLeft,
  Pilcrow,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";

export interface Tip {
  icon: LucideIcon;
  text: string;
}

export const TIPS: Tip[] = [
  {
    icon: MousePointerClick,
    text: "Right-click the pill to expand quick actions.",
  },
  {
    icon: Keyboard,
    text: "Press your global shortcut from any app to start recording.",
  },
  {
    icon: Radio,
    text: "Enable Push-to-Talk to record only while holding the shortcut.",
  },
  {
    icon: ClipboardCopy,
    text: 'Switch Output mode to "Auto-insert" to type directly into any app.',
  },
  {
    icon: BookOpen,
    text: "Add correction rules in Dictionary to fix recurring transcription errors.",
  },
  {
    icon: Scissors,
    text: 'Enable "Remove filler words" to strip uh, um, äh from output.',
  },
  {
    icon: Timer,
    text: "Set a silence timeout so recording stops automatically when you pause.",
  },
  {
    icon: HardDrive,
    text: 'Enable "Keep audio" to reprocess recordings with a different model later.',
  },
  {
    icon: Cpu,
    text: "Download a smaller Whisper model in Models for faster transcription.",
  },
  {
    icon: History,
    text: "Browse and search all past transcriptions in the History view.",
  },
  {
    icon: Languages,
    text: "Set a fixed language in Settings to skip auto-detection overhead.",
  },
  {
    icon: BellRing,
    text: "Enable success notifications to confirm each completed transcription.",
  },
  {
    icon: LogIn,
    text: 'Enable "Launch at login" so LocalVoice is always ready when you start Windows.',
  },
  {
    icon: LayoutPanelLeft,
    text: 'Set "Start minimized" to keep only the pill visible on launch.',
  },
  {
    icon: Pilcrow,
    text: 'Enable "Auto-punctuation" to get properly punctuated output without effort.',
  },
];
