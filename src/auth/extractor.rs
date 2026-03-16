use hmac::Hmac;
use uuid::Uuid;

struct AuthUser{
    user_id: Uuid,
}
struct AuthUserClaims {
    user_id: Uuid,
    exp: u64,
}

impl AuthUser{
    pub fn to_jwt(&self, state: &AppState) -> String {
        let hmac =
    }
}