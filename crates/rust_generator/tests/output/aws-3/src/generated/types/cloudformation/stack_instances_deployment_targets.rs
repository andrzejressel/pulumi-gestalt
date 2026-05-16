#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StackInstancesDeploymentTargets {
    /// Limit deployment targets to individual accounts or include additional accounts with provided OUs. Valid values: `INTERSECTION`, `DIFFERENCE`, `UNION`, `NONE`.
    #[builder(into)]
    #[serde(rename = "accountFilterType")]
    pub r#account_filter_type: Option<String>,
    /// List of accounts to deploy stack set updates.
    #[builder(into)]
    #[serde(rename = "accounts")]
    pub r#accounts: Option<Vec<String>>,
    /// S3 URL of the file containing the list of accounts.
    #[builder(into)]
    #[serde(rename = "accountsUrl")]
    pub r#accounts_url: Option<String>,
    /// Organization root ID or organizational unit (OU) IDs to which stack sets deploy.
    #[builder(into)]
    #[serde(rename = "organizationalUnitIds")]
    pub r#organizational_unit_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StackInstancesDeploymentTargets {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("account_filter_type".to_string(), self.r#account_filter_type.to_pulumi_value().await);
            map.insert("accounts".to_string(), self.r#accounts.to_pulumi_value().await);
            map.insert("accounts_url".to_string(), self.r#accounts_url.to_pulumi_value().await);
            map.insert("organizational_unit_ids".to_string(), self.r#organizational_unit_ids.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StackInstancesDeploymentTargets {
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
                    r#account_filter_type: {
                        let field_value = match fields_map.get("account_filter_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_filter_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#accounts: {
                        let field_value = match fields_map.get("accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#accounts_url: {
                        let field_value = match fields_map.get("accounts_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'accounts_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit_ids: {
                        let field_value = match fields_map.get("organizational_unit_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
