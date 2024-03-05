use std::{
    env,
    sync::{Arc, Mutex},
    collections::{HashMap, BTreeMap},
    time::{SystemTime, UNIX_EPOCH},
};

use bcrypt::verify;
use diesel::PgConnection;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use protos::auth::{auth_server::Auth, LoginRequest, Token, RegisterRequest};
use sha2::Sha256;
use tonic::{Request, Response, Status};

use crate::models::{NewUser, User};

pub struct Service {
    db: Arc<Mutex<PgConnection>>,
}

impl Service {
    pub fn new(db: PgConnection) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
        }
    }
}

const INV: &'static str = "Invalid username or password";

type AuthResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Auth for Service {
    async fn login(&self, request: Request<LoginRequest>) -> AuthResult<Token> {
        let db = self.db.lock();
        let data = request.into_inner();

        let user = User::find_by_username(&mut db.unwrap(), &data.username)
            .ok_or(Status::unauthenticated(INV))?;

        match verify(data.password, &user.password) {
            Ok(true) => (),
            Ok(false) | Err(_) => return Err(Status::unauthenticated(INV)),
        };

        let reply = generate_token(user).map_err(|_| Status::unauthenticated(INV))?;
        Ok(Response::new(reply))
    }

    async fn register(&self, request: Request<RegisterRequest>) -> AuthResult<Token> {
        let db = self.db.lock();
        let data = request.into_inner();

        let pw = bcrypt::hash(&data.password, 10).map_err(|_| Status::unknown("ERROR: invalid pw"))?;

        let user = User::create(&mut db.unwrap(), NewUser {
            username: &data.username,
            password: &pw,
        }).map_err(|_| Status::already_exists("User already exists"))?;

        let token = generate_token(user).map_err(|_| Status::unknown("ERROR: generating token"))?;

        Ok(Response::new(token))
    }
}

struct GenerateTokenError;
struct GenerateClaimsError;

pub struct VerifyTokenError;

pub fn verify_token(token: &str) -> Result<bool, VerifyTokenError> {
    let app_key: String = env::var("APP_KEY").expect("evn APP_KEY is not defined");
    let key: Hmac<Sha256> = Hmac::new_from_slice(app_key.as_bytes()).map_err(|_| VerifyTokenError)?;

    Ok(token
        .verify_with_key(&key)
        .map(|_: HashMap<String, String>| true)
        .unwrap_or(false))
}

fn generate_token(user: User) -> Result<Token, GenerateTokenError> {
    let app_key: String = env::var("APP_KEY").expect("env APP_KEY is not defined");
    let key: Hmac<Sha256> = Hmac::new_from_slice(app_key.as_bytes()).map_err(|_| GenerateTokenError)?;
    let claims = generate_claims(user).map_err(|_| GenerateTokenError)?;

    Ok(Token { access_token: claims.sign_with_key(&key).map_err(|_| GenerateTokenError)? })
}

fn generate_claims(user: User) -> Result<BTreeMap<&'static str, String>, GenerateClaimsError> {
    let mut claims: BTreeMap<&str, String> = BTreeMap::from([("sub", user.username.clone())]);

    let cur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| GenerateClaimsError)?
        .as_secs();

    claims.insert("iat", cur.to_string());
    claims.insert("exp", String::from("3600"));

    Ok(claims)
}
