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
