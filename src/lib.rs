#[macro_use] extern crate rutie;
#[macro_use] extern crate lazy_static;
extern crate liquid;

use rutie::{Object, RString, VM, AnyObject, Module, Hash, Fixnum};

pub struct Template {
    source: String,
    template: liquid::Template,
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

        let source = ruby_string.to_string();
        let template = liquid::ParserBuilder::with_stdlib()
            .build().unwrap()
            .parse(&source).unwrap();

        let obj = Template {
            source: source,
            template: template,
        };

        Module::from_existing("Liquid")
        .get_nested_module("Rust")
        .get_nested_class("Template")
        .wrap_data(obj, &*TEMPLATE_WRAPPER)
    }

    fn pub_source() -> RString {
        let obj = _itself.get_data(&*TEMPLATE_WRAPPER);
        RString::new_utf8(&obj.source)
    }

    fn pub_render(ruby_globals: Hash) -> RString {
        let obj = _itself.get_data(&*TEMPLATE_WRAPPER);

        let mut globals = liquid::model::Object::new();
        ruby_globals.unwrap().each(|key, value| {
            if let Ok(key) = key.try_convert_to::<RString>() {
                if let Ok(value) = value.try_convert_to::<Fixnum>() {
                    globals.insert(key.to_string().into(), liquid::model::to_value(&value.to_i64()).unwrap());
                }
            }
        });

        let result = obj.template.render(&globals).unwrap();

        RString::new_utf8(&result)
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
        itself.def("render", pub_render);
    });
}
