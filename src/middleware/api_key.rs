use std::future::{ready, Ready};
use std::rc::Rc;

use crate::AppState;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct ApiKey {
    api_key: Rc<String>,
}

impl ApiKey {
    pub fn new(api_key: String) -> Self {
        ApiKey {
            api_key: Rc::new(api_key),
        }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for ApiKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyMiddlware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddlware { service }))
    }
}

pub struct ApiKeyMiddlware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlware<S>
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
        // let bind = req.app_data::<AppState>().unwrap().clone();
        // let keys = bind.api_key.lock().unwrap();

        // println!("Hi from start. You requested: {}", req.path());

        // println!("{:#?}", keys);

        // verify API key
        // println!("{:#?}", req.headers());

        // if let hvalue = Some(req.headers().get("x-test-header")) {
        //     println!("Value: {}", hvalue.unwrap().unwrap().to_str().unwrap().to_string());
        // }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            // println!("Hi from response");
            Ok(res)
        })
    }
}
