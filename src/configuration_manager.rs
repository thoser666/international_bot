use crate::caseless_properties::CaselessProperties;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io::Write;
use std::fs;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::io::prelude::*;
use std::ops::Add;

pub struct  ConfigurationManager
{
    botlogin_txt_location : String,
    panel_standard_user : String,
    panel_standard_password : String,
    ouauth_prefix : String,
    prop_baseport : String,
    prop_usehttps : String,
    prop_webenable : String,
    prop_msglimit_30 : String,
    prop_musicenable : String,
    prop_whisperlimit_60 : String,
    prop_oauth : String,
    prop_channel : String,
    prop_owner : String,
    prop_user : String,
    prop_debugon : String,
    prop_debuglog : String,
    prop_reloadscripts : String,
    prop_rhinodebugger : String,
    prop_webauth : String,
    prop_webauth_ro : String,
    prop_panel_user : String,
    prop_panel_password : String,
    prop_ytauth : String,
    prop_ytauth_ro : String,
    prop_api_oauth : String,
    prop_silentscriptsload : String
}

impl ConfigurationManager
{
    pub fn new() -> Self
    {
        ConfigurationManager {
            botlogin_txt_location: "./config/botlogin.txt".to_string(),
            panel_standard_user: "panel".to_string(),
            panel_standard_password: "panel".to_string(),
            ouauth_prefix: "oauth:".to_string(),
            prop_baseport: "baseport".to_string(),
            prop_usehttps: "usehttps".to_string(),
            prop_webenable: "webenable".to_string(),
            prop_msglimit_30: "msglimit30".to_string(),
            prop_musicenable: "musicenable".to_string(),
            prop_whisperlimit_60: "whisperlimit60".to_string(),
            prop_oauth: "oauth".to_string(),
            prop_channel: "channel".to_string(),
            prop_owner: "owner".to_string(),
            prop_user: "user".to_string(),
            prop_debugon: "debugon".to_string(),
            prop_debuglog: "debuglog".to_string(),
            prop_reloadscripts: "reloadscripts".to_string(),
            prop_rhinodebugger: "rhinodebugger".to_string(),
            prop_webauth: "webauth".to_string(),
            prop_webauth_ro: "webauthro".to_string(),
            prop_panel_user: "paneluser".to_string(),
            prop_panel_password: "panelpassword".to_string(),
            prop_ytauth: "ytauth".to_string(),
            prop_ytauth_ro: "ytauthro".to_string(),
            prop_api_oauth: "apioauth".to_string(),
            prop_silentscriptsload: "silentscriptsload".to_string()
        }
    }

    pub fn get_configuration(&self)
    {
        enum RequiredProperties
        {
            PropOauth,
            PropChannel,
            PropOwner,
            PropUser
        }
        let required_properties_error_message : String;

        let mut conf = HashMap::new();


        // Configure Properties
        let start_properties = CaselessProperties::new();

        // Indicates that botlogin.txt should be created or updated
        let changed :bool = false;

        // is this a fresh setup
        let new_setup: bool = false;

        // loading the config file or (if not there) create a new one with default values
        let dateiname = &self.botlogin_txt_location;
        let result: HashMap<String, String> = HashMap::new();
        let mut config = String::new();     // configdaten

        let fileExists = Path::new(dateiname).exists();

        //config file exists
        if fileExists
        {
            println!("Test");
        }
        else
        {

            // create config file
            let slice = &dateiname[..8];
            fs::create_dir(slice);
            let mut file = File::create(dateiname);

            conf.insert(&self.prop_webenable, "true");


             for (key, value) in conf.iter()
             {
                 config = key.to_string();
                 config += ": ";
                 config += value;
                 config += "\n";


             }
            fs::write(dateiname,config);




        }


        // Config einlesen


         let mut contents = String::new();
        // buf_reader.read_to_string(&mut contents);
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