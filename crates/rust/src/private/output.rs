use crate::Output;
use pulumi_gestalt_model::ResolvedOutput;

macro_rules! impl_combine {
    ($($func_name:ident => ($($var_lower:ident : $var_upper:ident),+)),+) => {
        $(
            #[allow(clippy::too_many_arguments)]
            pub fn $func_name<A, $($var_upper),+>(a: crate::Output<A>, $($var_lower: crate::Output<$var_upper>),+) -> Output<(A, $($var_upper),+)>
            where
                A: Clone + Send + Sync + 'static,
                $($var_upper: Clone + Send + Sync + 'static),+
            {
                Output::from_resolved_future(async move {
                    let a_result = a.resolve().await;
                    $(let $var_lower = $var_lower.resolve().await;)+

                    let mut dependencies = a_result.dependencies;
                    let mut secret = a_result.secret;
                    $(dependencies.extend($var_lower.dependencies); secret |= $var_lower.secret;)+

                    let value = match (a_result.value, $($var_lower.value),+) {
                        (Some(a_value), $(Some($var_lower)),+) => Some((a_value, $($var_lower),+)),
                        _ => None,
                    };

                    ResolvedOutput {
                        value,
                        secret,
                        dependencies,
                    }
                })
            }
        )+
    };
}

impl_combine! {
    combine2 => (b: B),
    combine3 => (b: B, c: C),
    combine4 => (b: B, c: C, d: D),
    combine5 => (b: B, c: C, d: D, e: E),
    combine6 => (b: B, c: C, d: D, e: E, f: F),
    combine7 => (b: B, c: C, d: D, e: E, f: F, g: G),
    combine8 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H),
    combine9 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I),
    combine10 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J),
    combine11 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K),
    combine12 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L),
    combine13 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M),
    combine14 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N),
    combine15 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N, o: O),
    combine16 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N, o: O, p: P)
}
