#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LanguageModelInputDataConfig {
    /// IAM role with access to S3 bucket.
    #[builder(into)]
    #[serde(rename = "dataAccessRoleArn")]
    pub r#data_access_role_arn: String,
    /// S3 URI where training data is located.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: String,
    /// S3 URI where tuning data is located.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "tuningDataS3Uri")]
    pub r#tuning_data_s_3_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LanguageModelInputDataConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_access_role_arn".to_string(), self.r#data_access_role_arn.to_pulumi_value().await);
            map.insert("s_3_uri".to_string(), self.r#s_3_uri.to_pulumi_value().await);
            map.insert("tuning_data_s_3_uri".to_string(), self.r#tuning_data_s_3_uri.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LanguageModelInputDataConfig {
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
                    r#data_access_role_arn: {
                        let field_value = match fields_map.get("data_access_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_access_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_uri: {
                        let field_value = match fields_map.get("s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tuning_data_s_3_uri: {
                        let field_value = match fields_map.get("tuning_data_s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'tuning_data_s_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
