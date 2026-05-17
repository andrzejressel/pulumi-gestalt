#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordSetRoutingPolicy {
    /// Specifies whether to enable fencing for geo queries.
    #[builder(into)]
    #[serde(rename = "enableGeoFencing")]
    pub r#enable_geo_fencing: Option<bool>,
    /// The configuration for Geolocation based routing policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<super::super::types::dns::RecordSetRoutingPolicyGeo>>,
    /// The configuration for a failover policy with global to regional failover. Queries are responded to with the global primary targets, but if none of the primary targets are healthy, then we fallback to a regional failover policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "primaryBackup")]
    pub r#primary_backup: Option<Box<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackup>>,
    /// The configuration for Weighted Round Robin based routing policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "wrrs")]
    pub r#wrrs: Option<Vec<super::super::types::dns::RecordSetRoutingPolicyWrr>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RecordSetRoutingPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "enable_geo_fencing",
                    &self.r#enable_geo_fencing,
                ),
                to_pulumi_object_field(
                    "geos",
                    &self.r#geos,
                ),
                to_pulumi_object_field(
                    "primary_backup",
                    &self.r#primary_backup,
                ),
                to_pulumi_object_field(
                    "wrrs",
                    &self.r#wrrs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RecordSetRoutingPolicy {
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
                    r#enable_geo_fencing: {
                        let field_value = match fields_map.get("enable_geo_fencing") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_geo_fencing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geos: {
                        let field_value = match fields_map.get("geos") {
                            Some(value) => value,
                            None => bail!("Missing field 'geos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_backup: {
                        let field_value = match fields_map.get("primary_backup") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_backup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wrrs: {
                        let field_value = match fields_map.get("wrrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'wrrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
