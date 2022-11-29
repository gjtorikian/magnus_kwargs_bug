extern crate core;

use magnus::{define_module, function, scan_args, Error, Module, Object, Value};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "MagnusKwargsBug::Selector")]
pub struct MagnusKwargsBugSelector {
    match_element: Option<String>,
    match_text_within: Option<String>,
}

impl MagnusKwargsBugSelector {
    fn new(args: &[Value]) -> Result<Self, Error> {
        let (match_element, match_text_within) = Self::scan_parse_args(args)?;

        Ok(Self {
            match_element,
            match_text_within,
        })
    }

    #[allow(clippy::let_unit_value)]
    fn scan_parse_args(args: &[Value]) -> Result<(Option<String>, Option<String>), Error> {
        let args = scan_args::scan_args(args)?;
        let _: () = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let _: () = args.trailing;
        let _: () = args.block;

        let kw = scan_args::get_kwargs::<_, (), (Option<String>, Option<String>), ()>(
            args.keywords,
            &[],
            &["match_element", "match_text_within"],
        )?;
        let (match_element, match_text_within) = kw.optional;

        Ok((match_element, match_text_within))
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let mkb = define_module("MagnusKwargsBug").expect("cannot define ::MagnusKwargsBug module");

    let c_selector = mkb
        .define_class("Selector", Default::default())
        .expect("cannot define class MagnusKwargsBug::Selector");

    c_selector.define_singleton_method("new", function!(MagnusKwargsBugSelector::new, -1))?;

    Ok(())
}
