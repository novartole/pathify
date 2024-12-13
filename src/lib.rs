#[macro_export]
macro_rules! pathify {
    // single root
    ($s:ident) => {
        pathify! { @_ $s "" }
    };

    // (p)arent has (c)hildren:
    ($p:ident { $($c:tt)+ }) => {
        pathify! { @_ $p ("") $($c)+ }
    };

    // turn leaf into struct
    (@_ $s:ident $v:expr) => {
        #[derive(smart_default::SmartDefault, Debug)]
        pub struct $s {
            #[default = pathify!(@concat $v, $s)]
            path: &'static str,
        }

        impl std::fmt::Display for $s {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use convert_case::{Case, Casing};

                write!(f, "{}", self.path.to_case(Case::Snake))
            }
        }
    };

    // handle (p)arent w\ (c)hildren
    (@_ $p:ident ($v:expr) $($c:tt)+) => {
        pathify! { @_ () $p ($($c)+) $v }
    };

    // (p)arent's (c)hild is a leaf,
    // but there is a (n)ext child,
    // and also some other child (t)okens may exist
    (@_ ($($s:ident)*) $p:ident ($c:ident $n:ident $($t:tt)*) $v:expr) => {
        pathify! { @_ $c pathify!(@concat $v, $p) }

        pathify! { @_ ($($s)* $c) $p ($n $($t)*) $v }
    };

    // (p)arent's (c)hild is a parent w\ children (cc),
    // but some other child (t)okens of the top parent may exist
    (@_ ($($s:ident)*) $p:ident ($c:ident { $($cc:tt)+ } $($t:tt)*) $v:expr) => {
        pathify! { @_ $c (pathify!(@concat $v, $p)) $($cc)+ }

        pathify! { @_ ($($s)* $c) $p ($($t)*) $v }
    };

    // last (p)arent's (c)hild found,
    // so turn all collected (s)tructs into fields
    (@_ ($($s:ident)*) $p:ident ($($c:ident)?) $v:expr) => {
        paste::paste! {
            $(
                pathify! { @_ $c pathify!(@concat $v, $p) }
            )?

            #[derive(smart_default::SmartDefault, Debug)]
            pub struct $p {
                #[default = pathify!(@concat $v, $p)]
                path: &'static str,
                $(
                    pub [<$c:snake>]: $c,
                )?
                $(
                    pub [<$s:snake>]: $s,
                )*
            }

            impl std::fmt::Display for $p {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    use convert_case::{Case, Casing};

                    write!(f, "{}", self.path.to_case(Case::Snake))
                }
            }
        }
    };

    // ignore prefix for literals;
    // it should only happen for root
    (@concat $_:literal, $b:ident) => {
        stringify!($b)
    };

    // keep concatination in one place
    (@concat $a:expr, $b:ident) => {
        concat!($a, ".", stringify!($b))
    };
}
