#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StackSetInstanceDeploymentTargets {
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
    /// Organization root ID or organizational unit (OU) IDs to which StackSets deploys.
    #[builder(into)]
    #[serde(rename = "organizationalUnitIds")]
    pub r#organizational_unit_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StackSetInstanceDeploymentTargets {
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
                "account_filter_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_filter_type,
                )
                .await,
            );
            map.insert(
                "accounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accounts,
                )
                .await,
            );
            map.insert(
                "accounts_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accounts_url,
                )
                .await,
            );
            map.insert(
                "organizational_unit_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#organizational_unit_ids,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StackSetInstanceDeploymentTargets {
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
                    r#account_filter_type: {
                        let field_value = match fields_map.get("account_filter_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_filter_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accounts: {
                        let field_value = match fields_map.get("accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accounts_url: {
                        let field_value = match fields_map.get("accounts_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'accounts_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit_ids: {
                        let field_value = match fields_map.get("organizational_unit_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
