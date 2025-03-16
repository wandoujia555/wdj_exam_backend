use actix_web::HttpResponse;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub code: i32,
    pub password: String,
}
use chrono::Duration;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::{decode, encode, Header, Validation};
use serde_json::json;

use std::env;

use once_cell::sync::Lazy;
static SECRET_KEY: Lazy<String> =
    Lazy::new(|| env::var("JWT_SECRET_KEY").unwrap_or_else(|_| "123456".to_string()));
// 生成jwt
pub fn generate_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    // let secret_key = b"my_secret_key"; // 使用安全的密钥管理方式
    let secret_key = SECRET_KEY.as_bytes(); // 使用安全的密钥管理方式
    let expiration = Utc::now() + Duration::minutes(600); // 设置 JWT 的过期时间


    let claims = json!({
        "code": user.code,
        "exp": expiration.timestamp()
    });
    
    println!("{:?}",claims);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
}
// 验证jwt
fn verify_jwt(token: &str) -> Result<serde_json::Value, jsonwebtoken::errors::Error> {
    // let secret_key = b"my_secret_key"; // 使用相同的密钥
    let secret_key = SECRET_KEY.as_bytes(); // 使用相同的密钥
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = true; // 验证过期时间

    decode::<serde_json::Value>(token, &DecodingKey::from_secret(secret_key), &validation)
        .map(|data| data.claims)
        .map_err(|err| err.into())
}

use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        if req.path() == "/login" || req.path() == "/" || req.path() == "/test" {
            let fut = self.service.call(req);
            return Box::pin(fut);
        }
        // 获取 Authorization 头部
        let auth_header = req.headers().get("Authorization");
        if auth_header.is_none() {
            // 缺少 Authorization 头部
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "Authorization header is missing",
                ))
            });
        }
        let auth_header_value = auth_header.unwrap().to_str();
        if auth_header_value.is_err() {
            // 无效的头部格式
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "Authorization header is missing",
                ))
            });
        }
        let token = auth_header_value.unwrap().strip_prefix("Bearer ");
        if token.is_none() {
            // 无效的令牌格式
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "Authorization header is missing",
                ))
            });
        }
        let token_value = token.unwrap();

        match verify_jwt(token_value) {
            Ok(_) => {
                // 验证成功，继续处理请求
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => {
                // 验证失败，返回错误
                Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(
                        "Authorization header is missing",
                    ))
                })
            }
        }
    }
}
