import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { PillApp } from "./PillApp";
import { MainApp } from "./MainApp";

async function bootstrap() {
  const label = (await getCurrentWindow()).label;
  const App = label === "pill" ? PillApp : MainApp;

  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}

bootstrap();
