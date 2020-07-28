use nopulp_core::*;
use nopulp_components::*;
use nopulp_html::*;

#[test]
fn empty_html_tags_should_result_in_default_html() {
    assert_eq!(*html!(<Html></Html>), Html::default());
}

#[test]
fn nested_tags_should_be_child_of_parent() {
    let html: Html = *html!(<Html><Button></Button></Html>);

    let child_clone = html.0.children[0].clone();

    let button2: Button = Button::default();

    assert_eq!(child_clone.downcast::<Button>().expect("First element should be button").as_ref(), &button2);
}

#[test]
fn nested_tags_should_follow_insertion_order() {
    let value = 30f64;

    let html: Html = *html!(<Html><Button left=10f64/><Button left={value}/></Html>);

    let child_clone = html.0.children[1].clone();

    let mut button2: Button = Button::default();

    button2.0.style.left = value;

    assert_eq!(child_clone.downcast::<Button>().expect("Second element should be button").as_ref(), &button2);
}

#[test]
fn should_be_able_to_deeply_nest() {
    let value = 30f64;

    let html: Html = *html!(<Html><Div><Button left={value}/></Div></Html>);

    let div_clone = html.0.children[0].clone().downcast::<Div>().unwrap();

    let div: &Div = div_clone.as_ref();

    let button_clone = div.0.children[0].clone().downcast::<Button>().unwrap();

    let button: &Button = button_clone.as_ref();

    let mut button2: Button = Button::default();

    button2.0.style.left = value;

    assert_eq!(button, &button2);
}

#[test]
fn literal_variables_should_be_attached_to_object() {
    let html: Html = *html!(<Html left=200f64/>);

    assert_eq!(html.0.style.left, 200f64);
}

#[test]
fn expression_should_be_attached_to_object() {
    let html: Html = *html!(<Html top={10f64 + 20f64}/>);

    assert_eq!(html.0.style.top, 30f64);
}