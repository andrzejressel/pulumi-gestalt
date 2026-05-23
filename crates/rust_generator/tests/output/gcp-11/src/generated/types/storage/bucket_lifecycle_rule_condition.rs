#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleRuleCondition {
    /// Minimum age of an object in days to satisfy this condition. **Note** To set `0` value of `age`, `send_age_if_zero` should be set `true` otherwise `0` value of `age` field will be ignored.
    #[builder(into)]
    pub r#age: Option<i32>,
    /// A date in the RFC 3339 format YYYY-MM-DD. This condition is satisfied when an object is created before midnight of the specified date in UTC.
    #[builder(into)]
    pub r#created_before: Option<String>,
    /// A date in the RFC 3339 format YYYY-MM-DD. This condition is satisfied when the customTime metadata for the object is set to an earlier date than the date used in this lifecycle condition.
    #[builder(into)]
    pub r#custom_time_before: Option<String>,
    /// Number of days elapsed since the user-specified timestamp set on an object.
    #[builder(into)]
    pub r#days_since_custom_time: Option<i32>,
    /// Number of days elapsed since the noncurrent timestamp of an object. This
    /// condition is relevant only for versioned objects.
    #[builder(into)]
    pub r#days_since_noncurrent_time: Option<i32>,
    /// One or more matching name prefixes to satisfy this condition.
    #[builder(into)]
    pub r#matches_prefixes: Option<Vec<String>>,
    /// [Storage Class](https://cloud.google.com/storage/docs/storage-classes) of objects to satisfy this condition. Supported values include: `STANDARD`, `MULTI_REGIONAL`, `REGIONAL`, `NEARLINE`, `COLDLINE`, `ARCHIVE`, `DURABLE_REDUCED_AVAILABILITY`.
    #[builder(into)]
    pub r#matches_storage_classes: Option<Vec<String>>,
    /// One or more matching name suffixes to satisfy this condition.
    #[builder(into)]
    pub r#matches_suffixes: Option<Vec<String>>,
    /// Creation date of an object in RFC 3339 (e.g. 2017-06-13) to satisfy this condition.
    #[builder(into)]
    pub r#noncurrent_time_before: Option<String>,
    /// Relevant only for versioned objects. The number of newer versions of an object to satisfy this condition.
    #[builder(into)]
    pub r#num_newer_versions: Option<i32>,
    /// While set true, `age` value will be sent in the request even for zero value of the field. This field is only useful and required for setting 0 value to the `age` field. It can be used alone or together with `age` attribute. **NOTE** `age` attibute with `0` value will be ommitted from the API request if `send_age_if_zero` field is having `false` value.
    #[builder(into)]
    pub r#send_age_if_zero: Option<bool>,
    /// While set true, `days_since_custom_time` value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the `days_since_custom_time` field. It can be used alone or together with `days_since_custom_time`.
    #[builder(into)]
    pub r#send_days_since_custom_time_if_zero: Option<bool>,
    /// While set true, `days_since_noncurrent_time` value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the `days_since_noncurrent_time` field. It can be used alone or together with `days_since_noncurrent_time`.
    #[builder(into)]
    pub r#send_days_since_noncurrent_time_if_zero: Option<bool>,
    /// While set true, `num_newer_versions` value will be sent in the request even for zero value of the field. This field is only useful for setting 0 value to the `num_newer_versions` field. It can be used alone or together with `num_newer_versions`.
    #[builder(into)]
    pub r#send_num_newer_versions_if_zero: Option<bool>,
    /// Match to live and/or archived objects. Unversioned buckets have only live objects. Supported values include: `"LIVE"`, `"ARCHIVED"`, `"ANY"`.
    #[builder(into)]
    pub r#with_state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketLifecycleRuleCondition {
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
                    "age",
                    &self.r#age,
                ),
                to_pulumi_object_field(
                    "createdBefore",
                    &self.r#created_before,
                ),
                to_pulumi_object_field(
                    "customTimeBefore",
                    &self.r#custom_time_before,
                ),
                to_pulumi_object_field(
                    "daysSinceCustomTime",
                    &self.r#days_since_custom_time,
                ),
                to_pulumi_object_field(
                    "daysSinceNoncurrentTime",
                    &self.r#days_since_noncurrent_time,
                ),
                to_pulumi_object_field(
                    "matchesPrefixes",
                    &self.r#matches_prefixes,
                ),
                to_pulumi_object_field(
                    "matchesStorageClasses",
                    &self.r#matches_storage_classes,
                ),
                to_pulumi_object_field(
                    "matchesSuffixes",
                    &self.r#matches_suffixes,
                ),
                to_pulumi_object_field(
                    "noncurrentTimeBefore",
                    &self.r#noncurrent_time_before,
                ),
                to_pulumi_object_field(
                    "numNewerVersions",
                    &self.r#num_newer_versions,
                ),
                to_pulumi_object_field(
                    "sendAgeIfZero",
                    &self.r#send_age_if_zero,
                ),
                to_pulumi_object_field(
                    "sendDaysSinceCustomTimeIfZero",
                    &self.r#send_days_since_custom_time_if_zero,
                ),
                to_pulumi_object_field(
                    "sendDaysSinceNoncurrentTimeIfZero",
                    &self.r#send_days_since_noncurrent_time_if_zero,
                ),
                to_pulumi_object_field(
                    "sendNumNewerVersionsIfZero",
                    &self.r#send_num_newer_versions_if_zero,
                ),
                to_pulumi_object_field(
                    "withState",
                    &self.r#with_state,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketLifecycleRuleCondition {
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
                    r#age: {
                        let field_value = match fields_map.get("age") {
                            Some(value) => value,
                            None => bail!("Missing field 'age' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#created_before: {
                        let field_value = match fields_map.get("createdBefore") {
                            Some(value) => value,
                            None => bail!("Missing field 'createdBefore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_time_before: {
                        let field_value = match fields_map.get("customTimeBefore") {
                            Some(value) => value,
                            None => bail!("Missing field 'customTimeBefore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_since_custom_time: {
                        let field_value = match fields_map.get("daysSinceCustomTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'daysSinceCustomTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_since_noncurrent_time: {
                        let field_value = match fields_map.get("daysSinceNoncurrentTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'daysSinceNoncurrentTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matches_prefixes: {
                        let field_value = match fields_map.get("matchesPrefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'matchesPrefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matches_storage_classes: {
                        let field_value = match fields_map.get("matchesStorageClasses") {
                            Some(value) => value,
                            None => bail!("Missing field 'matchesStorageClasses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matches_suffixes: {
                        let field_value = match fields_map.get("matchesSuffixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'matchesSuffixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_time_before: {
                        let field_value = match fields_map.get("noncurrentTimeBefore") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrentTimeBefore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_newer_versions: {
                        let field_value = match fields_map.get("numNewerVersions") {
                            Some(value) => value,
                            None => bail!("Missing field 'numNewerVersions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_age_if_zero: {
                        let field_value = match fields_map.get("sendAgeIfZero") {
                            Some(value) => value,
                            None => bail!("Missing field 'sendAgeIfZero' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_days_since_custom_time_if_zero: {
                        let field_value = match fields_map.get("sendDaysSinceCustomTimeIfZero") {
                            Some(value) => value,
                            None => bail!("Missing field 'sendDaysSinceCustomTimeIfZero' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_days_since_noncurrent_time_if_zero: {
                        let field_value = match fields_map.get("sendDaysSinceNoncurrentTimeIfZero") {
                            Some(value) => value,
                            None => bail!("Missing field 'sendDaysSinceNoncurrentTimeIfZero' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_num_newer_versions_if_zero: {
                        let field_value = match fields_map.get("sendNumNewerVersionsIfZero") {
                            Some(value) => value,
                            None => bail!("Missing field 'sendNumNewerVersionsIfZero' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#with_state: {
                        let field_value = match fields_map.get("withState") {
                            Some(value) => value,
                            None => bail!("Missing field 'withState' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
