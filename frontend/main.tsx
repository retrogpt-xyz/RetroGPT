import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import "./App.css";
import App from "./App";
import { GoogleOAuthProvider } from "@react-oauth/google";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
  <GoogleOAuthProvider clientId={"262296442193-ctnfmo1iedrcj8q1luvn2724r7q8gkel.apps.googleusercontent.com"}>
    <StrictMode>
      <App />
    </StrictMode>
  </GoogleOAuthProvider>,
);
