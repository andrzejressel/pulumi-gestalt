#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInferenceProfilesInferenceProfileSummary {
    /// The time at which the inference profile was created.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: String,
    /// The description of the inference profile.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The Amazon Resource Name (ARN) of the inference profile.
    #[builder(into)]
    #[serde(rename = "inferenceProfileArn")]
    pub r#inference_profile_arn: String,
    /// The unique identifier of the inference profile.
    #[builder(into)]
    #[serde(rename = "inferenceProfileId")]
    pub r#inference_profile_id: String,
    /// The name of the inference profile.
    #[builder(into)]
    #[serde(rename = "inferenceProfileName")]
    pub r#inference_profile_name: String,
    /// A list of information about each model in the inference profile. See `models`.
    #[builder(into)]
    #[serde(rename = "models")]
    pub r#models: Vec<super::super::types::bedrock::GetInferenceProfilesInferenceProfileSummaryModel>,
    /// The status of the inference profile. `ACTIVE` means that the inference profile is available to use.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// The type of the inference profile. `SYSTEM_DEFINED` means that the inference profile is defined by Amazon Bedrock.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The time at which the inference profile was last updated.
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInferenceProfilesInferenceProfileSummary {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("created_at".to_string(), self.r#created_at.to_pulumi_value().await);
            map.insert("description".to_string(), self.r#description.to_pulumi_value().await);
            map.insert("inference_profile_arn".to_string(), self.r#inference_profile_arn.to_pulumi_value().await);
            map.insert("inference_profile_id".to_string(), self.r#inference_profile_id.to_pulumi_value().await);
            map.insert("inference_profile_name".to_string(), self.r#inference_profile_name.to_pulumi_value().await);
            map.insert("models".to_string(), self.r#models.to_pulumi_value().await);
            map.insert("status".to_string(), self.r#status.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);
            map.insert("updated_at".to_string(), self.r#updated_at.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInferenceProfilesInferenceProfileSummary {
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
                    r#created_at: {
                        let field_value = match fields_map.get("created_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#inference_profile_arn: {
                        let field_value = match fields_map.get("inference_profile_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_profile_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#inference_profile_id: {
                        let field_value = match fields_map.get("inference_profile_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_profile_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#inference_profile_name: {
                        let field_value = match fields_map.get("inference_profile_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_profile_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#models: {
                        let field_value = match fields_map.get("models") {
                            Some(value) => value,
                            None => bail!("Missing field 'models' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::bedrock::GetInferenceProfilesInferenceProfileSummaryModel> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#updated_at: {
                        let field_value = match fields_map.get("updated_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
