#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAutonomousDatabasePropertyConnectionUrl {
    /// Oracle Application Express (APEX) URL.
    #[builder(into)]
    #[serde(rename = "apexUri")]
    pub r#apex_uri: String,
    /// The URL of the Database Transforms for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "databaseTransformsUri")]
    pub r#database_transforms_uri: String,
    /// The URL of the Graph Studio for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "graphStudioUri")]
    pub r#graph_studio_uri: String,
    /// The URL of the Oracle Machine Learning (OML) Notebook for the Autonomous
    /// Database.
    #[builder(into)]
    #[serde(rename = "machineLearningNotebookUri")]
    pub r#machine_learning_notebook_uri: String,
    /// The URL of Machine Learning user management the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "machineLearningUserManagementUri")]
    pub r#machine_learning_user_management_uri: String,
    /// The URL of the MongoDB API for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "mongoDbUri")]
    pub r#mongo_db_uri: String,
    /// The Oracle REST Data Services (ORDS) URL of the Web Access for the
    /// Autonomous Database.
    #[builder(into)]
    #[serde(rename = "ordsUri")]
    pub r#ords_uri: String,
    /// The URL of the Oracle SQL Developer Web for the Autonomous Database.
    #[builder(into)]
    #[serde(rename = "sqlDevWebUri")]
    pub r#sql_dev_web_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAutonomousDatabasePropertyConnectionUrl {
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
                "apex_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apex_uri,
                )
                .await,
            );
            map.insert(
                "database_transforms_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_transforms_uri,
                )
                .await,
            );
            map.insert(
                "graph_studio_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#graph_studio_uri,
                )
                .await,
            );
            map.insert(
                "machine_learning_notebook_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#machine_learning_notebook_uri,
                )
                .await,
            );
            map.insert(
                "machine_learning_user_management_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#machine_learning_user_management_uri,
                )
                .await,
            );
            map.insert(
                "mongo_db_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mongo_db_uri,
                )
                .await,
            );
            map.insert(
                "ords_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ords_uri,
                )
                .await,
            );
            map.insert(
                "sql_dev_web_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sql_dev_web_uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAutonomousDatabasePropertyConnectionUrl {
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
                    r#apex_uri: {
                        let field_value = match fields_map.get("apex_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'apex_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_transforms_uri: {
                        let field_value = match fields_map.get("database_transforms_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_transforms_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#graph_studio_uri: {
                        let field_value = match fields_map.get("graph_studio_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'graph_studio_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_learning_notebook_uri: {
                        let field_value = match fields_map.get("machine_learning_notebook_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_learning_notebook_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_learning_user_management_uri: {
                        let field_value = match fields_map.get("machine_learning_user_management_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_learning_user_management_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mongo_db_uri: {
                        let field_value = match fields_map.get("mongo_db_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'mongo_db_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ords_uri: {
                        let field_value = match fields_map.get("ords_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'ords_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_dev_web_uri: {
                        let field_value = match fields_map.get("sql_dev_web_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_dev_web_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
