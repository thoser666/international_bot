use crate::caseless_properties::CaselessProperties;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;
use std::collections::HashMap;
use std::path::{Path};

#[derive(Default)]
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

        // defaultvalues
        conf.insert(&self.prop_musicenable, "true");
        conf.insert(&self.prop_channel, "");
        conf.insert(&self.prop_webenable, "true");
        conf.insert(&self.prop_ytauth, "");
        conf.insert(&self.prop_webauth, "");
        conf.insert(&self.prop_webauth_ro, "");
        conf.insert(&self.prop_panel_password, "");
        conf.insert(&self.prop_usehttps, "true");
        conf.insert(&self.prop_oauth, "");
        conf.insert(&self.prop_whisperlimit_60, "60.0");
        conf.insert(&self.prop_owner, "");
        conf.insert(&self.prop_baseport, "26000");
        conf.insert(&self.prop_msglimit_30,"19.0");
        conf.insert(&self.prop_ytauth_ro, "");
        conf.insert(&self.prop_panel_user, "");
        conf.insert(&self.prop_user, "");
        conf.insert(&self.prop_api_oauth, "");


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

        let file_exists = Path::new(dateiname).exists();

        //config file exists
        if file_exists
        {
            let file = File::open(dateiname).unwrap();
            let reader = BufReader::new(file);

            // doing configuration
            let mut my_defaults = ConfigurationManager::default();


            for line in reader.lines() {
                let tmp = line.unwrap();
                let prop = tmp.split(":");
                let vec: Vec<&str> = prop.collect();

                 if vec[0].to_string() == my_Defaults.prop_api_oauth.to_string()
                 {
                     my_Defaults.prop_api_oauth = vec[1].to_string();
                 }
                else if  vec[0].to_string() == my_Defaults.prop_user.to_string()
                {
                    my_Defaults.prop_user = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_panel_password.to_string()
                {
                    my_Defaults.prop_panel_password = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.panel_standard_user.to_string()
                {
                    my_Defaults.panel_standard_user = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.panel_standard_password.to_string()
                {
                    my_Defaults.panel_standard_password = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.ouauth_prefix.to_string()
                {
                    my_Defaults.ouauth_prefix = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_baseport.to_string()
                {
                    my_Defaults.prop_baseport = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_usehttps.to_string()
                {
                    my_Defaults.prop_usehttps = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_webenable.to_string()
                {
                    my_Defaults.prop_webenable = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_msglimit_30.to_string()
                {
                    my_Defaults.prop_msglimit_30 = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_musicenable.to_string()
                {
                    my_Defaults.prop_musicenable = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_whisperlimit_60.to_string()
                {
                    my_Defaults.prop_whisperlimit_60 = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_oauth.to_string()
                {
                    my_Defaults.prop_oauth = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_channel.to_string()
                {
                    my_Defaults.prop_channel = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_owner.to_string()
                {
                    my_Defaults.prop_owner = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_debugon.to_string()
                {
                    my_Defaults.prop_debugon = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_debuglog.to_string()
                {
                    my_Defaults.prop_debuglog = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_reloadscripts.to_string()
                {
                    my_Defaults.prop_reloadscripts = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_rhinodebugger.to_string()
                {
                    my_Defaults.prop_rhinodebugger = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_webauth.to_string()
                {
                    my_Defaults.prop_webauth = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_webauth_ro.to_string()
                {
                    my_Defaults.prop_webauth_ro = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_panel_user.to_string()
                {
                    my_Defaults.prop_panel_user = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_ytauth.to_string()
                {
                    my_Defaults.prop_ytauth = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_ytauth_ro.to_string()
                {
                    my_Defaults.prop_ytauth_ro = vec[1].to_string();
                }
                else if  vec[0].to_string() == my_Defaults.prop_silentscriptsload.to_string()
                {
                    myDefaults.prop_silentscriptsload = vec[1].to_string();
                }

            }
        }
        else
        {

            // create config file
            let slice = &dateiname[..8];
            fs::create_dir(slice).expect("Cannot create dir");
            let file = File::create(dateiname);


             for (key, value) in conf.iter()
             {
                 config += key;
                 config += ": ";
                 config += value;
                 config += "\n";


             }
            fs::write(dateiname,config).expect("Writing file failed !");
        }


    }
}


