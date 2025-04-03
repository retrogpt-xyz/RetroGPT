import { Context, Effect, Layer } from "effect";

export class WindowLocation extends Context.Tag("@rgpt/WindowLocation")<
  WindowLocation,
  {
    readonly ancestorOrigins: Effect.Effect<DOMStringList>;
    readonly hash: Effect.Effect<string>;
    readonly host: Effect.Effect<string>;
    readonly hostname: Effect.Effect<string>;
    readonly href: Effect.Effect<string>;
    toString(): Effect.Effect<string>;
    readonly origin: Effect.Effect<string>;
    readonly pathname: Effect.Effect<string>;
    readonly port: Effect.Effect<string>;
    readonly protocol: Effect.Effect<string>;
    readonly search: Effect.Effect<string>;
    assign(url: string | URL): Effect.Effect<void>;
    reload(): Effect.Effect<void>;
    replace(url: string | URL): Effect.Effect<void>;
  }
>() {}

export const layer = Layer.succeed(
  WindowLocation,
  WindowLocation.of({
    ancestorOrigins: Effect.sync(() => window.location.ancestorOrigins),
    hash: Effect.sync(() => window.location.hash),
    host: Effect.sync(() => window.location.host),
    hostname: Effect.sync(() => window.location.hostname),
    href: Effect.sync(() => window.location.href),
    toString: () => Effect.sync(() => window.location.toString()),
    origin: Effect.sync(() => window.location.origin),
    pathname: Effect.sync(() => window.location.pathname),
    port: Effect.sync(() => window.location.port),
    protocol: Effect.sync(() => window.location.protocol),
    search: Effect.sync(() => window.location.search),
    assign: (url: string | URL) =>
      Effect.sync(() => window.location.assign(url)),
    reload: () => Effect.sync(() => window.location.reload()),
    replace: (url: string | URL) =>
      Effect.sync(() => window.location.replace(url)),
  }),
);
