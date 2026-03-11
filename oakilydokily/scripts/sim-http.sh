#!/bin/bash
# Copyright (c) 2026 The Cochran Block. All rights reserved.
# HTTP-based simulation: UI/UX, Feature Gap, User Analysis
# Run against BASE (default http://127.0.0.1:3000). Server must be up.
# Usage: ./scripts/sim-http.sh   or   BASE=https://oakilydokily.com ./scripts/sim-http.sh

set -e
BASE="${BASE:-http://127.0.0.1:3000}"
FAILED=0
PASSED=0

check() {
  local name="$1" url="$2" pattern="$3" desc="$4"
  local code body
  code=$(curl -s -o /tmp/sim_body -w "%{http_code}" --max-time 10 "$url" 2>/dev/null) || code="000"
  body=$(cat /tmp/sim_body 2>/dev/null || echo "")
  if [[ "$code" =~ ^2 ]] && { [[ -z "$pattern" ]] || echo "$body" | grep -q "$pattern"; }; then
    echo "  [PASS] $name"
    ((PASSED++)) || true
    return 0
  else
    echo "  [FAIL] $name (code=$code, pattern=$pattern) $desc"
    ((FAILED++)) || true
    return 1
  fi
}

echo "=== Sim UI/UX ==="
check "home_200" "$BASE/" "" "GET / 200"
check "home_identity" "$BASE/" "OakilyDokily\|Kaylie" "site identity"
check "home_serving_md" "$BASE/" "Serving Maryland" "location"
check "home_main_landmark" "$BASE/" "id=\"main\"\|<main" "main landmark"
check "home_skip_link" "$BASE/" "skip-link\|#main" "skip link"
check "about_200" "$BASE/about" "" "GET /about 200"
check "about_resume" "$BASE/about" "resume\|experience\|Kaylie" "resume content"
check "about_print" "$BASE/about" "Print Resume\|window.print" "Print Resume"
check "contact_200" "$BASE/contact" "" "GET /contact 200"
check "contact_mailto" "$BASE/contact" "mailto:" "mailto CTA"
check "contact_book_call" "$BASE/contact" "Book a Call\|Discovery" "Book a Call"
check "waiver_200" "$BASE/waiver" "" "GET /waiver 200"
check "waiver_terms" "$BASE/waiver" "agree_terms\|consent_electronic" "waiver form"
check "waiver_scroll_hint" "$BASE/waiver" "Scroll through\|scroll" "scroll hint"
check "waiver_confirmed" "$BASE/waiver/confirmed" "" "GET /waiver/confirmed (may 404 if no prior POST)"
check "health_200" "$BASE/health" "OK" "health"
check "css_200" "$BASE/assets/css/main.css" "" "CSS 200" 2>/dev/null || check "css_200" "$BASE/assets/main.css" "" "CSS 200"
check "favicon_200" "$BASE/assets/favicon.svg" "" "favicon 200" 2>/dev/null || check "favicon_200" "$BASE/favicon.svg" "" "favicon 200"

echo ""
echo "=== Sim Feature Gap ==="
check "gap_home_services" "$BASE/" "kennel\|overnight\|surgical" "service areas"
check "gap_home_flexible" "$BASE/" "Flexible\|contract\|temp" "availability"
check "gap_waiver_full_name" "$BASE/waiver" "full_name" "waiver full_name field"
check "gap_waiver_email" "$BASE/waiver" "email" "waiver email field"
check "gap_nav_waiver" "$BASE/" "Waiver\|/waiver" "nav waiver link"
check "gap_nav_about" "$BASE/" "About\|/about" "nav about link"
check "gap_nav_contact" "$BASE/" "Contact\|/contact" "nav contact link"

echo ""
echo "=== Sim User Analysis (persona flows) ==="
# Clinic Manager: Home -> services visible -> About -> Contact
check "user_clinic_home" "$BASE/" "Veterinary\|professional" "clinic: home"
check "user_clinic_about" "$BASE/about" "experience\|resume" "clinic: about"
check "user_clinic_contact" "$BASE/contact" "mailto" "clinic: contact"
# Recruiter: About -> Print Resume
check "user_recruiter_resume" "$BASE/about" "Print Resume\|experience" "recruiter: resume"
# Facility Owner: Waiver flow (GET only here)
check "user_facility_waiver" "$BASE/waiver" "terms\|agree\|consent" "facility: waiver"

echo ""
echo "=== Summary ==="
echo "Passed: $PASSED  Failed: $FAILED"
if [[ $FAILED -gt 0 ]]; then
  exit 1
fi
exit 0
