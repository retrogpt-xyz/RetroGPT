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
  return Effect.gen(function* (_) {
    let encodedName = yield* _(
      Effect.map(encodeUriComponent(name), (s) => s + "="),
    );

    const cookies = document.cookie.split("; ");

    for (const cookie of cookies) {
      // Check if the cookie starts with the encoded name.
      if (cookie.indexOf(encodedName) === 0) {
        return yield* _(
          decodeUriComponent(cookie.substring(encodedName.length)),
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
