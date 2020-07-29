use actions::auth::{check_no_csrf, check_no_db, get_firebase_id};
use actix_web::{
    http::{header, HeaderMap, HeaderValue},
    web::Data,
    FromRequest, HttpMessage, HttpResponse,
};
use config::{COOKIE_DOMAIN, MAX_SIGNIN_COOKIE};
use core::settings::SETTINGS;
use futures::future::FutureExt;
use futures_util::future::BoxFuture;
use jsonwebtoken as jwt;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use shared::{
    auth::{AuthClaims, RegisterSuccess, SigninSuccess, CSRF_HEADER_NAME, JWT_COOKIE_NAME},
    user::UserRole,
};
use sqlx::postgres::PgPool;

pub struct FirebaseUser {
    pub id: String,
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let header: &HeaderValue = headers.get(header::AUTHORIZATION)?;

    let header: &str = header.to_str().ok()?;

    // ["Bearer " .. value]
    header.split("Bearer ").nth(1)
}

pub struct AuthError;

impl From<AuthError> for actix_web::Error {
    fn from(_other: AuthError) -> Self {
        HttpResponse::Unauthorized().into()
    }
}

impl FromRequest for FirebaseUser {
    type Error = AuthError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // this whole dance is to avoid cloning the headers.
        let token = match bearer_token(req.headers()) {
            Some(token) => token.to_owned(),
            None => return futures::future::err(AuthError).boxed(),
        };

        async move {
            get_firebase_id(&token)
                .await
                .map(|id| Self { id })
                .map_err(|_| AuthError)
        }
        .boxed()
    }
}

fn csrf_header(headers: &HeaderMap) -> Option<&str> {
    headers.get(CSRF_HEADER_NAME)?.to_str().ok()
}

#[repr(transparent)]
pub struct WrapAuthClaimsNoDb(pub AuthClaims);

impl FromRequest for WrapAuthClaimsNoDb {
    type Error = AuthError;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let cookie = req.cookie(JWT_COOKIE_NAME);
        let csrf = csrf_header(req.headers());

        let (cookie, csrf) = match (cookie, csrf) {
            (Some(cookie), Some(csrf)) => (cookie, csrf),
            _ => return futures::future::err(AuthError),
        };

        futures::future::ready(
            // todo: check this use of cookie.name()
            check_no_db(cookie.name(), csrf)
                .map(Self)
                .map_err(|_| AuthError),
        )
    }
}

#[repr(transparent)]
pub struct WrapAuthClaimsCookieDbNoCsrf(pub AuthClaims);

impl FromRequest for WrapAuthClaimsCookieDbNoCsrf {
    type Error = AuthError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let db: &Data<PgPool> = req.app_data().unwrap();
        let db = db.as_ref().clone();

        let cookie = match req.cookie(JWT_COOKIE_NAME) {
            Some(token) => token.to_owned(),
            None => return futures::future::err(AuthError).boxed(),
        };

        async move {
            // todo: check this use of cookie.name()
            check_no_csrf(&db, &cookie.name())
                .await
                .map(Self)
                .map_err(|_| AuthError)
        }
        .boxed()
    }
}

pub fn reply_signin_auth2(
    user_id: String,
    roles: Vec<UserRole>,
    is_register: bool,
) -> actix_web::Result<HttpResponse> {
    let csrf: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    let claims = AuthClaims {
        id: user_id,
        csrf: Some(csrf.clone()),
        roles,
    };

    let jwt = jwt::encode(
        &jwt::Header::default(),
        &claims,
        &SETTINGS.get().unwrap().jwt_encoding_key,
    )
    .map_err(|_| HttpResponse::InternalServerError())?;

    let set_cookie = if SETTINGS.get().unwrap().local_insecure {
        format!(
            "{}={}; HttpOnly; SameSite=Lax; Max-Age={}",
            JWT_COOKIE_NAME, jwt, MAX_SIGNIN_COOKIE
        )
    } else {
        format!(
            "{}={}; Secure; HttpOnly; SameSite=Lax; Max-Age={}; domain={}",
            JWT_COOKIE_NAME, jwt, MAX_SIGNIN_COOKIE, COOKIE_DOMAIN
        )
    };

    if is_register {
        Ok(HttpResponse::Created()
            .header(header::SET_COOKIE, set_cookie)
            .json(RegisterSuccess::Signin(csrf)))
    } else {
        Ok(HttpResponse::Ok()
            .header(header::SET_COOKIE, set_cookie)
            .json(SigninSuccess { csrf }))
    }
}
