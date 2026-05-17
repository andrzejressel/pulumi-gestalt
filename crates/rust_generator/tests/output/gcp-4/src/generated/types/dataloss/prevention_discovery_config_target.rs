#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTarget {
    /// BigQuery target for Discovery. The first target to match a table will be the one applied.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigQueryTarget")]
    pub r#big_query_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTarget>>,
    /// Cloud SQL target for Discovery. The first target to match a table will be the one applied.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudSqlTarget")]
    pub r#cloud_sql_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTarget>>,
    /// Cloud Storage target for Discovery. The first target to match a bucket will be the one applied.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorageTarget")]
    pub r#cloud_storage_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTarget>>,
    /// Discovery target that looks for credentials and secrets stored in cloud resource metadata and reports them as vulnerabilities to Security Command Center. Only one target of this type is allowed.
    #[builder(into)]
    #[serde(rename = "secretsTarget")]
    pub r#secrets_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetSecretsTarget>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTarget {
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
                "big_query_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#big_query_target,
                )
                .await,
            );
            map.insert(
                "cloud_sql_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_sql_target,
                )
                .await,
            );
            map.insert(
                "cloud_storage_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_storage_target,
                )
                .await,
            );
            map.insert(
                "secrets_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets_target,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigTarget {
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
                    r#big_query_target: {
                        let field_value = match fields_map.get("big_query_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'big_query_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_sql_target: {
                        let field_value = match fields_map.get("cloud_sql_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_sql_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_storage_target: {
                        let field_value = match fields_map.get("cloud_storage_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_storage_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets_target: {
                        let field_value = match fields_map.get("secrets_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
