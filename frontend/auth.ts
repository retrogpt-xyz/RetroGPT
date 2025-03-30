import { TokenResponse } from "@react-oauth/google";
import axios from "axios";
import { format_api_request_url } from "./request";

interface AuthResponse {
  session_token: string;
  user_id: number;
}

export async function auth(
  tokenResponse: TokenResponse,
): Promise<{ sessionToken: string; userId: number } | null> {
  try {
    const response = await axios.post<AuthResponse>(
      format_api_request_url("v0.0.1/auth"),
      {
        user_access_token: tokenResponse.access_token,
      },
      {
        headers: {
          "Content-Type": "application/json",
        },
      },
    );

    return {
      sessionToken: response.data.session_token,
      userId: response.data.user_id,
    };
  } catch (error) {
    console.error("Authentication error:", error);
    return null;
  }
}
