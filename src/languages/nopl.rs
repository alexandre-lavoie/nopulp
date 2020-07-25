#[macro_export]
macro_rules! nopl {
    (($tag:ident $($key:ident => $value:expr;)* {$val:expr})) => {
        Box::new($tag {
            0: Object {
                style: Style {
                    $($key: $value,)*
                    ..Style::default()
                },
                value: Some($val),
                ..Object::default()
            }
        })
    };

    (($tag:ident $($key:ident => $value:expr;)* [$($child:tt)*])) => {
        Box::new($tag {
            0: Object {
                style: Style {
                    $($key: $value,)*
                    ..Style::default()
                },
                children: vec![$(nopl!($child),)*],
                ..Object::default()
            }
        })
    };

    (($tag:ident $($key:ident => $value:expr;)*)) => {
        nopl! (($tag $($key => $value;)* []))
    };

    ($($key:ident => $value:expr;)* [$($child:tt)*]) => {
        Page {
            0: Object {
                style: Style {
                    $($key: $value,)*
                    ..Style::default()
                },
                children: vec![$(nopl!($child),)*],
                value: Some("Undefined".to_string()),
                ..Object::default()
            }
        }
    }
}