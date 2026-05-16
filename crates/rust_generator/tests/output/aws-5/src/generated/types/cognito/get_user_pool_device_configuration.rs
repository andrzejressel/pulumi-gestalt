#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolDeviceConfiguration {
    /// - Whether a challenge is required on new devices.
    #[builder(into)]
    #[serde(rename = "challengeRequiredOnNewDevice")]
    pub r#challenge_required_on_new_device: bool,
    /// - Whether devices are only remembered if the user prompts it.
    #[builder(into)]
    #[serde(rename = "deviceOnlyRememberedOnUserPrompt")]
    pub r#device_only_remembered_on_user_prompt: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPoolDeviceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("challenge_required_on_new_device".to_string(), self.r#challenge_required_on_new_device.to_pulumi_value().await);
            map.insert("device_only_remembered_on_user_prompt".to_string(), self.r#device_only_remembered_on_user_prompt.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserPoolDeviceConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#challenge_required_on_new_device: {
                        let field_value = match fields_map.get("challenge_required_on_new_device") {
                            Some(value) => value,
                            None => bail!("Missing field 'challenge_required_on_new_device' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#device_only_remembered_on_user_prompt: {
                        let field_value = match fields_map.get("device_only_remembered_on_user_prompt") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_only_remembered_on_user_prompt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
