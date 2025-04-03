import * as OAuth from "@react-oauth/google";
import { Effect, Schema } from "effect";
import { formatApiUrl } from "./ApiUrlFormat";
import { HttpBody, HttpClient } from "@effect/platform";
import { BrowserHttpClient } from "@effect/platform-browser";
import * as BaseUrl from "./BaseUrl";
import * as WindowLocation from "./WindowLocation";

export const AuthSchema = Schema.Struct({
  session_token: Schema.String,
  user_id: Schema.Number,
});

export type TokenResponse = Omit<
  OAuth.TokenResponse,
  "error" | "error_description" | "error_uri"
>;

export const auth = (tokenResponse: TokenResponse) =>
  Effect.gen(function* () {
    const url = yield* formatApiUrl("/api/v0.0.1/auth");
    const client = yield* HttpClient.HttpClient;
    const response = yield* client.post(url, {
      body: yield* HttpBody.json({
        user_access_token: tokenResponse.access_token,
      }),
    });
    const json = yield* response.json;
    const parsed = yield* Schema.decodeUnknown(AuthSchema)(json);
    return parsed;
  }).pipe(
    Effect.provide(BrowserHttpClient.layerXMLHttpRequest),
    Effect.provide(BaseUrl.layer),
    Effect.provide(WindowLocation.layer),
  );
