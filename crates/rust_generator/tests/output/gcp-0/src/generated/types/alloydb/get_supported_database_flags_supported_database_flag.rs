#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSupportedDatabaseFlagsSupportedDatabaseFlag {
    /// Whether the database flag accepts multiple values. If true, a comma-separated list of stringified values may be specified.
    #[builder(into)]
    #[serde(rename = "acceptsMultipleValues")]
    pub r#accepts_multiple_values: bool,
    /// The name of the database flag, e.g. "max_allowed_packets". The is a possibly key for the Instance.database_flags map field.
    #[builder(into)]
    #[serde(rename = "flagName")]
    pub r#flag_name: String,
    /// Restriction on `INTEGER` type value. Specifies the minimum value and the maximum value that can be specified, if applicable.
    #[builder(into)]
    #[serde(rename = "integerRestrictions")]
    pub r#integer_restrictions: Box<super::super::types::alloydb::GetSupportedDatabaseFlagsSupportedDatabaseFlagIntegerRestrictions>,
    /// The name of the flag resource, following Google Cloud conventions, e.g.: * projects/{project}/locations/{location}/flags/{flag} This field currently has no semantic meaning.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Whether setting or updating this flag on an Instance requires a database restart. If a flag that requires database restart is set, the backend will automatically restart the database (making sure to satisfy any availability SLO's).
    #[builder(into)]
    #[serde(rename = "requiresDbRestart")]
    pub r#requires_db_restart: bool,
    /// Restriction on `STRING` type value. The list of allowed values, if bounded. This field will be empty if there is a unbounded number of allowed values.
    #[builder(into)]
    #[serde(rename = "stringRestrictions")]
    pub r#string_restrictions: Box<super::super::types::alloydb::GetSupportedDatabaseFlagsSupportedDatabaseFlagStringRestrictions>,
    /// Major database engine versions for which this flag is supported. The supported values are `POSTGRES_14` and `DATABASE_VERSION_UNSPECIFIED`.
    #[builder(into)]
    #[serde(rename = "supportedDbVersions")]
    pub r#supported_db_versions: Vec<String>,
    /// ValueType describes the semantic type of the value that the flag accepts. Regardless of the ValueType, the Instance.database_flags field accepts the stringified version of the value, i.e. "20" or "3.14". The supported values are `VALUE_TYPE_UNSPECIFIED`, `STRING`, `INTEGER`, `FLOAT` and `NONE`.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSupportedDatabaseFlagsSupportedDatabaseFlag {
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
                "accepts_multiple_values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accepts_multiple_values,
                )
                .await,
            );
            map.insert(
                "flag_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flag_name,
                )
                .await,
            );
            map.insert(
                "integer_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#integer_restrictions,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "requires_db_restart".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#requires_db_restart,
                )
                .await,
            );
            map.insert(
                "string_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#string_restrictions,
                )
                .await,
            );
            map.insert(
                "supported_db_versions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#supported_db_versions,
                )
                .await,
            );
            map.insert(
                "value_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSupportedDatabaseFlagsSupportedDatabaseFlag {
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
                    r#accepts_multiple_values: {
                        let field_value = match fields_map.get("accepts_multiple_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'accepts_multiple_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flag_name: {
                        let field_value = match fields_map.get("flag_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'flag_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integer_restrictions: {
                        let field_value = match fields_map.get("integer_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'integer_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requires_db_restart: {
                        let field_value = match fields_map.get("requires_db_restart") {
                            Some(value) => value,
                            None => bail!("Missing field 'requires_db_restart' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_restrictions: {
                        let field_value = match fields_map.get("string_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#supported_db_versions: {
                        let field_value = match fields_map.get("supported_db_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_db_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_type: {
                        let field_value = match fields_map.get("value_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
