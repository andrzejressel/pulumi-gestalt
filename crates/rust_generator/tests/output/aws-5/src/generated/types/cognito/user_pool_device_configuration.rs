#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolDeviceConfiguration {
    /// Whether a challenge is required on a new device. Only applicable to a new device.
    #[builder(into)]
    #[serde(rename = "challengeRequiredOnNewDevice")]
    pub r#challenge_required_on_new_device: Option<bool>,
    /// Whether a device is only remembered on user prompt. `false` equates to "Always" remember, `true` is "User Opt In," and not using a `device_configuration` block is "No."
    #[builder(into)]
    #[serde(rename = "deviceOnlyRememberedOnUserPrompt")]
    pub r#device_only_remembered_on_user_prompt: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPoolDeviceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "challenge_required_on_new_device",
                    &self.r#challenge_required_on_new_device,
                ),
                to_pulumi_object_field(
                    "device_only_remembered_on_user_prompt",
                    &self.r#device_only_remembered_on_user_prompt,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPoolDeviceConfiguration {
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
                    r#challenge_required_on_new_device: {
                        let field_value = match fields_map.get("challenge_required_on_new_device") {
                            Some(value) => value,
                            None => bail!("Missing field 'challenge_required_on_new_device' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_only_remembered_on_user_prompt: {
                        let field_value = match fields_map.get("device_only_remembered_on_user_prompt") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_only_remembered_on_user_prompt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
