const PRODUCT_KEY: &str = "ewFK19b0ndPtKE7aygc6YEB15QGXJXpl1xOs/ScqXTLeKoFVSNxSF5C#gg47BL0VN+M3TjeUAuzYnz37wYYH1z6doUiA6WZbkzgZaSm#pI=uBtY#zNE#49kvnmXg22nMyMyO4ZUtxYf76w//MftvX9VpxvE#tKnT4c+MlMbcb6xaAZFO77vbYYhOgiyY7uEGo/0h71AG4amNnYFpC+CACi4Po=I4mtbr0Q#YDTeZinxSH2MPOKZ27cP/+YVEB7f0+LipEOWi=E5q06Lyl8QDRYpV0A6iTxVDLZre=tWx+jxZ";
const DEV_KEY: &str = "Kc85m5cC#ETEnYRl2iWx6pdAx37aNzpw3/5P3x9=YoiiDEjAG88+s/#DzNLkHIWNYUsOCWC2pC5uSY+Om0xWPxpUUrMz0Sw3k53YuI55vm#6orypkY8m2iptap/RaobJl6PvNAs7VKv_codeu+l+PRWnwNPm7vMC7bB0b/Kp/R3rubqgcgVg54jQajaW+MJ14PoshNzMZZJ=xHtK#VPv9Tumpq#C2DWq2tBH4a9Vmrs3#fIgDsxKr8A8nNO8LnOKSm9UzLOUG0DXDfTzs8qiUVwZWa4IVP9gDUcCZTfdKkOxvcxvI";

pub mod crypto {
    use magic_crypt::{ MagicCryptError, MagicCryptTrait };
    use crate::security::{ PRODUCT_KEY, DEV_KEY };

    #[allow(dead_code)]
    pub fn encrypt(data: &str, dev: bool) -> String {
        return if dev {
            let mcrypt = new_magic_crypt!(&DEV_KEY, 256);
            mcrypt.encrypt_str_to_base64(data)
        } else {
            let mcrypt = new_magic_crypt!(&PRODUCT_KEY, 256);
            mcrypt.encrypt_str_to_base64(data)
        }
    }

    pub fn decrypt(data: &str, dev: bool) -> Result<String, MagicCryptError> {
        return if dev {
            let mcrypt = new_magic_crypt!(&DEV_KEY, 256);
            mcrypt.decrypt_base64_to_string(data)
        } else {
            let mcrypt = new_magic_crypt!(&PRODUCT_KEY, 256);
            mcrypt.decrypt_base64_to_string(data)
        }
    }
}