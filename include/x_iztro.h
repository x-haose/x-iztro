/*
 * x_iztro C FFI header
 *
 * Purple Star Astrology (紫微斗数) library.
 *
 * All returned strings are heap-allocated JSON. The caller MUST free them
 * with iztro_free_string(). Functions never return NULL; on error they
 * return a JSON string of the form
 *   {"error": "message", "code": "invalid_date"}
 * where "code" is one of:
 *   invalid_date       - date string malformed, non-existent or out of range
 *   invalid_time_index - time index outside 0-12
 *   invalid_argument   - any other argument or config value rejected
 *   internal           - defect inside the library (never caused by input)
 * All input is validated up front (date format/existence, Gregorian years
 * 1583-9999, time index 0-12); invalid input yields an error JSON, never a crash.
 *
 * Result JSON carries two layers per field: translated display text
 * (name, brightness, ...) in the requested language, and
 * language-independent identifiers (key, nameKey, mutagenKey, ... using
 * iztro i18n keys such as "ziweiMaj"/"soulPalace"/"sihuaLu") for
 * programmatic matching that works in every output language.
 */

#ifndef X_IZTRO_H
#define X_IZTRO_H

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Generate an astrolabe from a solar (Gregorian) date.
 *
 * Parameters:
 *   solar_date  - Date string, e.g. "2000-8-16"
 *   time_index  - Time index (0-12)
 *   gender      - "male" or "female"
 *   fix_leap    - Whether to fix leap month
 *   language    - "zh-CN", "zh-TW", "en-US", "ja-JP", "ko-KR", or "vi-VN"
 *   config_json - NULL/empty for defaults, or a partial-config JSON such as
 *                 {"algorithm":"zhongzhou","yearDivide":"exact"}
 *
 * Returns: JSON string (must be freed with iztro_free_string)
 */
char* iztro_by_solar(
    const char* solar_date,
    uint8_t time_index,
    const char* gender,
    bool fix_leap,
    const char* language,
    const char* config_json
);

/*
 * Generate an astrolabe from a lunar (Chinese calendar) date.
 *
 * Parameters:
 *   lunar_date     - Lunar date string, e.g. "2000-7-16"
 *   time_index     - Time index (0-12)
 *   gender         - "male" or "female"
 *   is_leap_month  - Whether the lunar month is a leap month
 *   fix_leap       - Whether to fix leap month
 *   language       - "zh-CN", "zh-TW", "en-US", "ja-JP", "ko-KR", or "vi-VN"
 *   config_json    - NULL/empty for defaults, or a partial-config JSON
 *
 * Returns: JSON string (must be freed with iztro_free_string)
 */
char* iztro_by_lunar(
    const char* lunar_date,
    uint8_t time_index,
    const char* gender,
    bool is_leap_month,
    bool fix_leap,
    const char* language,
    const char* config_json
);

/*
 * Calculate horoscope data for a birth chart and a target date.
 *
 * Stateless interface: the birth chart is recomputed from its parameters,
 * no chart JSON round-trip is needed.
 *
 * Parameters:
 *   solar_date        - Birth date string, e.g. "2000-8-16"
 *   time_index        - Birth time index (0-12)
 *   gender            - "male" or "female"
 *   fix_leap          - Whether to fix leap month
 *   language          - "zh-CN", "zh-TW", "en-US", "ja-JP", "ko-KR", or "vi-VN"
 *   config_json       - NULL/empty for defaults, or a partial-config JSON
 *   target_date       - Target date string, e.g. "2024-1-1"
 *   target_time_index - Target time index (0-12)
 *
 * Returns: JSON string (must be freed with iztro_free_string)
 */
char* iztro_get_horoscope(
    const char* solar_date,
    uint8_t time_index,
    const char* gender,
    bool fix_leap,
    const char* language,
    const char* config_json,
    const char* target_date,
    uint8_t target_time_index
);

/*
 * Run a lightweight query; the `kind` field of the input JSON selects which
 * one. Covers the astro lightweight queries, astro/palace, util, star, the
 * data tables, the i18n translation/lookup helpers and the AI prompt
 * generators -- one entry point instead of one symbol per function.
 *
 * Parameters:
 *   query_json - Query object, e.g. {"kind":"getPalaceNames","soulIndex":0}.
 *                Keys are camelCase; identifiers (stars, stems, branches,
 *                palaces, ...) are passed and returned as language-independent
 *                iztro i18n keys.
 *
 * Returns: JSON string {"value": <result>} (must be freed with iztro_free_string)
 */
char* iztro_query(const char* query_json);

/*
 * Free a string returned by any iztro_* function.
 * Passing NULL is safe (no-op).
 */
void iztro_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* X_IZTRO_H */
