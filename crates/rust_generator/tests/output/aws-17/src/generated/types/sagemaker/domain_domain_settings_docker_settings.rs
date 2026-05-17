#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainSettingsDockerSettings {
    /// Indicates whether the domain can access Docker. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "enableDockerAccess")]
    pub r#enable_docker_access: Option<String>,
    /// The list of Amazon Web Services accounts that are trusted when the domain is created in VPC-only mode.
    #[builder(into)]
    #[serde(rename = "vpcOnlyTrustedAccounts")]
    pub r#vpc_only_trusted_accounts: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDomainSettingsDockerSettings {
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
                "enable_docker_access".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_docker_access,
                )
                .await,
            );
            map.insert(
                "vpc_only_trusted_accounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_only_trusted_accounts,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDomainSettingsDockerSettings {
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
                    r#enable_docker_access: {
                        let field_value = match fields_map.get("enable_docker_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_docker_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_only_trusted_accounts: {
                        let field_value = match fields_map.get("vpc_only_trusted_accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_only_trusted_accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
