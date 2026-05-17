#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceClone {
    /// The name of the allocated ip range for the private ip CloudSQL instance. For example: "google-managed-services-default". If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression a-z?.
    #[builder(into)]
    #[serde(rename = "allocatedIpRange")]
    pub r#allocated_ip_range: Option<String>,
    /// (SQL Server only, use with `point_in_time`) Clone only the specified databases from the source instance. Clone all databases if empty.
    #[builder(into)]
    #[serde(rename = "databaseNames")]
    pub r#database_names: Option<Vec<String>>,
    /// The timestamp of the point in time that should be restored.
    /// 
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "pointInTime")]
    pub r#point_in_time: Option<String>,
    /// (Point-in-time recovery for PostgreSQL only) Clone to an instance in the specified zone. If no zone is specified, clone to the same zone as the source instance. [clone-unavailable-instance](https://cloud.google.com/sql/docs/postgres/clone-instance#clone-unavailable-instance)
    #[builder(into)]
    #[serde(rename = "preferredZone")]
    pub r#preferred_zone: Option<String>,
    /// Name of the source instance which will be cloned.
    #[builder(into)]
    #[serde(rename = "sourceInstanceName")]
    pub r#source_instance_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceClone {
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
                "allocated_ip_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocated_ip_range,
                )
                .await,
            );
            map.insert(
                "database_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_names,
                )
                .await,
            );
            map.insert(
                "point_in_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#point_in_time,
                )
                .await,
            );
            map.insert(
                "preferred_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preferred_zone,
                )
                .await,
            );
            map.insert(
                "source_instance_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_instance_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceClone {
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
                    r#allocated_ip_range: {
                        let field_value = match fields_map.get("allocated_ip_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocated_ip_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_names: {
                        let field_value = match fields_map.get("database_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#point_in_time: {
                        let field_value = match fields_map.get("point_in_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'point_in_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preferred_zone: {
                        let field_value = match fields_map.get("preferred_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_instance_name: {
                        let field_value = match fields_map.get("source_instance_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_instance_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
