#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FieldLevelEncryptionConfigQueryArgProfileConfig {
    /// Flag to set if you want a request to be forwarded to the origin even if the profile specified by the field-level encryption query argument, fle-profile, is unknown.
    #[builder(into)]
    #[serde(rename = "forwardWhenQueryArgProfileIsUnknown")]
    pub r#forward_when_query_arg_profile_is_unknown: bool,
    /// Object that contains an attribute `items` that contains the list ofrofiles specified for query argument-profile mapping for field-level encryption. see Query Arg Profile.
    #[builder(into)]
    #[serde(rename = "queryArgProfiles")]
    pub r#query_arg_profiles: Option<Box<super::super::types::cloudfront::FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfiles>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FieldLevelEncryptionConfigQueryArgProfileConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("forward_when_query_arg_profile_is_unknown".to_string(), self.r#forward_when_query_arg_profile_is_unknown.to_pulumi_value().await);
            map.insert("query_arg_profiles".to_string(), self.r#query_arg_profiles.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FieldLevelEncryptionConfigQueryArgProfileConfig {
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
                    r#forward_when_query_arg_profile_is_unknown: {
                        let field_value = match fields_map.get("forward_when_query_arg_profile_is_unknown") {
                            Some(value) => value,
                            None => bail!("Missing field 'forward_when_query_arg_profile_is_unknown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#query_arg_profiles: {
                        let field_value = match fields_map.get("query_arg_profiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_arg_profiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cloudfront::FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfiles>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
