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
    const encodedName = yield* _(encodeUriComponent(name));
    const encodedValue = yield* _(encodeUriComponent(value));

    document.cookie = `${encodedName}=${encodedValue}; path=/;`;
  });
}

export function getCookie(name: string) {
export function getCookie(name: string) {
  return Effect.gen(function* (_) {
    const encodedName = yield* _(
      Effect.map(encodeUriComponent(name), (s) => s + "="),
    );

    const cookies = document.cookie.split("; ");

    for (const cookie of cookies) {
      // Split cookie into name and value to avoid partial name matches
      const [cookieName, cookieValue] = cookie.split('=', 2);
      const decodedName = yield* _(decodeUriComponent(cookieName));
      if (decodedName === name) {
        return yield* _(
          decodeUriComponent(cookieValue),
        );
      }
    }

    return yield* _(Effect.fail(new CookieNotFoundError(name)));
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
