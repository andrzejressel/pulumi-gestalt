#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInferenceProfilesInferenceProfileSummary {
    /// The time at which the inference profile was created.
    #[builder(into)]
    pub r#created_at: String,
    /// The description of the inference profile.
    #[builder(into)]
    pub r#description: String,
    /// The Amazon Resource Name (ARN) of the inference profile.
    #[builder(into)]
    pub r#inference_profile_arn: String,
    /// The unique identifier of the inference profile.
    #[builder(into)]
    pub r#inference_profile_id: String,
    /// The name of the inference profile.
    #[builder(into)]
    pub r#inference_profile_name: String,
    /// A list of information about each model in the inference profile. See `models`.
    #[builder(into)]
    pub r#models: Vec<super::super::types::bedrock::GetInferenceProfilesInferenceProfileSummaryModel>,
    /// The status of the inference profile. `ACTIVE` means that the inference profile is available to use.
    #[builder(into)]
    pub r#status: String,
    /// The type of the inference profile. `SYSTEM_DEFINED` means that the inference profile is defined by Amazon Bedrock.
    #[builder(into)]
    pub r#type_: String,
    /// The time at which the inference profile was last updated.
    #[builder(into)]
    pub r#updated_at: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInferenceProfilesInferenceProfileSummary {
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
                    "createdAt",
                    &self.r#created_at,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "inferenceProfileArn",
                    &self.r#inference_profile_arn,
                ),
                to_pulumi_object_field(
                    "inferenceProfileId",
                    &self.r#inference_profile_id,
                ),
                to_pulumi_object_field(
                    "inferenceProfileName",
                    &self.r#inference_profile_name,
                ),
                to_pulumi_object_field(
                    "models",
                    &self.r#models,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "updatedAt",
                    &self.r#updated_at,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInferenceProfilesInferenceProfileSummary {
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
                    r#created_at: {
                        let field_value = match fields_map.get("createdAt") {
                            Some(value) => value,
                            None => bail!("Missing field 'createdAt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_profile_arn: {
                        let field_value = match fields_map.get("inferenceProfileArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'inferenceProfileArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_profile_id: {
                        let field_value = match fields_map.get("inferenceProfileId") {
                            Some(value) => value,
                            None => bail!("Missing field 'inferenceProfileId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_profile_name: {
                        let field_value = match fields_map.get("inferenceProfileName") {
                            Some(value) => value,
                            None => bail!("Missing field 'inferenceProfileName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#models: {
                        let field_value = match fields_map.get("models") {
                            Some(value) => value,
                            None => bail!("Missing field 'models' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated_at: {
                        let field_value = match fields_map.get("updatedAt") {
                            Some(value) => value,
                            None => bail!("Missing field 'updatedAt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
