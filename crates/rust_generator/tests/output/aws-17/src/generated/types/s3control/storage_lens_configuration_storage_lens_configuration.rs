#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StorageLensConfigurationStorageLensConfiguration {
    /// The account-level configurations of the S3 Storage Lens configuration. See Account Level below for more details.
    #[builder(into)]
    #[serde(rename = "accountLevel")]
    pub r#account_level: Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevel>,
    /// The Amazon Web Services organization for the S3 Storage Lens configuration. See AWS Org below for more details.
    #[builder(into)]
    #[serde(rename = "awsOrg")]
    pub r#aws_org: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAwsOrg>>,
    /// Properties of S3 Storage Lens metrics export including the destination, schema and format. See Data Export below for more details.
    #[builder(into)]
    #[serde(rename = "dataExport")]
    pub r#data_export: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExport>>,
    /// Whether the S3 Storage Lens configuration is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// What is excluded in this configuration. Conflicts with `include`. See Exclude below for more details.
    #[builder(into)]
    #[serde(rename = "exclude")]
    pub r#exclude: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationExclude>>,
    /// What is included in this configuration. Conflicts with `exclude`. See Include below for more details.
    #[builder(into)]
    #[serde(rename = "include")]
    pub r#include: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationInclude>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StorageLensConfigurationStorageLensConfiguration {
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
                    "account_level",
                    &self.r#account_level,
                ),
                to_pulumi_object_field(
                    "aws_org",
                    &self.r#aws_org,
                ),
                to_pulumi_object_field(
                    "data_export",
                    &self.r#data_export,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "exclude",
                    &self.r#exclude,
                ),
                to_pulumi_object_field(
                    "include",
                    &self.r#include,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StorageLensConfigurationStorageLensConfiguration {
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
                    r#account_level: {
                        let field_value = match fields_map.get("account_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aws_org: {
                        let field_value = match fields_map.get("aws_org") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_org' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_export: {
                        let field_value = match fields_map.get("data_export") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_export' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude: {
                        let field_value = match fields_map.get("exclude") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include: {
                        let field_value = match fields_map.get("include") {
                            Some(value) => value,
                            None => bail!("Missing field 'include' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
