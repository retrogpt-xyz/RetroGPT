import { Effect } from "effect";
import { TokenResponse } from "./Api";
import * as Api from "./Api";

export async function auth(tokenResponse: TokenResponse) {
  return Effect.runPromise(Api.auth(tokenResponse));
}
