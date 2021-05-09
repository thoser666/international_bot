pub struct  ConfigurationManager
{
    BOTLOGIN_TXT_LOCATION : String,
    PANEL_STANDARD_USER : String,
    PANEL_STANDARD_PASSWORD : String,
    OUAUTH_PREFIX : String,
    PROP_BASEPORT : String,
    PROP_USEHTTPS : String,
    PROP_WEBENABLE : String,
    PROP_MSGLIMIT30 : String,
    PROP_MUSICENABLE : String,
    PROP_WHISPERLIMIT60 : String,
    PROP_OAUTH : String,
    PROP_CHANNEL : String,
    PROP_OWNER : String,
    PROP_USER : String,
    PROP_DEBUGON : String,
    PROP_DEBUGLOG : String,
    PROP_RELOADSCRIPTS : String,
    PROP_RHINODEBUGGER : String,
    PROP_WEBAUTH : String,
    PROP_WEBAUTH_RO : String,
    PROP_PANEL_USER : String,
    PROP_PANEL_PASSWORD : String,
    PROP_YTAUTH : String,
    PROP_YTAUTH_RO : String,
    PROP_API_OAUTH : String,
    PROP_SILENTSCRIPTSLOAD : String
}

impl ConfigurationManager
{
    pub fn new() -> Self
    {
        ConfigurationManager {
            BOTLOGIN_TXT_LOCATION: "./config/botlogin.txt".to_string(),
            PANEL_STANDARD_USER: "panel".to_string(),
            PANEL_STANDARD_PASSWORD: "panel".to_string(),
            OUAUTH_PREFIX: "oauth:".to_string(),
            PROP_BASEPORT: "baseport".to_string(),
            PROP_USEHTTPS: "usehttps".to_string(),
            PROP_WEBENABLE: "webenable".to_string(),
            PROP_MSGLIMIT30: "msglimit30".to_string(),
            PROP_MUSICENABLE: "musicenable".to_string(),
            PROP_WHISPERLIMIT60: "whisperlimit60".to_string(),
            PROP_OAUTH: "oauth".to_string(),
            PROP_CHANNEL: "channel".to_string(),
            PROP_OWNER: "owner".to_string(),
            PROP_USER: "user".to_string(),
            PROP_DEBUGON: "debugon".to_string(),
            PROP_DEBUGLOG: "debuglog".to_string(),
            PROP_RELOADSCRIPTS: "reloadscripts".to_string(),
            PROP_RHINODEBUGGER: "rhinodebugger".to_string(),
            PROP_WEBAUTH: "webauth".to_string(),
            PROP_WEBAUTH_RO: "webauthro".to_string(),
            PROP_PANEL_USER: "paneluser".to_string(),
            PROP_PANEL_PASSWORD: "panelpassword".to_string(),
            PROP_YTAUTH: "ytauth".to_string(),
            PROP_YTAUTH_RO: "ytauthro".to_string(),
            PROP_API_OAUTH: "apioauth".to_string(),
            PROP_SILENTSCRIPTSLOAD: "silentscriptsload".to_string()
        }
    }
}


//    pub fn get_configuration()
//    {
        // let _botlogin_txt_location = String::from("./config/botlogin.txt");
        // let _panel_standard_user = String::from("panel");
        // let _panel_standard_password = String::from("panel");
        // let _ouauth_prefix = String::from("oauth:");
        //
        // let _prop_baseport = String::from("baseport");
        // let _prop_usehttps = String::from("usehttps");
        // let _prop_webenable = String::from("webenable");
        // let _prop_msglimit30 = String::from("msglimit30");
        // let _prop_musicenable = String::from("musicenable");
        // let _prop_whisperlimit60 = String::from("whisperlimit60");
        // let _prop_oauth = String::from("oauth");
        // let _prop_channel = String::from("channel");
        // let _prop_owner = String::from("owner");
        // let _prop_user = String::from("user");
        // let _prop_debugon = String::from("debugon");
        // let _prop_debuglog = String::from("debuglog");
        // let _prop_reloadscripts = String::from("reloadscripts");
        // let _prop_rhinodebugger = String::from("rhinodebugger");
        // let _prop_webauth = String::from("webauth");
        // let _prop_webauth_ro = String::from("webauthro");
        // let _prop_panel_user = String::from("paneluser");
        // let _prop_panel_password = String::from("panelpassword");
        // let _prop_ytauth = String::from("ytauth");
        // let _prop_ytauth_ro = String::from("ytauthro");
        // let _prop_api_oauth = String::from("apioauth");
        // let _prop_silentscriptsload = String::from("silentscriptsload");
//    }
//}