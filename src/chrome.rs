pub fn get_active_tab_url() -> Option<String> {
    let script: osascript::JavaScript = osascript::JavaScript::new(
        "
        var Chrome = Application('Google Chrome');
        Chrome.includeStandardAdditions = true;
        return Chrome.windows[0].activeTab().url();
    ",
    );

    script.execute().ok()
}
