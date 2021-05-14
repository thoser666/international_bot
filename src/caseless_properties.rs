pub struct CaselessProperties
{
    prop_baseport : String,
    prop_usehttps : String,
    prop_webenable : String,
    prop_msglimit_30 : String,
    prop_musicenable : String,
    prop_whisperlimit_60 : String
}

impl CaselessProperties
{
    pub fn new() -> Self
    {
        CaselessProperties{
            prop_baseport: "".to_string(),
            prop_usehttps: "".to_string(),
            prop_webenable: "".to_string(),
            prop_msglimit_30: "".to_string(),
            prop_musicenable: "".to_string(),
            prop_whisperlimit_60: "".to_string()
        }
    }
}
