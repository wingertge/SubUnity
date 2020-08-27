#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SayRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SayResponse {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageUploadRequest {
    #[prost(string, tag = "1")]
    pub file: std::string::String,
    #[prost(uint32, tag = "2")]
    pub width: u32,
    #[prost(uint32, tag = "3")]
    pub height: u32,
    #[prost(uint32, tag = "4")]
    pub crop_size: u32,
    #[prost(uint32, tag = "5")]
    pub offset_x: u32,
    #[prost(uint32, tag = "6")]
    pub offset_y: u32,
    #[prost(bytes, tag = "7")]
    pub content: std::vec::Vec<u8>,
}
/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
///
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// The status code, which should be an enum value of [google.rpc.Code][google.rpc.Code].
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// [google.rpc.Status.details][google.rpc.Status.details] field, or localized by the client.
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod user_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct UserClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UserClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn send(
            &mut self,
            request: impl tonic::IntoRequest<super::SayRequest>,
        ) -> Result<tonic::Response<super::SayResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/user.User/Send");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_profile_picture(
            &mut self,
            request: impl tonic::IntoRequest<super::ImageUploadRequest>,
        ) -> Result<tonic::Response<super::Status>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/user.User/SetProfilePicture");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UserClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UserClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod user_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UserServer."]
    #[async_trait]
    pub trait User: Send + Sync + 'static {
        async fn send(
            &self,
            request: tonic::Request<super::SayRequest>,
        ) -> Result<tonic::Response<super::SayResponse>, tonic::Status>;
        async fn set_profile_picture(
            &self,
            request: tonic::Request<super::ImageUploadRequest>,
        ) -> Result<tonic::Response<super::Status>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UserServer<T: User> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: User> UserServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for UserServer<T>
    where
        T: User,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/user.User/Send" => {
                    #[allow(non_camel_case_types)]
                    struct SendSvc<T: User>(pub Arc<T>);
                    impl<T: User> tonic::server::UnaryService<super::SayRequest> for SendSvc<T> {
                        type Response = super::SayResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SayRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/user.User/SetProfilePicture" => {
                    #[allow(non_camel_case_types)]
                    struct SetProfilePictureSvc<T: User>(pub Arc<T>);
                    impl<T: User> tonic::server::UnaryService<super::ImageUploadRequest> for SetProfilePictureSvc<T> {
                        type Response = super::Status;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImageUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_profile_picture(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetProfilePictureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: User> Clone for UserServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: User> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: User> tonic::transport::NamedService for UserServer<T> {
        const NAME: &'static str = "user.User";
    }
}
