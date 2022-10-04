use shared::{self, Result};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event, HtmlInputElement};

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let callback = Closure::wrap(Box::new(|e: Event| {
        e.prevent_default();

        let left = get_input_element_by_id("client-left").unwrap();
        let right = get_input_element_by_id("client-right").unwrap();
        let res = get_input_element_by_id("client-res").unwrap();
        res.set_value_as_number(shared::add(left.value_as_number(), right.value_as_number()))
    }) as Box<dyn FnMut(_)>);

    let form = get_element_by_id("client-form").unwrap();
    form.add_event_listener_with_callback("submit", callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}

fn get_input_element_by_id(element_id: &str) -> Result<HtmlInputElement> {
    let element = get_element_by_id(element_id)?;
    let element = element
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| "element is not input")?;
    Ok(element)
}

fn get_element_by_id(element_id: &str) -> Result<Element> {
    let window = web_sys::window().ok_or("window is not found")?;
    let document = window.document().ok_or("document is not found")?;
    let element = document
        .get_element_by_id(element_id)
        .ok_or("element is not found")?;
    Ok(element)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = shared::add(2., 2.);
        assert_eq!(result, 4.);
    }
}
