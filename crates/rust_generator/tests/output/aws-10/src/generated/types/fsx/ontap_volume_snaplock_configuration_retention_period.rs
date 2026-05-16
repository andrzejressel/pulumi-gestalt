#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OntapVolumeSnaplockConfigurationRetentionPeriod {
    /// The retention period assigned to a write once, read many (WORM) file by default if an explicit retention period is not set for an FSx for ONTAP SnapLock volume. The default retention period must be greater than or equal to the minimum retention period and less than or equal to the maximum retention period. See `default_retention` Block for details.
    #[builder(into)]
    #[serde(rename = "defaultRetention")]
    pub r#default_retention: Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodDefaultRetention>>,
    /// The longest retention period that can be assigned to a WORM file on an FSx for ONTAP SnapLock volume. See `maximum_retention` Block for details.
    #[builder(into)]
    #[serde(rename = "maximumRetention")]
    pub r#maximum_retention: Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodMaximumRetention>>,
    /// The shortest retention period that can be assigned to a WORM file on an FSx for ONTAP SnapLock volume. See `minimum_retention` Block for details.
    #[builder(into)]
    #[serde(rename = "minimumRetention")]
    pub r#minimum_retention: Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodMinimumRetention>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OntapVolumeSnaplockConfigurationRetentionPeriod {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("default_retention".to_string(), self.r#default_retention.to_pulumi_value().await);
            map.insert("maximum_retention".to_string(), self.r#maximum_retention.to_pulumi_value().await);
            map.insert("minimum_retention".to_string(), self.r#minimum_retention.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OntapVolumeSnaplockConfigurationRetentionPeriod {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#default_retention: {
                        let field_value = match fields_map.get("default_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodDefaultRetention>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#maximum_retention: {
                        let field_value = match fields_map.get("maximum_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodMaximumRetention>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#minimum_retention: {
                        let field_value = match fields_map.get("minimum_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodMinimumRetention>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
