import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import MainScreen from "./App.tsx";
import "./main.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <MainScreen />
  </StrictMode>,
);
