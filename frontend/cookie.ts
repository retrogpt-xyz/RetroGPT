import { Effect } from "effect";
import { encodeUriComponent, decodeUriComponent } from "effect/Encoding";

class CookieNotFoundError {
  readonly _tag = "CookieNotFoundError";
  readonly name: string;

  constructor(name: string) {
    this.name = name;
  }
}

export function setCookie(name: string, value: string) {
  return Effect.gen(function* (_) {
    const encodedName = yield* encodeUriComponent(name);
    const encodedValue = yield* encodeUriComponent(value);

    document.cookie = `${encodedName}=${encodedValue}; path=/;`;
  });
}

export function getCookie(name: string) {
  return Effect.gen(function* (_) {
    const cookies = document.cookie.split("; ");

    for (const cookie of cookies) {
      const [cookieName, cookieValue] = cookie.split("=", 2);
      const decodedName = yield* decodeUriComponent(cookieName);
      if (decodedName === name) {
        return yield* decodeUriComponent(cookieValue);
      }
    }

    return yield* Effect.fail(new CookieNotFoundError(name));
  });
}

export function getSessionTokenCookie() {
  return getCookie("retrogptSessionToken").pipe(
    Effect.catchTag("CookieNotFoundError", (_) =>
      Effect.succeed("__default__"),
    ),
  );
}

export function setSessionTokenCookie(sessionToken: string) {
  return setCookie("retrogptSessionToken", sessionToken);
}

export function getSessionTokenCookieWrapper() {
  return Effect.runSync(getSessionTokenCookie());
}

export function setSessionTokenCookieWrapper(sessionToken: string) {
  return Effect.runSync(setSessionTokenCookie(sessionToken));
}
