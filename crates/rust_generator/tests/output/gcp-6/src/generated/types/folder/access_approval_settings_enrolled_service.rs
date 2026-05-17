#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApprovalSettingsEnrolledService {
    /// The product for which Access Approval will be enrolled. Allowed values are listed (case-sensitive):
    /// * all
    /// * App Engine
    /// * BigQuery
    /// * Cloud Bigtable
    /// * Cloud Key Management Service
    /// * Compute Engine
    /// * Cloud Dataflow
    /// * Cloud Identity and Access Management
    /// * Cloud Pub/Sub
    /// * Cloud Storage
    /// * Persistent Disk
    /// Note: These values are supported as input, but considered a legacy format:
    /// * all
    /// * appengine.googleapis.com
    /// * bigquery.googleapis.com
    /// * bigtable.googleapis.com
    /// * cloudkms.googleapis.com
    /// * compute.googleapis.com
    /// * dataflow.googleapis.com
    /// * iam.googleapis.com
    /// * pubsub.googleapis.com
    /// * storage.googleapis.com
    #[builder(into)]
    #[serde(rename = "cloudProduct")]
    pub r#cloud_product: String,
    /// The enrollment level of the service.
    /// Default value is `BLOCK_ALL`.
    /// Possible values are: `BLOCK_ALL`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "enrollmentLevel")]
    pub r#enrollment_level: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessApprovalSettingsEnrolledService {
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
                    "cloud_product",
                    &self.r#cloud_product,
                ),
                to_pulumi_object_field(
                    "enrollment_level",
                    &self.r#enrollment_level,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessApprovalSettingsEnrolledService {
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
                    r#cloud_product: {
                        let field_value = match fields_map.get("cloud_product") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_product' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enrollment_level: {
                        let field_value = match fields_map.get("enrollment_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'enrollment_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
