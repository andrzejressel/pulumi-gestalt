#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainSettings {
    /// A collection of settings that configure the domain’s Docker interaction. see `docker_settings` Block below.
    #[builder(into)]
    #[serde(rename = "dockerSettings")]
    pub r#docker_settings: Option<Box<super::super::types::sagemaker::DomainDomainSettingsDockerSettings>>,
    /// The configuration for attaching a SageMaker user profile name to the execution role as a sts:SourceIdentity key [AWS Docs](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html). Valid values are `USER_PROFILE_NAME` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "executionRoleIdentityConfig")]
    pub r#execution_role_identity_config: Option<String>,
    /// A collection of settings that configure the RStudioServerPro Domain-level app. see `r_studio_server_pro_domain_settings` Block below.
    #[builder(into)]
    #[serde(rename = "rStudioServerProDomainSettings")]
    pub r#r_studio_server_pro_domain_settings: Option<Box<super::super::types::sagemaker::DomainDomainSettingsRStudioServerProDomainSettings>>,
    /// The security groups for the Amazon Virtual Private Cloud that the Domain uses for communication between Domain-level apps and user apps.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDomainSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "docker_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#docker_settings,
                )
                .await,
            );
            map.insert(
                "execution_role_identity_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execution_role_identity_config,
                )
                .await,
            );
            map.insert(
                "r_studio_server_pro_domain_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#r_studio_server_pro_domain_settings,
                )
                .await,
            );
            map.insert(
                "security_group_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_group_ids,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDomainSettings {
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
                    r#docker_settings: {
                        let field_value = match fields_map.get("docker_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_role_identity_config: {
                        let field_value = match fields_map.get("execution_role_identity_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_role_identity_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#r_studio_server_pro_domain_settings: {
                        let field_value = match fields_map.get("r_studio_server_pro_domain_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'r_studio_server_pro_domain_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_ids: {
                        let field_value = match fields_map.get("security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
