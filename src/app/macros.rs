#[macro_export]
macro_rules! app {
    ($app: ident) => {
        use wasm_bindgen::prelude::*;

        #[cfg(feature = "wee_alloc")]
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
        
        #[wasm_bindgen(start)]
        pub fn run() -> Result<(), JsValue> {
            let mut app = App::new(());
        
            let mut page = $app::route(&mut app);
        
            app.render(&mut page);
        
            App::attach_listeners(app, page);
        
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! router {
    ($($path:pat => $page:ident;)*) => {
        fn route(app: &mut App) -> Page {
            match app.get_href_split().as_slice() {
                $($path => $page::new(app),)*
            }
        }
    };
}

#[macro_export]
macro_rules! storage {
    ($app:ident => {$($var:literal => $val:expr;)*}) => {
        $($app.storage.insert($var, $val);)*
    };
}