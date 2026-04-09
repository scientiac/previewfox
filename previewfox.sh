#!/usr/bin/env bash

set -euo pipefail

# Configuration
URL="${1:-about:logo}"
PROFILE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/ff-previewer-profile"

cleanup() {
    if [[ -n "${FIREFOX_PID:-}" ]]; then
        kill "$FIREFOX_PID" 2>/dev/null
        wait "$FIREFOX_PID" 2>/dev/null || true
    fi
    exit
}

trap cleanup SIGINT SIGTERM EXIT

# Profile Generation
create_minimal_profile() {
    # You can 'rm -rf ~/.cache/ff-previewer' to force a rebuild
    if [[ -f "$PROFILE_DIR/user.js" ]]; then
        return
    fi

    mkdir -p "$PROFILE_DIR/chrome"

    # user.js
    cat <<EOF > "$PROFILE_DIR/user.js"
/* 1. Disable Skeletons & UI Junk */
user_pref("toolkit.cosmeticAnimations.enabled", false);
user_pref("layout.css.skeletons.enabled", false);
user_pref("browser.aboutHome.skeleton", false);
user_pref("browser.newtabpage.activity-stream.skeleton", false);

/* 2. Disable First Run / Welcome / Privacy Policy Popups */
user_pref("browser.aboutWelcome.enabled", false);
user_pref("browser.startup.homepage_override.mstone", "ignore");
user_pref("browser.shell.checkDefaultBrowser", false);
user_pref("privacy.sanitize.sanitizeOnShutdown", true);
user_pref("browser.contentblocking.introCount", 20);
user_pref("browser.download.panel.shown", true);
user_pref("browser.engagement.startup_button.has_been_opened", true);
user_pref("browser.messaging-system.whatsNewPanel.enabled", false);

/* 3. Disable Telemetry & Data Collection */
user_pref("datareporting.healthreport.uploadEnabled", false);
user_pref("datareporting.policy.dataSubmissionEnabled", false);
user_pref("toolkit.telemetry.enabled", false);
user_pref("toolkit.telemetry.unified", false);
user_pref("toolkit.telemetry.archive.enabled", false);
user_pref("toolkit.telemetry.newProfilePing.enabled", false);
user_pref("toolkit.telemetry.shutdownPingSender.enabled", false);
user_pref("toolkit.telemetry.updatePing.enabled", false);
user_pref("toolkit.telemetry.bhrPing.enabled", false);
user_pref("toolkit.telemetry.firstShutdownPing.enabled", false);
user_pref("browser.newtabpage.activity-stream.feeds.telemetry", false);
user_pref("browser.newtabpage.activity-stream.telemetry", false);

/* 4. UI Cleanliness & Bookmarks */
user_pref("toolkit.legacyUserProfileCustomizations.stylesheets", true);
user_pref("browser.toolbars.bookmarks.visibility", "never");
user_pref("browser.bookmarks.addedImportButton", false);
user_pref("browser.newtabpage.enabled", false);
user_pref("browser.newtabpage.activity-stream.showSponsored", false);
user_pref("browser.newtabpage.activity-stream.showSponsoredTopSites", false);
user_pref("browser.newtabpage.activity-stream.feeds.section.topstories", false);

/* FORCE NEW WINDOWS INSTEAD OF TABS */
user_pref("browser.link.open_newwindow", 2);
user_pref("browser.link.open_newwindow.override.external", 2);
user_pref("browser.link.open_newwindow.restriction", 0);
user_pref("browser.tabs.loadInBackground", true)

/* 5. Performance/Behavior */
user_pref("browser.tabs.drawInTitlebar", true);
user_pref("browser.sidebar.position", "none");
EOF

    # userChrome.css
    cat <<EOF > "$PROFILE_DIR/chrome/userChrome.css"
#TabsToolbar, 
#nav-bar, 
#urlbar-container,
#PersonalToolbar,
#navigator-toolbox,
#titlebar {
    visibility: collapse !important;
    margin: 0 !important;
    padding: 0 !important;
    height: 0 !important;
}
EOF

    # 3. userContent.css
    cat <<EOF > "$PROFILE_DIR/chrome/userContent.css"
* {
    scrollbar-width: none !important;
}
EOF

    # 4. customKeys
    cat <<EOF > "$PROFILE_DIR/customKeys.json"
{"key_newNavigatorTab":{},"key_showAllTabs":{},"key_restoreLastClosedTabOrWindowOrSession":{}}
EOF

}

# Execution
create_minimal_profile

# --no-remote to ensure it doesn't try to merge with the main firefox profile
firefox --new-window --profile "$PROFILE_DIR" --no-remote "$URL" >/dev/null 2>&1 &
FIREFOX_PID=$!

wait "$FIREFOX_PID" || true
