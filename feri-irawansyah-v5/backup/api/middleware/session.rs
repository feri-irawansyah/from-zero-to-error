#[cfg(feature = "ssr")]
pub mod ssr {
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};
    use chrono::{Duration, Utc};

    use crate::api::models::auth_model::User;

    const SECRETS: &str = "superbanget1234567";

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Claims {
        pub result: bool,
        pub usernid: i32,
        pub email: String,
        pub fullname: String,
        pub disabled_login: bool,
        pub expired_token: i64,
        pub expired_date: String,
        pub register_date: chrono::DateTime<chrono::Utc>,
        pub exp: usize,
        pub picture: Option<String>,
        pub comp_name: Option<String>,
        pub ip_address: Option<String>,
        pub app_name: Option<String>,
    }

    impl From<User> for Claims {
        fn from(user: User) -> Self {
            let expired_token = Utc::now() + Duration::days(2); // Token berlaku 2 hari
            let expired_date = expired_token.format("%Y-%m-%d %H:%M:%S").to_string();
            let exp = expired_token.timestamp() as usize; // ⏳ Set exp untuk validasi JWT

            Self {
                result: true,
                email: user.email,
                fullname: user.fullname,
                expired_token: expired_token.timestamp(),
                expired_date,
                disabled_login: user.disabled_login,
                usernid: user.usernid,
                picture: user.picture,
                register_date: user.register_date,
                exp,
                comp_name: user.comp_name,
                ip_address: user.ip_address,
                app_name: user.app_name,
            }
        }
    }

    // 🔥 Generate JWT Token
    pub fn create_jwt(user: Claims) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::from(user); // 🔥 Clone user di sini
        let mut header = Header::default();
        header.alg = Algorithm::HS256; // ✅ Set eksplisit algoritma HS256
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(SECRETS.as_bytes()),
        )?;
        Ok(token)
    }

    // 🔥 Validate JWT Token
    pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRETS.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => {
                let claims = token_data.claims;
                let now = Utc::now().timestamp() as usize;

                if claims.exp < now {
                    return Err(jsonwebtoken::errors::Error::from(
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature,
                    ));
                }

                Ok(claims)
            }
            Err(err) => {
                println!("❌ JWT Validation Error: {:?}", err);
                Err(err)
            }
        }
    }
    
}