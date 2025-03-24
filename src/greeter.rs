/// 分页信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageInfo {
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_p_num: i32,
}
/// The request message containing the user's name.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GreeterRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response message containing the greetings
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GreeterReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// The request message containing the user's name.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub login_type: i32,
}
/// The response message containing the greetings
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginReply {
    #[prost(int32, tag = "3")]
    pub login_type: i32,
    #[prost(oneof = "login_reply::Message", tags = "1, 2")]
    pub message: ::core::option::Option<login_reply::Message>,
}
/// Nested message and enum types in `LoginReply`.
pub mod login_reply {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(bool, tag = "1")]
        IsLogin(bool),
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaperRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
/// 定义 Question 消息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Question {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(enumeration = "QuestionType", tag = "2")]
    pub question_type: i32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub score: i32,
}
/// 定义 QuestionList 消息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuestionList {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(enumeration = "QuestionType", tag = "2")]
    pub question_type: i32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub content: ::prost::alloc::vec::Vec<Question>,
    #[prost(int32, tag = "5")]
    pub question_num: i32,
    #[prost(int32, tag = "6")]
    pub total_score: i32,
}
/// 定义 Paper 消息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paper {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub question_num: i32,
    #[prost(int32, tag = "4")]
    pub minutes: i32,
    #[prost(int32, tag = "5")]
    pub status: i32,
    #[prost(int32, tag = "6")]
    pub created_time: i32,
    #[prost(int32, tag = "7")]
    pub update_time: i32,
    #[prost(int32, tag = "8")]
    pub start_time: i32,
    #[prost(message, repeated, tag = "9")]
    pub content: ::prost::alloc::vec::Vec<QuestionList>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaperInfo {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub question_num: i32,
    #[prost(int32, tag = "4")]
    pub minutes: i32,
    #[prost(int32, tag = "5")]
    pub status: i32,
    #[prost(int32, tag = "6")]
    pub created_time: i32,
    #[prost(int32, tag = "7")]
    pub update_time: i32,
    #[prost(int32, tag = "8")]
    pub start_time: i32,
    #[prost(string, tag = "9")]
    pub desc: ::prost::alloc::string::String,
    #[prost(int32, tag = "10")]
    pub total: i32,
    #[prost(int32, tag = "11")]
    pub duration: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaperInfoList {
    #[prost(message, repeated, tag = "1")]
    pub content: ::prost::alloc::vec::Vec<PaperInfo>,
    #[prost(int32, tag = "2")]
    pub total: i32,
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    #[prost(int32, tag = "4")]
    pub page_num: i32,
}
/// 保存答案
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerPaper {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub user_id: i32,
    #[prost(int32, tag = "3")]
    pub paper_id: i32,
    #[prost(int32, tag = "4")]
    pub answer_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerReply {
    #[prost(bool, tag = "1")]
    pub is_save: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerRequest {
    #[prost(int32, tag = "1")]
    pub user_id: i32,
    #[prost(int32, tag = "2")]
    pub paper_id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuestionRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuestionReply {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub answer: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerListRequest {
    #[prost(int32, tag = "1")]
    pub paper_id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerInfo {
    #[prost(int32, tag = "1")]
    pub user_id: i32,
    #[prost(int32, tag = "2")]
    pub paper_id: i32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerListReply {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<AnswerInfo>,
}
/// 定义枚举类型 QuestionType
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuestionType {
    Choice = 0,
    Selection = 1,
    Interlocution = 2,
    Judge = 3,
}
impl QuestionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QuestionType::Choice => "CHOICE",
            QuestionType::Selection => "SELECTION",
            QuestionType::Interlocution => "INTERLOCUTION",
            QuestionType::Judge => "JUDGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHOICE" => Some(Self::Choice),
            "SELECTION" => Some(Self::Selection),
            "INTERLOCUTION" => Some(Self::Interlocution),
            "JUDGE" => Some(Self::Judge),
            _ => None,
        }
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
        pub async fn get_paper_by_id(
            &mut self,
            request: Request<super::PaperRequest>,
        ) -> Result<Response<super::Paper>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::PaperRequest,
                super::Paper,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/greeter.Greeter/getPaperById",
            );
            self.inner.unary(request, codec, path).await
        }
        pub async fn get_paper_list_by_id(
            &mut self,
            request: Request<super::PaperRequest>,
        ) -> Result<Response<super::PaperInfoList>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::PaperRequest,
                super::PaperInfoList,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/greeter.Greeter/getPaperListById",
            );
            self.inner.unary(request, codec, path).await
        }
        pub async fn set_answer_by_id(
            &mut self,
            request: Request<super::AnswerPaper>,
        ) -> Result<Response<super::AnswerReply>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::AnswerPaper,
                super::AnswerReply,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/greeter.Greeter/set_answer_by_id",
            );
            self.inner.unary(request, codec, path).await
        }
        pub async fn get_answer_by_id(
            &mut self,
            request: Request<super::AnswerRequest>,
        ) -> Result<Response<super::AnswerPaper>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::AnswerRequest,
                super::AnswerPaper,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/greeter.Greeter/get_answer_by_id",
            );
            self.inner.unary(request, codec, path).await
        }
        pub async fn get_answer_by_question_id(
            &mut self,
            request: Request<super::QuestionRequest>,
        ) -> Result<Response<super::QuestionReply>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::QuestionRequest,
                super::QuestionReply,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/greeter.Greeter/get_answer_by_question_id",
            );
            self.inner.unary(request, codec, path).await
        }
        pub async fn get_answer_list_by_paper_id(
            &mut self,
            request: Request<super::AnswerListRequest>,
        ) -> Result<Response<super::AnswerListReply>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::AnswerListRequest,
                super::AnswerListReply,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/greeter.Greeter/get_answer_list_by_paper_id",
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
        async fn get_paper_by_id(
            &self,
            request: Request<super::PaperRequest>,
        ) -> Result<Response<super::Paper>, dubbo::status::Status>;
        async fn get_paper_list_by_id(
            &self,
            request: Request<super::PaperRequest>,
        ) -> Result<Response<super::PaperInfoList>, dubbo::status::Status>;
        async fn set_answer_by_id(
            &self,
            request: Request<super::AnswerPaper>,
        ) -> Result<Response<super::AnswerReply>, dubbo::status::Status>;
        async fn get_answer_by_id(
            &self,
            request: Request<super::AnswerRequest>,
        ) -> Result<Response<super::AnswerPaper>, dubbo::status::Status>;
        async fn get_answer_by_question_id(
            &self,
            request: Request<super::QuestionRequest>,
        ) -> Result<Response<super::QuestionReply>, dubbo::status::Status>;
        async fn get_answer_list_by_paper_id(
            &self,
            request: Request<super::AnswerListRequest>,
        ) -> Result<Response<super::AnswerListReply>, dubbo::status::Status>;
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
                "/greeter.Greeter/getPaperById" => {
                    #[allow(non_camel_case_types)]
                    struct getPaperByIdServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::PaperRequest>
                    for getPaperByIdServer<T> {
                        type Response = super::Paper;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::PaperRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move {
                                inner.get_paper_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::Paper,
                                super::PaperRequest,
                            >::default(),
                        );
                        let res = server.unary(getPaperByIdServer { inner }, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/greeter.Greeter/getPaperListById" => {
                    #[allow(non_camel_case_types)]
                    struct getPaperListByIdServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::PaperRequest>
                    for getPaperListByIdServer<T> {
                        type Response = super::PaperInfoList;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::PaperRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move {
                                inner.get_paper_list_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::PaperInfoList,
                                super::PaperRequest,
                            >::default(),
                        );
                        let res = server
                            .unary(getPaperListByIdServer { inner }, req)
                            .await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/greeter.Greeter/set_answer_by_id" => {
                    #[allow(non_camel_case_types)]
                    struct set_answer_by_idServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::AnswerPaper>
                    for set_answer_by_idServer<T> {
                        type Response = super::AnswerReply;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::AnswerPaper>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move {
                                inner.set_answer_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::AnswerReply,
                                super::AnswerPaper,
                            >::default(),
                        );
                        let res = server
                            .unary(set_answer_by_idServer { inner }, req)
                            .await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/greeter.Greeter/get_answer_by_id" => {
                    #[allow(non_camel_case_types)]
                    struct get_answer_by_idServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::AnswerRequest>
                    for get_answer_by_idServer<T> {
                        type Response = super::AnswerPaper;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::AnswerRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move {
                                inner.get_answer_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::AnswerPaper,
                                super::AnswerRequest,
                            >::default(),
                        );
                        let res = server
                            .unary(get_answer_by_idServer { inner }, req)
                            .await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/greeter.Greeter/get_answer_by_question_id" => {
                    #[allow(non_camel_case_types)]
                    struct get_answer_by_question_idServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::QuestionRequest>
                    for get_answer_by_question_idServer<T> {
                        type Response = super::QuestionReply;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::QuestionRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move {
                                inner.get_answer_by_question_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::QuestionReply,
                                super::QuestionRequest,
                            >::default(),
                        );
                        let res = server
                            .unary(
                                get_answer_by_question_idServer {
                                    inner,
                                },
                                req,
                            )
                            .await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/greeter.Greeter/get_answer_list_by_paper_id" => {
                    #[allow(non_camel_case_types)]
                    struct get_answer_list_by_paper_idServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::AnswerListRequest>
                    for get_answer_list_by_paper_idServer<T> {
                        type Response = super::AnswerListReply;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::AnswerListRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move {
                                inner.get_answer_list_by_paper_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::AnswerListReply,
                                super::AnswerListRequest,
                            >::default(),
                        );
                        let res = server
                            .unary(
                                get_answer_list_by_paper_idServer {
                                    inner,
                                },
                                req,
                            )
                            .await;
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
