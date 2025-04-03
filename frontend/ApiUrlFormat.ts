import { Effect } from "effect";
import * as BaseUrl from "./BaseUrl";
import { Url } from "@effect/platform";

export const formatApiUrl = (slug: string) =>
  BaseUrl.BaseUrlProvider.pipe(
    Effect.andThen((urlProvider) => urlProvider.get),
    Effect.andThen((url) => Url.fromString(slug, url)),
    Effect.catchTag("IllegalArgumentException", (_) =>
      Effect.fail(new BaseUrl.InvalidUrlError()),
    ),
  );
