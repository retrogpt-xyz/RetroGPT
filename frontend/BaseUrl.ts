import { Url } from "@effect/platform";
import { Context, Effect, Layer } from "effect";
import * as WindowLocation from "./WindowLocation";
import { IllegalArgumentException } from "effect/Cause";

export const getViteBaseUrl = Url.fromString("http://localhost:4002");
export const getComposeBaseUrl = Effect.gen(function* () {
  const location = yield* WindowLocation.WindowLocation;
  return yield* Url.fromString(yield* location.origin);
});

export class BaseUrlProvider extends Context.Tag("@rgpt/BaseUrlProvider")<
  BaseUrlProvider,
  {
    get: Effect.Effect<URL, IllegalArgumentException>;
  }
>() {}

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
