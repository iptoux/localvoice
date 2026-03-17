import React from "react";

interface Props {
  children: React.ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    console.error("[ErrorBoundary] Uncaught error:", error, info);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="flex flex-col items-center justify-center h-screen bg-neutral-950 text-white p-8">
          <div className="max-w-md text-center">
            <h1 className="text-xl font-semibold mb-3 text-red-400">
              Something went wrong
            </h1>
            <p className="text-sm text-neutral-400 mb-4">
              An unexpected error occurred. Try reloading the window.
            </p>
            {this.state.error && (
              <pre className="text-xs text-neutral-500 bg-neutral-900 border border-neutral-700 rounded p-3 text-left overflow-auto max-h-40">
                {this.state.error.message}
              </pre>
            )}
            <button
              onClick={() => this.setState({ hasError: false, error: null })}
              className="mt-5 px-4 py-2 text-sm bg-neutral-700 hover:bg-neutral-600 rounded transition-colors"
            >
              Try again
            </button>
          </div>
        </div>
      );
    }
    return this.props.children;
  }
}
