use nopulp::*;

pub fn new(app: &mut App) -> Box<Html> {
    storage! {
        app => {
            "clicks" => "0";
        }
    }

    html!(
        <html>
            <span 
                left=10f64 
                top=20f64
            >{"Click Me!".to_string()}</span>
            <span 
                left=10f64 
                top=40f64 
                on_click={|app: &mut App, obj: &mut Object<String>| { 
                    let clicks = app.storage.get("clicks").unwrap_or(&"0");

                    let mut clicks_val = clicks.to_string().parse::<i32>().unwrap_or(0i32);

                    clicks_val += 1i32;

                    let clicks_string = clicks_val.to_string();

                    obj.value = Some(clicks_string.clone());

                    app.storage.insert("clicks", Box::leak(clicks_string.into_boxed_str()));
                }}
            >{"0".to_string()}</span>
            <span
                left=10f64
                top=60f64

                on_click={|app: &mut App, obj: &mut Object<String>| { 
                    if app.storage.get("clicks").unwrap_or(&"0").parse::<i32>().unwrap_or(0i32) > 50i32 {
                        obj.value = Some("Congrats for 50+ clicks!".to_string());
                    }
                }}
            ></span>
        </html>
    )
}