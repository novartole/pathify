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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_root() {
        pathify! { SingleRoot }

        let root = SingleRoot::default();
        assert_eq!(root.to_string(), "single_root");
    }

    #[test]
    fn different_levels() {
        pathify! {
            Root {
                Node1
                Node2 {
                    Node21
                    Node22
                }
                Node3 {
                    Node31 {
                        Node311
                    }
                    Node32
                }
                Node4 {
                    Node41
                    Node42 {
                        Node421
                    }
                }
                Node5 {
                    Node51 {
                        Node511
                    }
                    Node52 {
                        Node521
                    }
                }
            }
        }

        let root = Root::default();
        assert_eq!(root.to_string(), "root");
        assert_eq!(root.node1.to_string(), "root.node_1");
        assert_eq!(root.node2.to_string(), "root.node_2");
        assert_eq!(root.node2.node21.to_string(), "root.node_2.node_21");
        assert_eq!(root.node2.node22.to_string(), "root.node_2.node_22");
        assert_eq!(root.node3.to_string(), "root.node_3");
        assert_eq!(root.node3.node31.to_string(), "root.node_3.node_31");
        assert_eq!(
            root.node3.node31.node311.to_string(),
            "root.node_3.node_31.node_311"
        );
        assert_eq!(root.node3.node32.to_string(), "root.node_3.node_32");
        assert_eq!(root.node4.to_string(), "root.node_4");
        assert_eq!(root.node4.node41.to_string(), "root.node_4.node_41");
        assert_eq!(
            root.node4.node42.node421.to_string(),
            "root.node_4.node_42.node_421"
        );
        assert_eq!(root.node5.to_string(), "root.node_5");
        assert_eq!(root.node5.node51.to_string(), "root.node_5.node_51");
        assert_eq!(
            root.node5.node51.node511.to_string(),
            "root.node_5.node_51.node_511"
        );
        assert_eq!(
            root.node5.node52.node521.to_string(),
            "root.node_5.node_52.node_521"
        );
    }
}
