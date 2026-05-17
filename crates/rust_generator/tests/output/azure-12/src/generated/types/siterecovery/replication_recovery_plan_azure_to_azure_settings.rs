#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicationRecoveryPlanAzureToAzureSettings {
    /// The Edge Zone within the Azure Region where the VM exists. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    #[builder(into)]
    #[serde(rename = "primaryEdgeZone")]
    pub r#primary_edge_zone: Option<String>,
    /// The Availability Zone in which the VM is located. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    #[builder(into)]
    #[serde(rename = "primaryZone")]
    pub r#primary_zone: Option<String>,
    /// The Edge Zone within the Azure Region where the VM is recovered. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    /// 
    /// > **Note:** `primary_edge_zone` and `recovery_edge_zone` must be specified together.
    #[builder(into)]
    #[serde(rename = "recoveryEdgeZone")]
    pub r#recovery_edge_zone: Option<String>,
    /// The Availability Zone in which the VM is recovered. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    /// 
    /// > **Note:** `primary_zone` and `recovery_zone` must be specified together.
    #[builder(into)]
    #[serde(rename = "recoveryZone")]
    pub r#recovery_zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicationRecoveryPlanAzureToAzureSettings {
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
                "primary_edge_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary_edge_zone,
                )
                .await,
            );
            map.insert(
                "primary_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary_zone,
                )
                .await,
            );
            map.insert(
                "recovery_edge_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recovery_edge_zone,
                )
                .await,
            );
            map.insert(
                "recovery_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recovery_zone,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicationRecoveryPlanAzureToAzureSettings {
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
                    r#primary_edge_zone: {
                        let field_value = match fields_map.get("primary_edge_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_edge_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_zone: {
                        let field_value = match fields_map.get("primary_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_edge_zone: {
                        let field_value = match fields_map.get("recovery_edge_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_edge_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_zone: {
                        let field_value = match fields_map.get("recovery_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
