export function get_api_host(): string {
  const is_dev = import.meta.env.DEV;

  return is_dev ? "localhost:4002" : window.location.host;
}
