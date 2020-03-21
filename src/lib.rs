#[macro_use] extern crate rutie;
#[macro_use] extern crate lazy_static;
extern crate liquid;
extern crate kstring;

use rutie::{Object, RString, VM, AnyObject, Module, Hash, Fixnum};
use liquid::model::{ObjectView, ValueView};
use liquid::model::value::{DisplayCow, ValueCow};
use liquid::model::scalar::{ScalarCow};
use kstring::KStringCow;
use std::iter::empty;
use std::fmt;

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

    fn pub_render(ruby_globals: AnyObject) -> RString {
        let obj = _itself.get_data(&*TEMPLATE_WRAPPER);

        let result = obj.template.render(&RubyObject(ruby_globals.unwrap())).unwrap();

        RString::new_utf8(&result)
    }
);

#[derive(Debug)]
struct RubyObject(AnyObject);

impl ValueView for RubyObject {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn render(&self) -> DisplayCow<'_> {
        unimplemented!()
    }
    fn source(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new("<source>"))
    }
    fn type_name(&self) -> &'static str {
        "object"
    }
    fn query_state(&self, state: liquid::model::State) -> bool {
        unimplemented!()
    }

    fn to_kstr(&self) -> KStringCow<'_> {
        unimplemented!()
    }
    fn to_value(&self) -> liquid::model::Value {
        unimplemented!()
    }

    fn as_object(&self) -> Option<&dyn ObjectView> {
        Some(self)
    }

    fn as_scalar(&self) -> Option<ScalarCow<'_>> {
        if let Ok(value) = self.0.try_convert_to::<Fixnum>() {
            Some(liquid::model::scalar::to_scalar(&value.to_i64()).unwrap())
        } else {
            None
        }
    }
}

impl ObjectView for RubyObject {
    fn as_value(&self) -> &dyn ValueView {
        self
    }

    fn size(&self) -> i32 {
        0
    }

    fn keys<'k>(&'k self) -> Box<dyn Iterator<Item = KStringCow<'k>> + 'k> {
        println!("keys");
        Box::new(empty())
    }

    fn values<'k>(&'k self) -> Box<dyn Iterator<Item = &'k dyn ValueView> + 'k> {
        Box::new(empty())
    }

    fn iter<'k>(&'k self) -> Box<dyn Iterator<Item = (KStringCow<'k>, &'k dyn ValueView)> + 'k> {
        println!("iter");
        Box::new(empty())
    }

    fn contains_key(&self, index: &str) -> bool {
        let result = self.0.protect_send("has_key?", &[RString::new_utf8(index).to_any_object()]).unwrap().is_true();
        println!("contains_key({}) -> {}", index, result);
        result
    }

    fn get<'k>(&'k self, index: &str) -> Option<&'k dyn ValueView> {
        println!("get {}", index);

        // FIXME: This is wrong!
        // The API doesn't allow us to return an owned `dyn ValueView` here. (Ideally we'd like
        // ValueCow).
        // Without changing it, we can either cache property lookups inside this object, or leak
        // memory.
        Some(Box::leak(Box::new(RubyObject(self.0.protect_send("[]", &[RString::new_utf8(index).to_any_object()]).unwrap()))))
    }
}

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
