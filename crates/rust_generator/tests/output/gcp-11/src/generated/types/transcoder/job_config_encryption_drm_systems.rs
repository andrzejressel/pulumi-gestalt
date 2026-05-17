#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobConfigEncryptionDrmSystems {
    /// Clearkey configuration.
    #[builder(into)]
    #[serde(rename = "clearkey")]
    pub r#clearkey: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsClearkey>>,
    /// Fairplay configuration.
    #[builder(into)]
    #[serde(rename = "fairplay")]
    pub r#fairplay: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsFairplay>>,
    /// Playready configuration.
    #[builder(into)]
    #[serde(rename = "playready")]
    pub r#playready: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsPlayready>>,
    /// Widevine configuration.
    #[builder(into)]
    #[serde(rename = "widevine")]
    pub r#widevine: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsWidevine>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobConfigEncryptionDrmSystems {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "clearkey",
                    &self.r#clearkey,
                ),
                to_pulumi_object_field(
                    "fairplay",
                    &self.r#fairplay,
                ),
                to_pulumi_object_field(
                    "playready",
                    &self.r#playready,
                ),
                to_pulumi_object_field(
                    "widevine",
                    &self.r#widevine,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobConfigEncryptionDrmSystems {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#clearkey: {
                        let field_value = match fields_map.get("clearkey") {
                            Some(value) => value,
                            None => bail!("Missing field 'clearkey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fairplay: {
                        let field_value = match fields_map.get("fairplay") {
                            Some(value) => value,
                            None => bail!("Missing field 'fairplay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#playready: {
                        let field_value = match fields_map.get("playready") {
                            Some(value) => value,
                            None => bail!("Missing field 'playready' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#widevine: {
                        let field_value = match fields_map.get("widevine") {
                            Some(value) => value,
                            None => bail!("Missing field 'widevine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
