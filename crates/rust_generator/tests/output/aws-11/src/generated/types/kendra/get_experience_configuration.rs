#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetExperienceConfiguration {
    /// The identifiers of your data sources and FAQs. This is the content you want to use for your Amazon Kendra Experience. Documented below.
    #[builder(into)]
    #[serde(rename = "contentSourceConfigurations")]
    pub r#content_source_configurations: Vec<super::super::types::kendra::GetExperienceConfigurationContentSourceConfiguration>,
    /// The AWS SSO field name that contains the identifiers of your users, such as their emails. Documented below.
    #[builder(into)]
    #[serde(rename = "userIdentityConfigurations")]
    pub r#user_identity_configurations: Vec<super::super::types::kendra::GetExperienceConfigurationUserIdentityConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetExperienceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "content_source_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content_source_configurations,
                )
                .await,
            );
            map.insert(
                "user_identity_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_identity_configurations,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetExperienceConfiguration {
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
                    r#content_source_configurations: {
                        let field_value = match fields_map.get("content_source_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_source_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_identity_configurations: {
                        let field_value = match fields_map.get("user_identity_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_identity_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
