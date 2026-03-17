import { useNavigate } from "react-router-dom";

interface Props {
  onDismiss: () => void;
}

export function Onboarding({ onDismiss }: Props) {
  const navigate = useNavigate();

  const goToModels = () => {
    navigate("/models");
    onDismiss();
  };

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
      <div className="bg-neutral-900 border border-neutral-700 rounded-2xl p-8 w-full max-w-md shadow-2xl">
        <h1 className="text-xl font-semibold text-white mb-2">
          Welcome to LocalVoice
        </h1>
        <p className="text-neutral-400 text-sm mb-6">
          To start dictating, you need to download a transcription model first.
          LocalVoice runs entirely offline — no cloud, no account required.
        </p>

        <div className="space-y-3 mb-6">
          <Step number={1} text="Go to Models and download a model (Tiny or Base recommended for a quick start)" />
          <Step number={2} text="Press the global shortcut to start recording — the pill turns red while listening" />
          <Step number={3} text="Stop speaking; transcription appears in a moment and is copied to your clipboard" />
        </div>

        <div className="flex gap-3">
          <button
            onClick={goToModels}
            className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-medium rounded-lg transition-colors"
          >
            Download a Model
          </button>
          <button
            onClick={onDismiss}
            className="px-4 py-2 text-sm text-neutral-400 hover:text-white transition-colors"
          >
            Skip for now
          </button>
        </div>
      </div>
    </div>
  );
}

function Step({ number, text }: { number: number; text: string }) {
  return (
    <div className="flex items-start gap-3">
      <span className="w-6 h-6 rounded-full bg-blue-600 text-white text-xs font-semibold flex items-center justify-center shrink-0 mt-0.5">
        {number}
      </span>
      <p className="text-sm text-neutral-300">{text}</p>
    </div>
  );
}
