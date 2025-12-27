#[macro_export]
macro_rules! sym {
    ($s:literal) => {
        $crate::abi::symbols::sym($s)
    };
}

#[macro_export]
macro_rules! thing_try {
    (id: $id:expr, kind: $kind:expr, body: $body:expr $(,)?) => {{
        $crate::Thing::try_with($id, $kind, &$body)
    }};
}

#[macro_export]
macro_rules! thing {
    (id: $id:expr, kind: $kind:expr, body: $body:expr $(,)?) => {{
        $crate::Thing::with($id, $kind, &$body)
    }};
}

#[macro_export]
macro_rules! link {
    ($from:expr, $pred:expr, $to:expr) => {
        $crate::link::LinkBody {
            from: $from,
            to: $to,
            predicate: $pred,
        }
    };
}

#[macro_export]
macro_rules! typed_thing {
    (id: $id:expr, kind: $kind:expr, value: $val:expr $(,)?) => {{
        let v = &$val;
        // Allows using this macro where thing_std is available as a crate
        // or re-exported.
        use ::thing_std::typed::ThingType;
        let bytes = ThingType::encode(v).expect("encode failed");
        $crate::Thing {
            id: $id,
            kind: $kind,
            body: $crate::ThingBody { bytes },
        }
    }};
}
