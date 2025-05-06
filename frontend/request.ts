import { Effect } from "effect";
import { deleteChatApi } from "./Api";

export function get_api_host(): string {
  const is_dev = import.meta.env.DEV;

  return is_dev ? "localhost:4002" : window.location.host;
}

function get_api_base_url(): string {
  const host = get_api_host();
  const protocol = window.location.protocol;

  return `${protocol}//${host}`;
}

export function format_api_request_url(slug: string) {
  const base_url = get_api_base_url();
  const normalizedSlug = slug.startsWith("/") ? slug.substring(1) : slug;

  return `${base_url}/api/${normalizedSlug}`;
}

export async function delete_chat(chat_id: number, sessionToken: string) {
  await Effect.runPromise(deleteChatApi({ chat_id: chat_id }, sessionToken));
}
