import { Url } from "@effect/platform";
import { Context, Effect, Layer } from "effect";
import * as WindowLocation from "./WindowLocation";

export const getViteBaseUrl = Url.fromString("http://localhost:4002").pipe(
  Effect.mapError((_) => new UrlError()),
);

export const getComposeBaseUrl = Effect.gen(function* () {
  const location = yield* WindowLocation.WindowLocation;
  return yield* Url.fromString(yield* location.origin);
}).pipe(Effect.mapError((_) => new UrlError()));

export class BaseUrlProvider extends Context.Tag("@rgpt/BaseUrlProvider")<
  BaseUrlProvider,
  {
    get: Effect.Effect<URL, UrlError>;
  }
>() {}

export class UrlError {
  readonly _tag = "@rgpt/UrlError";
}

export const isVite = Effect.sync(() => import.meta.env.DEV);

export const layer = Layer.effect(
  BaseUrlProvider,
  Effect.gen(function* () {
    const getter = (yield* isVite)
      ? getViteBaseUrl
      : getComposeBaseUrl.pipe(
          Effect.provideService(
            WindowLocation.WindowLocation,
            yield* WindowLocation.WindowLocation,
          ),
        );

    return { get: getter };
  }),
);
