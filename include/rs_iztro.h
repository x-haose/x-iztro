/*
 * rs_iztro C FFI header
 *
 * Purple Star Astrology (紫微斗数) library.
 *
 * All returned strings are heap-allocated JSON. The caller MUST free them
 * with iztro_free_string(). Functions never return NULL; on error they
 * return a JSON string of the form {"error": "message"}.
 *
 * Result JSON carries two layers per field: translated display text
 * (name, brightness, ...) in the requested language, and
 * language-independent identifiers (key, nameKey, mutagenKey, ... using
 * iztro i18n keys such as "ziweiMaj"/"soulPalace"/"sihuaLu") for
 * programmatic matching that works in every output language.
 */

#ifndef RS_IZTRO_H
#define RS_IZTRO_H

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
 *   language    - "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
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
 *   language       - "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
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
 *   language          - "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
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
 * Free a string returned by any iztro_* function.
 * Passing NULL is safe (no-op).
 */
void iztro_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* RS_IZTRO_H */
