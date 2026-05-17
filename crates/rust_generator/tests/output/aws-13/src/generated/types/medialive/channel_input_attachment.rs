#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachment {
    /// User-specified settings for defining what the conditions are for declaring the input unhealthy and failing over to a different input. See Automatic Input Failover Settings for more details.
    #[builder(into)]
    #[serde(rename = "automaticInputFailoverSettings")]
    pub r#automatic_input_failover_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettings>>,
    /// User-specified name for the attachment.
    #[builder(into)]
    #[serde(rename = "inputAttachmentName")]
    pub r#input_attachment_name: String,
    /// The ID of the input.
    #[builder(into)]
    #[serde(rename = "inputId")]
    pub r#input_id: String,
    /// Settings of an input. See Input Settings for more details.
    #[builder(into)]
    #[serde(rename = "inputSettings")]
    pub r#input_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "automatic_input_failover_settings",
                    &self.r#automatic_input_failover_settings,
                ),
                to_pulumi_object_field(
                    "input_attachment_name",
                    &self.r#input_attachment_name,
                ),
                to_pulumi_object_field(
                    "input_id",
                    &self.r#input_id,
                ),
                to_pulumi_object_field(
                    "input_settings",
                    &self.r#input_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachment {
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
                    r#automatic_input_failover_settings: {
                        let field_value = match fields_map.get("automatic_input_failover_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic_input_failover_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_attachment_name: {
                        let field_value = match fields_map.get("input_attachment_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_attachment_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_id: {
                        let field_value = match fields_map.get("input_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_settings: {
                        let field_value = match fields_map.get("input_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
