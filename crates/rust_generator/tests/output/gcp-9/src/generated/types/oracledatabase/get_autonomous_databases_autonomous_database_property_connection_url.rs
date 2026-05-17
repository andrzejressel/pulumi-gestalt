#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAutonomousDatabasesAutonomousDatabasePropertyConnectionUrl {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAutonomousDatabasesAutonomousDatabasePropertyConnectionUrl {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "apex_uri",
                    &self.r#apex_uri,
                ),
                to_pulumi_object_field(
                    "database_transforms_uri",
                    &self.r#database_transforms_uri,
                ),
                to_pulumi_object_field(
                    "graph_studio_uri",
                    &self.r#graph_studio_uri,
                ),
                to_pulumi_object_field(
                    "machine_learning_notebook_uri",
                    &self.r#machine_learning_notebook_uri,
                ),
                to_pulumi_object_field(
                    "machine_learning_user_management_uri",
                    &self.r#machine_learning_user_management_uri,
                ),
                to_pulumi_object_field(
                    "mongo_db_uri",
                    &self.r#mongo_db_uri,
                ),
                to_pulumi_object_field(
                    "ords_uri",
                    &self.r#ords_uri,
                ),
                to_pulumi_object_field(
                    "sql_dev_web_uri",
                    &self.r#sql_dev_web_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAutonomousDatabasesAutonomousDatabasePropertyConnectionUrl {
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
