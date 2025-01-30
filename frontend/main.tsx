import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import "./App.css";
import App from "./App";
import { GoogleOAuthProvider } from "@react-oauth/google";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
  <GoogleOAuthProvider clientId={"..."}>
    <StrictMode>
      <App />
    </StrictMode>
  </GoogleOAuthProvider>,
);
