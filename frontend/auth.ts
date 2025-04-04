import { Effect } from "effect";
import * as Api from "./Api";

export async function auth(access_token: string) {
  return Effect.runPromise(
    Api.authApi({
      user_access_token: access_token,
    }),
  );
}
