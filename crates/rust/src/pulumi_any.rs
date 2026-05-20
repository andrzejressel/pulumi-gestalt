#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal {
    (null) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_middleware(
            $crate::__private::pulumi_gestalt_model::PulumiValueContent::None,
        )
    };
    ([$($tt:tt)*]) => {
        $crate::pulumi_any_v2_internal_array!([] $($tt)*)
    };
    ({$($tt:tt)*}) => {
        $crate::pulumi_any_v2_internal_object!([] $($tt)*)
    };
    ($other:expr) => {
        $crate::__private::pulumi_gestalt_model::__private::to_pulumi_value_middleware($other)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal_array {
    ([$($elems:expr,)*]) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_middleware_array(
            vec![$($elems,)*]
        )
    };
    ([$($elems:expr,)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!([$($elems,)*] $($rest)*)
    };
    ([$($elems:expr,)*] null , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!(null),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] [$($inner:tt)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!([$($inner)*]),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] {$($inner:tt)*} , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!({$($inner)*}),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] $next:expr , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!($next),
            ]
            $($rest)*
        )
    };
    ([$($elems:expr,)*] null) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!(null),
            ]
        )
    };
    ([$($elems:expr,)*] [$($inner:tt)*]) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!([$($inner)*]),
            ]
        )
    };
    ([$($elems:expr,)*] {$($inner:tt)*}) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!({$($inner)*}),
            ]
        )
    };
    ([$($elems:expr,)*] $next:expr) => {
        $crate::pulumi_any_v2_internal_array!(
            [
                $($elems,)*
                $crate::pulumi_any_v2_internal!($next),
            ]
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pulumi_any_v2_internal_object {
    ([$($fields:expr,)*]) => {
        $crate::__private::pulumi_gestalt_model::__private::pulumi_value_middleware_object(
            vec![$($fields,)*]
        )
    };
    ([$($fields:expr,)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!([$($fields,)*] $($rest)*)
    };
    ([$($fields:expr,)*] $key:tt : null , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!(null),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : [$($inner:tt)*] , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!([$($inner)*]),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : {$($inner:tt)*} , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!({$($inner)*}),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : $value:expr , $($rest:tt)*) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!($value),
                ),
            ]
            $($rest)*
        )
    };
    ([$($fields:expr,)*] $key:tt : null) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!(null),
                ),
            ]
        )
    };
    ([$($fields:expr,)*] $key:tt : [$($inner:tt)*]) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!([$($inner)*]),
                ),
            ]
        )
    };
    ([$($fields:expr,)*] $key:tt : {$($inner:tt)*}) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!({$($inner)*}),
                ),
            ]
        )
    };
    ([$($fields:expr,)*] $key:tt : $value:expr) => {
        $crate::pulumi_any_v2_internal_object!(
            [
                $($fields,)*
                (
                    ::std::convert::Into::<::std::string::String>::into($key),
                    $crate::pulumi_any_v2_internal!($value),
                ),
            ]
        )
    };
}

#[macro_export]
macro_rules! pulumi_any_v2 {
    ($($tt:tt)+) => {
        $crate::pulumi_any_v2_internal!($($tt)+)
    };
}

#[cfg(test)]
mod tests {
    use pulumi_gestalt_model::__private::futures::executor::block_on;
    use pulumi_gestalt_model::{PulumiValue, PulumiValueContent, ToPulumiValue};
    use std::collections::HashSet;

    struct CustomToPulumiValue {
        id: i32,
    }

    impl ToPulumiValue for CustomToPulumiValue {
        fn to_pulumi_value(&self) -> impl std::future::Future<Output = PulumiValue> + Send {
            let id = self.id;
            std::future::ready(PulumiValue {
                content: PulumiValueContent::Object(vec![(
                    "custom-id".to_string(),
                    pv(PulumiValueContent::Integer(id + 1000)),
                )]),
                secret: false,
                dependencies: HashSet::new(),
            })
        }
    }

    fn pv(content: PulumiValueContent) -> PulumiValue {
        PulumiValue {
            content,
            secret: false,
            dependencies: HashSet::new(),
        }
    }

    fn pvs(content: PulumiValueContent) -> PulumiValue {
        PulumiValue {
            content,
            secret: true,
            dependencies: HashSet::new(),
        }
    }

    #[test]
    fn pulumi_any_v2_macro_scalar() {
        let integer = block_on(pulumi_any_v2!(42).to_pulumi_value());
        assert_eq!(integer, pv(PulumiValueContent::Integer(42)));

        let float = block_on(pulumi_any_v2!(1.5f64).to_pulumi_value());
        assert_eq!(float, pv(PulumiValueContent::Number(1.5)));

        let boolean = block_on(pulumi_any_v2!(true).to_pulumi_value());
        assert_eq!(boolean, pv(PulumiValueContent::Boolean(true)));

        let string = block_on(pulumi_any_v2!("hello").to_pulumi_value());
        assert_eq!(string, pv(PulumiValueContent::String("hello".to_string())));
    }
}
