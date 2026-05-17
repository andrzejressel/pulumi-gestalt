#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNames {
    /// An `application_server` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "applicationServer")]
    pub r#application_server: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServer>>,
    /// A `central_server` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "centralServer")]
    pub r#central_server: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServer>>,
    /// A `database_server` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "databaseServer")]
    pub r#database_server: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesDatabaseServer>>,
    /// A `shared_storage` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sharedStorage")]
    pub r#shared_storage: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesSharedStorage>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNames {
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
                "application_server".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_server,
                )
                .await,
            );
            map.insert(
                "central_server".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#central_server,
                )
                .await,
            );
            map.insert(
                "database_server".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_server,
                )
                .await,
            );
            map.insert(
                "shared_storage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shared_storage,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNames {
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
                    r#application_server: {
                        let field_value = match fields_map.get("application_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#central_server: {
                        let field_value = match fields_map.get("central_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'central_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_server: {
                        let field_value = match fields_map.get("database_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_storage: {
                        let field_value = match fields_map.get("shared_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'shared_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
