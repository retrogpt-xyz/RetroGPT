import { Effect, Schema } from "effect";
import { formatApiUrl } from "./ApiUrlFormat";
import { HttpBody, HttpClient } from "@effect/platform";
import { BrowserHttpClient } from "@effect/platform-browser";
import * as BaseUrl from "./BaseUrl";
import * as WindowLocation from "./WindowLocation";

const makePostEndpoint =
  <DI, EI, RI, DO, EO, RO>(
    inputSchema: Schema.Schema<DI, EI, RI>,
    outputSchema: Schema.Schema<DO, EO, RO>,
    slug: string,
  ) =>
  (bodyDecoded: DI, sessionToken?: string) =>
    Effect.gen(function* () {
      const url = yield* formatApiUrl(slug);
      const client = yield* HttpClient.HttpClient;
      const body = yield* HttpBody.jsonSchema(inputSchema)(bodyDecoded);
      const response = yield* client.post(url, {
        body: body,
        headers: sessionToken ? { "X-Session-Token": sessionToken } : undefined,
      });
      return yield* Schema.decodeUnknown(outputSchema)(yield* response.json);
    }).pipe(
      Effect.provide(BrowserHttpClient.layerXMLHttpRequest),
      Effect.provide(BaseUrl.layer),
      Effect.provide(WindowLocation.layer),
    );

export const authApi = makePostEndpoint(
  Schema.Struct({ user_access_token: Schema.String }),
  Schema.Struct({ session_token: Schema.String, user_id: Schema.Number }),
  "/api/v0.0.1/auth",
);

export const userChatsApi = makePostEndpoint(
  Schema.Struct({ user_id: Schema.Union(Schema.Number, Schema.Null) }),
  Schema.Struct({
    chats: Schema.Array(
      Schema.Struct({
        id: Schema.Number,
        name: Schema.String,
      }),
    ),
    user_id: Schema.Number,
  }),
  "/api/v0.0.1/user_chats",
);

export const chatMsgsApi = makePostEndpoint(
  Schema.Struct({ chat_id: Schema.Number }),
  Schema.Array(
    Schema.Struct({
      text: Schema.String,
      sender: Schema.Union(Schema.Literal("ai"), Schema.Literal("user")),
    }),
  ),
  "/api/v0.0.1/chat_msgs",
);

export const promptApi = makePostEndpoint(
  Schema.Struct({
    text: Schema.String,
    chat_id: Schema.Union(Schema.Number, Schema.Null),
  }),
  Schema.Struct({
    chat_id: Schema.Number,
    attach_token: Schema.String,
  }),
  "/api/v0.0.1/prompt",
);
