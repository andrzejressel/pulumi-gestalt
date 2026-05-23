#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTarget {
    /// BigQuery target for Discovery. The first target to match a table will be the one applied.
    /// Structure is documented below.
    #[builder(into)]
    pub r#big_query_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTarget>>,
    /// Cloud SQL target for Discovery. The first target to match a table will be the one applied.
    /// Structure is documented below.
    #[builder(into)]
    pub r#cloud_sql_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTarget>>,
    /// Cloud Storage target for Discovery. The first target to match a bucket will be the one applied.
    /// Structure is documented below.
    #[builder(into)]
    pub r#cloud_storage_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTarget>>,
    /// Discovery target that looks for credentials and secrets stored in cloud resource metadata and reports them as vulnerabilities to Security Command Center. Only one target of this type is allowed.
    #[builder(into)]
    pub r#secrets_target: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetSecretsTarget>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTarget {
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
                    "bigQueryTarget",
                    &self.r#big_query_target,
                ),
                to_pulumi_object_field(
                    "cloudSqlTarget",
                    &self.r#cloud_sql_target,
                ),
                to_pulumi_object_field(
                    "cloudStorageTarget",
                    &self.r#cloud_storage_target,
                ),
                to_pulumi_object_field(
                    "secretsTarget",
                    &self.r#secrets_target,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("bigQueryTarget") {
                            Some(value) => value,
                            None => bail!("Missing field 'bigQueryTarget' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_sql_target: {
                        let field_value = match fields_map.get("cloudSqlTarget") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudSqlTarget' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_storage_target: {
                        let field_value = match fields_map.get("cloudStorageTarget") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudStorageTarget' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets_target: {
                        let field_value = match fields_map.get("secretsTarget") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretsTarget' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
