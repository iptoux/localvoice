import type { Meta, StoryObj } from "@storybook/react";
import { useEffect, useRef } from "react";
import { Waveform } from "../components/pill/Waveform";

const meta: Meta<typeof Waveform> = {
  title: "Waveform",
  component: Waveform,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
    backgrounds: {
      default: "dark",
    },
  },
};

export default meta;
type Story = StoryObj<typeof Waveform>;

export const Default: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({ audioLevel: 0.5 });
      return <Story />;
    },
  ],
};

export const HighLevel: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({ audioLevel: 0.9 });
      return <Story />;
    },
  ],
};

export const LowLevel: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({ audioLevel: 0.2 });
      return <Story />;
    },
  ],
};

export const Silent: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({ audioLevel: 0 });
      return <Story />;
    },
  ],
};

export const DynamicLevel: Story = {
  parameters: {
    layout: "centered",
  },
  render: () => {
    const { useAppStore } = require("../stores/app-store");
    const { mockInvoke } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;
    const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);
    const phaseRef = useRef(0);

    useEffect(() => {
      timerRef.current = setInterval(() => {
        phaseRef.current += 0.1;
        const level = (Math.sin(phaseRef.current) + 1) / 2;
        useAppStore.setState({ audioLevel: level });
      }, 100);

      return () => {
        if (timerRef.current) clearInterval(timerRef.current);
      };
    }, []);

    return (
      <div className="space-y-4">
        <h3 className="text-sm text-zinc-400">Dynamic Audio Level</h3>
        <Waveform />
      </div>
    );
  },
};

export const AllLevels: Story = {
  parameters: {
    layout: "fullscreen",
  },
  render: () => {
    const { useAppStore } = require("../stores/app-store");
    const { mockInvoke } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;

    const levels = [0, 0.2, 0.4, 0.6, 0.8, 1.0];

    return (
      <div className="p-8 space-y-8 bg-zinc-950 min-h-screen">
        <h2 className="text-xl font-semibold text-white">Waveform Levels</h2>
        <div className="grid grid-cols-2 md:grid-cols-3 gap-8">
          {levels.map((level) => {
            useAppStore.setState({ audioLevel: level });
            return (
              <div key={level} className="space-y-2">
                <h3 className="text-sm text-zinc-400">{Math.round(level * 100)}%</h3>
                <div className="bg-zinc-900 p-4 rounded-xl">
                  <Waveform />
                </div>
              </div>
            );
          })}
        </div>
      </div>
    );
  },
};
