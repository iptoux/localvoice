import type { Meta, StoryObj } from "@storybook/react";
import { Pill } from "../components/pill/Pill";
import type { RecordingState } from "../types";
import { mockTranscription } from "../mocks/tauri";

const meta: Meta<typeof Pill> = {
  title: "Pill",
  component: Pill,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
    backgrounds: {
      default: "dark",
    },
  },
};

export default meta;
type Story = StoryObj<typeof Pill>;

export const Idle: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "idle",
        lastTranscription: null,
        lastOutputResult: null,
        recordingError: null,
        isPillExpanded: false,
      });
      return <Story />;
    },
  ],
};

export const IdleWithLastTranscription: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "idle",
        lastTranscription: mockTranscription,
        lastOutputResult: { mode: "clipboard", success: true },
        recordingError: null,
        isPillExpanded: false,
      });
      return <Story />;
    },
  ],
};

export const Listening: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "listening",
        lastTranscription: null,
        lastOutputResult: null,
        recordingError: null,
        isPillExpanded: false,
        audioLevel: 0.7,
      });
      return <Story />;
    },
  ],
};

export const Processing: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "processing",
        lastTranscription: null,
        lastOutputResult: null,
        recordingError: null,
        isPillExpanded: false,
      });
      return <Story />;
    },
  ],
};

export const Success: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "success",
        lastTranscription: mockTranscription,
        lastOutputResult: { mode: "clipboard", success: true },
        recordingError: null,
        isPillExpanded: false,
      });
      return <Story />;
    },
  ],
};

export const SuccessInsert: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "success",
        lastTranscription: mockTranscription,
        lastOutputResult: { mode: "insert", success: true },
        recordingError: null,
        isPillExpanded: false,
      });
      return <Story />;
    },
  ],
};

export const Error: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "error",
        lastTranscription: null,
        lastOutputResult: null,
        recordingError: "Microphone not available",
        isPillExpanded: false,
      });
      return <Story />;
    },
  ],
};

export const ErrorOutputFailed: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "error",
        lastTranscription: mockTranscription,
        lastOutputResult: { mode: "clipboard", success: false, error: "Clipboard access denied" },
        recordingError: "Output failed",
        isPillExpanded: false,
      });
      return <Story />;
    },
  ],
};

export const ExpandedIdle: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { useSettingsStore } = require("../stores/settings-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "idle",
        lastTranscription: mockTranscription,
        lastOutputResult: { mode: "clipboard", success: true },
        recordingError: null,
        isPillExpanded: true,
      });
      useSettingsStore.setState({
        settings: { "transcription.default_language": "auto" },
      });
      return <Story />;
    },
  ],
};

export const ExpandedListening: Story = {
  decorators: [
    (Story) => {
      const { useAppStore } = require("../stores/app-store");
      const { useSettingsStore } = require("../stores/settings-store");
      const { mockInvoke } = require("../mocks/tauri");
      require("@tauri-apps/api/core").invoke = mockInvoke;
      useAppStore.setState({
        recordingState: "listening",
        lastTranscription: null,
        lastOutputResult: null,
        recordingError: null,
        isPillExpanded: true,
        audioLevel: 0.5,
      });
      useSettingsStore.setState({
        settings: { "transcription.default_language": "de" },
      });
      return <Story />;
    },
  ],
};

export const AllStates: Story = {
  parameters: {
    layout: "fullscreen",
  },
  render: () => {
    const { useAppStore } = require("../stores/app-store");
    const { mockInvoke } = require("../mocks/tauri");
    require("@tauri-apps/api/core").invoke = mockInvoke;

    const states: RecordingState[] = ["idle", "listening", "processing", "success", "error"];

    return (
      <div className="p-8 space-y-8 bg-zinc-950 min-h-screen">
        <h2 className="text-xl font-semibold text-white">Pill States</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {states.map((state) => {
            useAppStore.setState({
              recordingState: state,
              lastTranscription: state === "success" ? mockTranscription : null,
              lastOutputResult: state === "success" ? { mode: "clipboard", success: true } : null,
              recordingError: state === "error" ? "Microphone unavailable" : null,
              isPillExpanded: false,
              audioLevel: state === "listening" ? 0.6 : 0,
            });

            return (
              <div key={state} className="space-y-2">
                <h3 className="text-sm text-zinc-400 uppercase">{state}</h3>
                <div className="w-80 h-16 rounded-2xl overflow-hidden shadow-2xl">
                  <Pill />
                </div>
              </div>
            );
          })}
        </div>
      </div>
    );
  },
};
