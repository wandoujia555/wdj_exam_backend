/// The request message containing the user's name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GreeterRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response message containing the greetings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GreeterReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// The response message containing the greetings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginReply {
    #[prost(oneof = "login_reply::Message", tags = "1, 2")]
    pub message: ::core::option::Option<login_reply::Message>,
}
/// Nested message and enum types in `LoginReply`.
pub mod login_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(bool, tag = "1")]
        IsLogin(bool),
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
    }
}
/// Generated client implementations.
pub mod greeter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use dubbo::codegen::*;
    #[derive(Debug, Clone, Default)]
    pub struct GreeterClient {
        inner: TripleClient,
        uri: String,
    }
    impl GreeterClient {
        pub fn new() -> Self {
            Self {
                inner: TripleClient::new(),
                uri: "".to_string(),
            }
        }
        pub fn with_uri(mut self, uri: String) -> Self {
            self.uri = uri.clone();
            self.inner = self.inner.with_host(uri);
            self
        }
        /// unary
        pub async fn greet(
            &mut self,
            request: Request<super::GreeterRequest>,
        ) -> Result<Response<super::GreeterReply>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::GreeterRequest,
                super::GreeterReply,
            >::default();
            let path = http::uri::PathAndQuery::from_static("/greeter.Greeter/greet");
            self.inner.unary(request, codec, path).await
        }
        pub async fn authenticate(
            &mut self,
            request: Request<super::LoginRequest>,
        ) -> Result<Response<super::LoginReply>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::LoginRequest,
                super::LoginReply,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/greeter.Greeter/authenticate",
            );
            self.inner.unary(request, codec, path).await
        }
    }
}
/// Generated server implementations.
pub mod greeter_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use dubbo::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with GreeterServer.
    #[async_trait]
    pub trait Greeter: Send + Sync + 'static {
        /// unary
        async fn greet(
            &self,
            request: Request<super::GreeterRequest>,
        ) -> Result<Response<super::GreeterReply>, dubbo::status::Status>;
        async fn authenticate(
            &self,
            request: Request<super::LoginRequest>,
        ) -> Result<Response<super::LoginReply>, dubbo::status::Status>;
    }
    #[derive(Debug)]
    pub struct GreeterServer<T: Greeter, I = TripleInvoker> {
        inner: _Inner<T>,
        invoker: Option<I>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Greeter, I> GreeterServer<T, I> {
        pub fn new(inner: T) -> Self {
            Self {
                inner: _Inner(Arc::new(inner)),
                invoker: None,
            }
        }
    }
    impl<T, I, B> Service<http::Request<B>> for GreeterServer<T, I>
    where
        T: Greeter,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
        I: Invoker + Send + 'static,
    {
        type Response = http::Response<BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/greeter.Greeter/greet" => {
                    #[allow(non_camel_case_types)]
                    struct greetServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::GreeterRequest> for greetServer<T> {
                        type Response = super::GreeterReply;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::GreeterRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move { inner.greet(request).await };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::GreeterReply,
                                super::GreeterRequest,
                            >::default(),
                        );
                        let res = server.unary(greetServer { inner }, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/greeter.Greeter/authenticate" => {
                    #[allow(non_camel_case_types)]
                    struct authenticateServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::LoginRequest>
                    for authenticateServer<T> {
                        type Response = super::LoginReply;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move { inner.authenticate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::LoginReply,
                                super::LoginRequest,
                            >::default(),
                        );
                        let res = server.unary(authenticateServer { inner }, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Greeter, I: Invoker + Send + 'static> Clone for GreeterServer<T, I> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner, invoker: None }
        }
    }
    impl<T: Greeter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    pub fn register_server<T: Greeter>(server: T) {
        let s = GreeterServer::<_, TripleInvoker>::new(server);
        dubbo::protocol::triple::TRIPLE_SERVICES
            .write()
            .unwrap()
            .insert(
                "greeter.Greeter".to_string(),
                dubbo::utils::boxed_clone::BoxCloneService::new(s),
            );
    }
}
