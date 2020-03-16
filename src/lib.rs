#[macro_use] extern crate rutie;
#[macro_use] extern crate lazy_static;

use rutie::{Class, Object, RString, VM, AnyObject, Module};

pub struct Template {
    source: String,
}

wrappable_struct!(Template, TemplateWrapper, TEMPLATE_WRAPPER);

class!(RubyTemplate);

methods!(
    RubyTemplate,
    _itself,

    fn pub_parse(input: RString) -> AnyObject {
        let ruby_string = input.
          map_err(|e| VM::raise_ex(e) ).
          unwrap();

        let template = Template {
            source: ruby_string.to_string()
        };

        Module::from_existing("Liquid")
        .get_nested_module("Rust")
        .get_nested_class("Template")
        .wrap_data(template, &*TEMPLATE_WRAPPER)
    }

    fn pub_source() -> RString {
        let template = _itself.get_data(&*TEMPLATE_WRAPPER);
        RString::new_utf8(&template.source)
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_liquid_rust() {
    Module::from_existing("Liquid")
    .get_nested_module("Rust")
    .define_nested_class("Template", None)
    .define(|itself| {
        itself.def_self("parse", pub_parse);
        itself.def("source", pub_source);
    });
}
