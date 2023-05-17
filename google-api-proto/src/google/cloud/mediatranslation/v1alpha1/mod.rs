/// Provides information to the speech translation that specifies how to process
/// the request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateSpeechConfig {
    /// Required. Encoding of audio data.
    /// Supported formats:
    ///
    /// - `linear16`
    ///
    ///    Uncompressed 16-bit signed little-endian samples (Linear PCM).
    ///
    /// - `flac`
    ///
    ///    `flac` (Free Lossless Audio Codec) is the recommended encoding
    ///    because it is lossless--therefore recognition is not compromised--and
    ///    requires only about half the bandwidth of `linear16`.
    ///
    /// - `mulaw`
    ///
    ///    8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    ///
    /// - `amr`
    ///
    ///    Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
    ///
    /// - `amr-wb`
    ///
    ///    Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
    ///
    /// - `ogg-opus`
    ///
    ///    Opus encoded audio frames in Ogg container
    ///    (\[OggOpus\](<https://wiki.xiph.org/OggOpus>)).
    ///    `sample_rate_hertz` must be one of 8000, 12000, 16000, 24000, or 48000.
    ///
    /// - `mp3`
    ///
    ///    MP3 audio. Support all standard MP3 bitrates (which range from 32-320
    ///    kbps). When using this encoding, `sample_rate_hertz` has to match the
    ///    sample rate of the file being used.
    ///
    ///
    #[prost(string, tag = "1")]
    pub audio_encoding: ::prost::alloc::string::String,
    /// Required. Source language code (BCP-47) of the input audio.
    #[prost(string, tag = "2")]
    pub source_language_code: ::prost::alloc::string::String,
    /// Required. Target language code (BCP-47) of the output.
    #[prost(string, tag = "3")]
    pub target_language_code: ::prost::alloc::string::String,
    /// Optional. A list of up to 3 additional language codes (BCP-47), listing possible
    /// alternative languages of the supplied audio. If alternative source
    /// languages are listed, speech translation result will translate in the most
    /// likely language detected including the main source_language_code. The
    /// translated result will include the language code of the language detected
    /// in the audio.
    /// Note:
    /// 1. If the provided alternative_source_language_code is not supported
    /// by current API version, we will skip that language code.
    /// 2. If user only provided one eligible alternative_source_language_codes,
    /// the translation will happen between source_language_code and
    /// alternative_source_language_codes. The target_language_code will be
    /// ignored. It will be useful in conversation mode.
    #[prost(string, repeated, tag = "6")]
    pub alternative_source_language_codes: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Optional. Sample rate in Hertz of the audio data. Valid values are:
    /// 8000-48000. 16000 is optimal. For best results, set the sampling rate of
    /// the audio source to 16000 Hz. If that's not possible, use the native sample
    /// rate of the audio source (instead of re-sampling).
    ///
    #[prost(int32, tag = "4")]
    pub sample_rate_hertz: i32,
    /// Optional.
    #[prost(string, tag = "5")]
    pub model: ::prost::alloc::string::String,
}
/// Config used for streaming translation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechConfig {
    /// Required. The common config for all the following audio contents.
    #[prost(message, optional, tag = "1")]
    pub audio_config: ::core::option::Option<TranslateSpeechConfig>,
    /// Optional. If `false` or omitted, the system performs
    /// continuous translation (continuing to wait for and process audio even if
    /// the user pauses speaking) until the client closes the input stream (gRPC
    /// API) or until the maximum time limit has been reached. May return multiple
    /// `StreamingTranslateSpeechResult`s with the `is_final` flag set to `true`.
    ///
    /// If `true`, the speech translator will detect a single spoken utterance.
    /// When it detects that the user has paused or stopped speaking, it will
    /// return an `END_OF_SINGLE_UTTERANCE` event and cease translation.
    /// When the client receives `END_OF_SINGLE_UTTERANCE` event, the client should
    /// stop sending the requests. However, clients should keep receiving remaining
    /// responses until the stream is terminated. To construct the complete
    /// sentence in a streaming way, one should override (if `is_final` of previous
    /// response is false), or append (if 'is_final' of previous response is true).
    #[prost(bool, tag = "2")]
    pub single_utterance: bool,
    /// Optional. Stability control for the media translation text. The value should be
    /// "LOW", "MEDIUM", "HIGH". It applies to text/text_and_audio translation
    /// only.
    /// For audio translation mode, we only support HIGH stability mode,
    /// low/medium stability mode will throw argument error.
    /// Default empty string will be treated as "HIGH" in audio translation mode;
    /// will be treated as "LOW" in other translation mode.
    /// Note that stability and speed would be trade off.
    /// 1. "LOW": In low mode, translation service will start to do translation
    /// right after getting recognition response. The speed will be faster.
    /// 2. "MEDIUM": In medium mode, translation service will
    /// check if the recognition response is stable enough or not, and only
    /// translate recognition response which is not likely to be changed later.
    /// 3. "HIGH": In high mode, translation service will wait for more stable
    /// recognition responses, and then start to do translation. Also, the
    /// following recognition responses cannot modify previous recognition
    /// responses. Thus it may impact quality in some situation. "HIGH" stability
    /// will generate "final" responses more frequently.
    ///
    #[prost(string, tag = "3")]
    pub stability: ::prost::alloc::string::String,
    /// Optional. Translation mode, the value should be "text", "audio", "text_and_audio".
    /// Default empty string will be treated as "text".
    /// 1. "text": The response will be text translation. Text translation has a
    /// field "is_final". Detailed definition can be found in
    /// `TextTranslationResult`.
    /// 2. "audio": The response will be audio translation. Audio translation does
    /// not have "is_final" field, which means each audio translation response is
    /// stable and will not be changed by later response.
    /// Translation mode "audio" can only be used with "high" stability mode,
    /// 3. "text_and_audio": The response will have a text translation, when
    /// "is_final" is true, we will also output its corresponding audio
    /// translation. When "is_final" is false, audio_translation field will be
    /// empty.
    #[prost(string, tag = "4")]
    pub translation_mode: ::prost::alloc::string::String,
    /// Optional. If disable_interim_results is true, we will only return "final" responses.
    /// Otherwise, we will return all the responses. Default value will be false.
    /// User can only set disable_interim_results to be true with "high" stability
    /// mode.
    #[prost(bool, tag = "5")]
    pub disable_interim_results: bool,
}
/// The top-level message sent by the client for the `StreamingTranslateSpeech`
/// method. Multiple `StreamingTranslateSpeechRequest` messages are sent. The
/// first message must contain a `streaming_config` message and must not contain
/// `audio_content` data. All subsequent messages must contain `audio_content`
/// data and must not contain a `streaming_config` message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechRequest {
    /// The streaming request, which is either a streaming config or content.
    #[prost(
        oneof = "streaming_translate_speech_request::StreamingRequest",
        tags = "1, 2"
    )]
    pub streaming_request: ::core::option::Option<
        streaming_translate_speech_request::StreamingRequest,
    >,
}
/// Nested message and enum types in `StreamingTranslateSpeechRequest`.
pub mod streaming_translate_speech_request {
    /// The streaming request, which is either a streaming config or content.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamingRequest {
        /// Provides information to the recognizer that specifies how to process the
        /// request. The first `StreamingTranslateSpeechRequest` message must contain
        /// a `streaming_config` message.
        #[prost(message, tag = "1")]
        StreamingConfig(super::StreamingTranslateSpeechConfig),
        /// The audio data to be translated. Sequential chunks of audio data are sent
        /// in sequential `StreamingTranslateSpeechRequest` messages. The first
        /// `StreamingTranslateSpeechRequest` message must not contain
        /// `audio_content` data and all subsequent `StreamingTranslateSpeechRequest`
        /// messages must contain `audio_content` data. The audio bytes must be
        /// encoded as specified in `StreamingTranslateSpeechConfig`. Note: as with
        /// all bytes fields, protobuffers use a pure binary representation (not
        /// base64).
        #[prost(bytes, tag = "2")]
        AudioContent(::prost::bytes::Bytes),
    }
}
/// A streaming speech translation result corresponding to a portion of the audio
/// that is currently being processed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechResult {
    /// Text translation result.
    #[prost(message, optional, tag = "1")]
    pub text_translation_result: ::core::option::Option<
        streaming_translate_speech_result::TextTranslationResult,
    >,
    /// Audio translation result.
    #[prost(message, optional, tag = "2")]
    pub audio_translation_result: ::core::option::Option<
        streaming_translate_speech_result::AudioTranslationResult,
    >,
    /// Output only. The debug only recognition result in original language. This field is debug
    /// only and will be set to empty string if not available.
    /// This is implementation detail and will not be backward compatible.
    #[prost(string, tag = "3")]
    pub recognition_result: ::prost::alloc::string::String,
    /// Output only.
    #[prost(string, tag = "4")]
    pub detected_source_language_code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StreamingTranslateSpeechResult`.
pub mod streaming_translate_speech_result {
    /// Text translation result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextTranslationResult {
        /// Output only. The translated sentence.
        #[prost(string, tag = "1")]
        pub translation: ::prost::alloc::string::String,
        /// Output only. If `false`, this `StreamingTranslateSpeechResult` represents
        /// an interim result that may change. If `true`, this is the final time the
        /// translation service will return this particular
        /// `StreamingTranslateSpeechResult`, the streaming translator will not
        /// return any further hypotheses for this portion of the transcript and
        /// corresponding audio.
        #[prost(bool, tag = "2")]
        pub is_final: bool,
    }
    /// Audio translation result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudioTranslationResult {
        /// Output only. The translated audio.
        #[prost(bytes = "bytes", tag = "1")]
        pub audio_translation: ::prost::bytes::Bytes,
    }
}
/// A streaming speech translation response corresponding to a portion of
/// the audio currently processed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingTranslateSpeechResponse {
    /// Output only. If set, returns a \[google.rpc.Status][google.rpc.Status\] message that
    /// specifies the error for the operation.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. The translation result that is currently being processed (For text
    /// translation, `is_final` could be `true` or `false`.
    /// For audio translation, we do not have is_final field, which means each
    /// audio response is stable and will not get changed later. For
    /// text_and_audio, we still have `is_final` field in text translation, but we
    /// only output corresponsding audio when `is_final` is true.).
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<StreamingTranslateSpeechResult>,
    /// Output only. Indicates the type of speech event.
    #[prost(
        enumeration = "streaming_translate_speech_response::SpeechEventType",
        tag = "3"
    )]
    pub speech_event_type: i32,
}
/// Nested message and enum types in `StreamingTranslateSpeechResponse`.
pub mod streaming_translate_speech_response {
    /// Indicates the type of speech event.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SpeechEventType {
        /// No speech event specified.
        Unspecified = 0,
        /// This event indicates that the server has detected the end of the user's
        /// speech utterance and expects no additional speech. Therefore, the server
        /// will not process additional audio (although it may subsequently return
        /// additional results). When the client receives `END_OF_SINGLE_UTTERANCE`
        /// event, the client should stop sending the requests. However, clients
        /// should keep receiving remaining responses until the stream is terminated.
        /// To construct the complete sentence in a streaming way, one should
        /// override (if `is_final` of previous response is `false`), or append (if
        /// `is_final` of previous response is `true`). This event is only sent if
        /// `single_utterance` was set to `true`, and is not used otherwise.
        EndOfSingleUtterance = 1,
    }
    impl SpeechEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SpeechEventType::Unspecified => "SPEECH_EVENT_TYPE_UNSPECIFIED",
                SpeechEventType::EndOfSingleUtterance => "END_OF_SINGLE_UTTERANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SPEECH_EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "END_OF_SINGLE_UTTERANCE" => Some(Self::EndOfSingleUtterance),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod speech_translation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides translation from/to media types.
    #[derive(Debug, Clone)]
    pub struct SpeechTranslationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SpeechTranslationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SpeechTranslationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SpeechTranslationServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Performs bidirectional streaming speech translation: receive results while
        /// sending audio. This method is only available via the gRPC API (not REST).
        pub async fn streaming_translate_speech(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::StreamingTranslateSpeechRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamingTranslateSpeechResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.mediatranslation.v1alpha1.SpeechTranslationService/StreamingTranslateSpeech",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.mediatranslation.v1alpha1.SpeechTranslationService",
                        "StreamingTranslateSpeech",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
