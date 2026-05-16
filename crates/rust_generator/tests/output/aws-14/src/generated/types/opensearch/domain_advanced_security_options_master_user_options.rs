#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainAdvancedSecurityOptionsMasterUserOptions {
    /// ARN for the main user. Only specify if `internal_user_database_enabled` is not set or set to `false`.
    #[builder(into)]
    #[serde(rename = "masterUserArn")]
    pub r#master_user_arn: Option<String>,
    /// Main user's username, which is stored in the Amazon OpenSearch Service domain's internal database. Only specify if `internal_user_database_enabled` is set to `true`.
    #[builder(into)]
    #[serde(rename = "masterUserName")]
    pub r#master_user_name: Option<String>,
    /// Main user's password, which is stored in the Amazon OpenSearch Service domain's internal database. Only specify if `internal_user_database_enabled` is set to `true`.
    #[builder(into)]
    #[serde(rename = "masterUserPassword")]
    pub r#master_user_password: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainAdvancedSecurityOptionsMasterUserOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("master_user_arn".to_string(), self.r#master_user_arn.to_pulumi_value().await);
            map.insert("master_user_name".to_string(), self.r#master_user_name.to_pulumi_value().await);
            map.insert("master_user_password".to_string(), self.r#master_user_password.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainAdvancedSecurityOptionsMasterUserOptions {
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
                    r#master_user_arn: {
                        let field_value = match fields_map.get("master_user_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_user_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#master_user_name: {
                        let field_value = match fields_map.get("master_user_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_user_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#master_user_password: {
                        let field_value = match fields_map.get("master_user_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_user_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
