import { Download, RefreshCw, X } from "lucide-react";
import { useUpdaterStore } from "../stores/updater-store";
import { Button } from "@/components/ui/button";

function formatProgress(percent?: number | null) {
  return typeof percent === "number" ? `${percent}%` : "Downloading";
}

export function UpdateBanner() {
  const { status, dismissedVersion, loading, install, dismiss } = useUpdaterStore();
  const info = status.available;

  if (!info || dismissedVersion === info.version) {
    return null;
  }

  const installing = status.phase === "installing";
  const downloading = status.phase === "downloading";
  const busy = loading || downloading || installing;
  const progress = status.progress?.percent;

  return (
    <div className="border-b border-border bg-card text-card-foreground">
      <div className="flex min-h-14 items-center justify-between gap-4 px-5 py-3">
        <div className="flex min-w-0 items-center gap-3">
          <div className="flex size-8 shrink-0 items-center justify-center rounded-md bg-secondary/15 text-secondary">
            {busy ? (
              <RefreshCw className="size-4 animate-spin" />
            ) : (
              <Download className="size-4" />
            )}
          </div>
          <div className="min-w-0">
            <p className="text-sm font-medium text-foreground">
              LocalVoice {info.version} is available
            </p>
            <p className="text-xs text-muted-foreground">
              Current version: {info.currentVersion}
              {downloading && ` · ${formatProgress(progress)}`}
              {installing && " · Restarting to install"}
            </p>
            {status.lastError && (
              <p className="mt-1 text-xs text-destructive">{status.lastError}</p>
            )}
          </div>
        </div>

        <div className="flex shrink-0 items-center gap-2">
          <Button
            size="sm"
            onClick={() => install().catch(() => {})}
            disabled={busy}
            className="h-8 gap-2"
          >
            <Download className="size-3.5" />
            {busy ? "Updating" : "Update Now"}
          </Button>
          <Button
            type="button"
            variant="ghost"
            size="icon"
            aria-label="Dismiss update"
            onClick={dismiss}
            disabled={busy}
            className="size-8"
          >
            <X className="size-4" />
          </Button>
        </div>
      </div>
    </div>
  );
}
