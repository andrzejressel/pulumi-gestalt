#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePscConfig {
    /// List of VPCs that are allowed ingress into the Looker instance.
    #[builder(into)]
    #[serde(rename = "allowedVpcs")]
    pub r#allowed_vpcs: Option<Vec<String>>,
    /// (Output)
    /// URI of the Looker service attachment.
    #[builder(into)]
    #[serde(rename = "lookerServiceAttachmentUri")]
    pub r#looker_service_attachment_uri: Option<String>,
    /// List of egress service attachment configurations.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceAttachments")]
    pub r#service_attachments: Option<Vec<super::super::types::looker::InstancePscConfigServiceAttachment>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePscConfig {
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
                    "allowed_vpcs",
                    &self.r#allowed_vpcs,
                ),
                to_pulumi_object_field(
                    "looker_service_attachment_uri",
                    &self.r#looker_service_attachment_uri,
                ),
                to_pulumi_object_field(
                    "service_attachments",
                    &self.r#service_attachments,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstancePscConfig {
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
                    r#allowed_vpcs: {
                        let field_value = match fields_map.get("allowed_vpcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_vpcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#looker_service_attachment_uri: {
                        let field_value = match fields_map.get("looker_service_attachment_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'looker_service_attachment_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachments: {
                        let field_value = match fields_map.get("service_attachments") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_attachments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
