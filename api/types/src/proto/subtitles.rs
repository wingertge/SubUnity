#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRequest {
    #[prost(string, tag = "1")]
    pub video_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(enumeration = "download_request::Format", tag = "3")]
    pub format: i32,
}
pub mod download_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Format {
        Srt = 0,
    }
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    #[prost(bytes, tag = "1")]
    pub content: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleId {
    #[prost(string, tag = "1")]
    pub video_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitles {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<subtitles::Entry>,
    #[prost(string, tag = "2")]
    pub video_id: std::string::String,
    #[prost(string, tag = "3")]
    pub language: std::string::String,
    #[prost(string, tag = "4")]
    pub video_title: std::string::String,
    #[prost(string, tag = "5")]
    pub uploader_id: std::string::String,
    #[prost(string, tag = "6")]
    pub uploader_name: std::string::String,
}
pub mod subtitles {
    #[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Entry {
        #[prost(float, tag = "1")]
        pub start_seconds: f32,
        #[prost(float, tag = "2")]
        pub end_seconds: f32,
        #[prost(string, tag = "3")]
        pub text: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSubtitleResponse {}
#[doc = r" Generated client implementations."]
pub mod video_subs_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct VideoSubsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VideoSubsClient<tonic::transport::Channel> {
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
    impl<T> VideoSubsClient<T>
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
        pub async fn set_subtitles(
            &mut self,
            request: impl tonic::IntoRequest<super::Subtitles>,
        ) -> Result<tonic::Response<super::SetSubtitleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/subtitles.VideoSubs/SetSubtitles");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_subtitles(
            &mut self,
            request: impl tonic::IntoRequest<super::SubtitleId>,
        ) -> Result<tonic::Response<super::Subtitles>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/subtitles.VideoSubs/GetSubtitles");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn download_subtitles(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Chunk>>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/subtitles.VideoSubs/DownloadSubtitles");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for VideoSubsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VideoSubsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VideoSubsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod video_subs_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VideoSubsServer."]
    #[async_trait]
    pub trait VideoSubs: Send + Sync + 'static {
        async fn set_subtitles(
            &self,
            request: tonic::Request<super::Subtitles>,
        ) -> Result<tonic::Response<super::SetSubtitleResponse>, tonic::Status>;
        async fn get_subtitles(
            &self,
            request: tonic::Request<super::SubtitleId>,
        ) -> Result<tonic::Response<super::Subtitles>, tonic::Status>;
        #[doc = "Server streaming response type for the DownloadSubtitles method."]
        type DownloadSubtitlesStream: Stream<Item = Result<super::Chunk, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn download_subtitles(
            &self,
            request: tonic::Request<super::DownloadRequest>,
        ) -> Result<tonic::Response<Self::DownloadSubtitlesStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct VideoSubsServer<T: VideoSubs> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: VideoSubs> VideoSubsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VideoSubsServer<T>
    where
        T: VideoSubs,
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
                "/subtitles.VideoSubs/SetSubtitles" => {
                    #[allow(non_camel_case_types)]
                    struct SetSubtitlesSvc<T: VideoSubs>(pub Arc<T>);
                    impl<T: VideoSubs> tonic::server::UnaryService<super::Subtitles> for SetSubtitlesSvc<T> {
                        type Response = super::SetSubtitleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Subtitles>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_subtitles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetSubtitlesSvc(inner);
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
                "/subtitles.VideoSubs/GetSubtitles" => {
                    #[allow(non_camel_case_types)]
                    struct GetSubtitlesSvc<T: VideoSubs>(pub Arc<T>);
                    impl<T: VideoSubs> tonic::server::UnaryService<super::SubtitleId> for GetSubtitlesSvc<T> {
                        type Response = super::Subtitles;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubtitleId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_subtitles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSubtitlesSvc(inner);
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
                "/subtitles.VideoSubs/DownloadSubtitles" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadSubtitlesSvc<T: VideoSubs>(pub Arc<T>);
                    impl<T: VideoSubs> tonic::server::ServerStreamingService<super::DownloadRequest>
                        for DownloadSubtitlesSvc<T>
                    {
                        type Response = super::Chunk;
                        type ResponseStream = T::DownloadSubtitlesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).download_subtitles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = DownloadSubtitlesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: VideoSubs> Clone for VideoSubsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: VideoSubs> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VideoSubs> tonic::transport::NamedService for VideoSubsServer<T> {
        const NAME: &'static str = "subtitles.VideoSubs";
    }
}
