#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileCloudsql {
    /// (Output)
    /// Output only. The Cloud SQL instance ID that this connection profile is associated with.
    #[builder(into)]
    #[serde(rename = "cloudSqlId")]
    pub r#cloud_sql_id: Option<String>,
    /// (Output)
    /// Output only. The Cloud SQL database instance's private IP.
    #[builder(into)]
    #[serde(rename = "privateIp")]
    pub r#private_ip: Option<String>,
    /// (Output)
    /// Output only. The Cloud SQL database instance's public IP.
    #[builder(into)]
    #[serde(rename = "publicIp")]
    pub r#public_ip: Option<String>,
    /// Immutable. Metadata used to create the destination Cloud SQL database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionProfileCloudsql {
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
                "cloud_sql_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_sql_id,
                )
                .await,
            );
            map.insert(
                "private_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_ip,
                )
                .await,
            );
            map.insert(
                "public_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_ip,
                )
                .await,
            );
            map.insert(
                "settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionProfileCloudsql {
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
                    r#cloud_sql_id: {
                        let field_value = match fields_map.get("cloud_sql_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_sql_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip: {
                        let field_value = match fields_map.get("private_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip: {
                        let field_value = match fields_map.get("public_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#settings: {
                        let field_value = match fields_map.get("settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
