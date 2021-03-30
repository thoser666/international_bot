mod configuration_manager
{
    pub fn get_configuration()
    {
        let _botlogin_txt_location = String::from("./config/botlogin.txt");
        let _panel_standard_user = String::from("panel");
        let _panel_standard_password = String::from("panel");
        let _ouauth_prefix = String::from("oauth:");

        let _prop_baseport = String::from("baseport");
        let _prop_usehttps = String::from("usehttps");
        let _prop_webenable = String::from("webenable");
        let _prop_msglimit30 = String::from("msglimit30");
        let _prop_musicenable = String::from("musicenable");
        let _prop_whisperlimit60 = String::from("whisperlimit60");
        let _prop_oauth = String::from("oauth");
        let _prop_channel = String::from("channel");
        let _prop_owner = String::from("owner");
        let _prop_user = String::from("user");
        let _prop_debugon = String::from("debugon");
        let _prop_debuglog = String::from("debuglog");
        let _prop_reloadscripts = String::from("reloadscripts");
        let _prop_rhinodebugger = String::from("rhinodebugger");
        let _prop_webauth = String::from("webauth");
        let _prop_webauth_ro = String::from("webauthro");
        let _prop_panel_user = String::from("paneluser");
        let _prop_panel_password = String::from("panelpassword");
        let _prop_ytauth = String::from("ytauth");
        let _prop_ytauth_ro = String::from("ytauthro");
        let _prop_api_oauth = String::from("apioauth");
        let _prop_silentscriptsload = String::from("silentscriptsload");
    }
}