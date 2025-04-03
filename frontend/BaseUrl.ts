import { Url } from "@effect/platform";
import { Context, Effect, Layer } from "effect";
import * as WindowLocation from "./WindowLocation";
import { IllegalArgumentException } from "effect/Cause";

const viteBaseUrl = Url.fromString("http://localhost:4002");
const composeBaseUrl = Effect.gen(function* () {
  const location = yield* WindowLocation.WindowLocation;
  return yield* Url.fromString(yield* location.origin);
}).pipe(Effect.provide(WindowLocation.layer));

export class BaseUrlProvider extends Context.Tag("tag")<
  BaseUrlProvider,
  {
    get: Effect.Effect<URL, IllegalArgumentException>;
  }
>() {}

const providerBuilder = (
  getter: Effect.Effect<URL, IllegalArgumentException>,
) => {
  Layer.succeed(
    BaseUrlProvider,
    BaseUrlProvider.of({
      get: getter,
    }),
  );
};

export const layers = {
  viteBaseLayer: providerBuilder(viteBaseUrl),
  composeBaseLayer: providerBuilder(composeBaseUrl),
};
