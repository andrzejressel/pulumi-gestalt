#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordSetRoutingPolicyPrimaryBackup {
    /// The backup geo targets, which provide a regional failover policy for the otherwise global primary targets.
    /// Structure is document above.
    #[builder(into)]
    #[serde(rename = "backupGeos")]
    pub r#backup_geos: Vec<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackupBackupGeo>,
    /// Specifies whether to enable fencing for backup geo queries.
    #[builder(into)]
    #[serde(rename = "enableGeoFencingForBackups")]
    pub r#enable_geo_fencing_for_backups: Option<bool>,
    /// The list of global primary targets to be health checked.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackupPrimary>,
    /// Specifies the percentage of traffic to send to the backup targets even when the primary targets are healthy.
    #[builder(into)]
    #[serde(rename = "trickleRatio")]
    pub r#trickle_ratio: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RecordSetRoutingPolicyPrimaryBackup {
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
                    "backup_geos",
                    &self.r#backup_geos,
                ),
                to_pulumi_object_field(
                    "enable_geo_fencing_for_backups",
                    &self.r#enable_geo_fencing_for_backups,
                ),
                to_pulumi_object_field(
                    "primary",
                    &self.r#primary,
                ),
                to_pulumi_object_field(
                    "trickle_ratio",
                    &self.r#trickle_ratio,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RecordSetRoutingPolicyPrimaryBackup {
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
                    r#backup_geos: {
                        let field_value = match fields_map.get("backup_geos") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup_geos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_geo_fencing_for_backups: {
                        let field_value = match fields_map.get("enable_geo_fencing_for_backups") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_geo_fencing_for_backups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary: {
                        let field_value = match fields_map.get("primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trickle_ratio: {
                        let field_value = match fields_map.get("trickle_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'trickle_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
