/// Provides an Elastic Transcoder preset resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   bar:
///     type: aws:elastictranscoder:Preset
///     properties:
///       container: mp4
///       description: Sample Preset
///       name: sample_preset
///       audio:
///         audioPackingMode: SingleTrack
///         bitRate: 96
///         channels: 2
///         codec: AAC
///         sampleRate: 44100
///       audioCodecOptions:
///         profile: AAC-LC
///       video:
///         bitRate: '1600'
///         codec: H.264
///         displayAspectRatio: 16:9
///         fixedGop: 'false'
///         frameRate: auto
///         maxFrameRate: '60'
///         keyframesMaxDist: 240
///         maxHeight: auto
///         maxWidth: auto
///         paddingPolicy: Pad
///         sizingPolicy: Fit
///       videoCodecOptions:
///         Profile: main
///         Level: '2.2'
///         MaxReferenceFrames: 3
///         InterlacedMode: Progressive
///         ColorSpaceConversionMode: None
///       videoWatermarks:
///         - id: Test
///           maxWidth: 20%
///           maxHeight: 20%
///           sizingPolicy: ShrinkToFit
///           horizontalAlign: Right
///           horizontalOffset: 10px
///           verticalAlign: Bottom
///           verticalOffset: 10px
///           opacity: '55.5'
///           target: Content
///       thumbnails:
///         format: png
///         interval: 120
///         maxWidth: auto
///         maxHeight: auto
///         paddingPolicy: Pad
///         sizingPolicy: Fit
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elastic Transcoder presets using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:elastictranscoder/preset:Preset basic_preset 1407981661351-cttk8b
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod preset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PresetArgs {
        /// Audio parameters object (documented below).
        #[builder(into, default)]
        pub audio: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetAudio>,
        >,
        /// Codec options for the audio parameters (documented below)
        #[builder(into, default)]
        pub audio_codec_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetAudioCodecOptions>,
        >,
        /// The container type for the output file. Valid values are `flac`, `flv`, `fmp4`, `gif`, `mp3`, `mp4`, `mpg`, `mxf`, `oga`, `ogg`, `ts`, and `webm`.
        #[builder(into)]
        pub container: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the preset (maximum 255 characters)
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the preset. (maximum 40 characters)
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Thumbnail parameters object (documented below)
        #[builder(into, default)]
        pub thumbnails: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetThumbnails>,
        >,
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Video parameters object (documented below)
        #[builder(into, default)]
        pub video: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetVideo>,
        >,
        /// Codec options for the video parameters
        #[builder(into, default)]
        pub video_codec_options: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Watermark parameters for the video parameters (documented below)
        #[builder(into, default)]
        pub video_watermarks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::elastictranscoder::PresetVideoWatermark>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PresetResult {
        /// Amazon Resource Name (ARN) of the Elastic Transcoder Preset.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Audio parameters object (documented below).
        pub audio: pulumi_gestalt_rust::Output<
            Option<super::super::types::elastictranscoder::PresetAudio>,
        >,
        /// Codec options for the audio parameters (documented below)
        pub audio_codec_options: pulumi_gestalt_rust::Output<
            super::super::types::elastictranscoder::PresetAudioCodecOptions,
        >,
        /// The container type for the output file. Valid values are `flac`, `flv`, `fmp4`, `gif`, `mp3`, `mp4`, `mpg`, `mxf`, `oga`, `ogg`, `ts`, and `webm`.
        pub container: pulumi_gestalt_rust::Output<String>,
        /// A description of the preset (maximum 255 characters)
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the preset. (maximum 40 characters)
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Thumbnail parameters object (documented below)
        pub thumbnails: pulumi_gestalt_rust::Output<
            Option<super::super::types::elastictranscoder::PresetThumbnails>,
        >,
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Video parameters object (documented below)
        pub video: pulumi_gestalt_rust::Output<
            Option<super::super::types::elastictranscoder::PresetVideo>,
        >,
        /// Codec options for the video parameters
        pub video_codec_options: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Watermark parameters for the video parameters (documented below)
        pub video_watermarks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::elastictranscoder::PresetVideoWatermark>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PresetArgs,
    ) -> PresetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audio_binding = args.audio.get_output(context);
        let audio_codec_options_binding = args.audio_codec_options.get_output(context);
        let container_binding = args.container.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let thumbnails_binding = args.thumbnails.get_output(context);
        let type__binding = args.type_.get_output(context);
        let video_binding = args.video.get_output(context);
        let video_codec_options_binding = args.video_codec_options.get_output(context);
        let video_watermarks_binding = args.video_watermarks.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elastictranscoder/preset:Preset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "audio".into(),
                    value: &audio_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "audioCodecOptions".into(),
                    value: &audio_codec_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "container".into(),
                    value: &container_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thumbnails".into(),
                    value: &thumbnails_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "video".into(),
                    value: &video_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "videoCodecOptions".into(),
                    value: &video_codec_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "videoWatermarks".into(),
                    value: &video_watermarks_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PresetResult {
            arn: o.get_field("arn"),
            audio: o.get_field("audio"),
            audio_codec_options: o.get_field("audioCodecOptions"),
            container: o.get_field("container"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            thumbnails: o.get_field("thumbnails"),
            type_: o.get_field("type"),
            video: o.get_field("video"),
            video_codec_options: o.get_field("videoCodecOptions"),
            video_watermarks: o.get_field("videoWatermarks"),
        }
    }
}
