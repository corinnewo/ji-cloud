use std::rc::Rc;
use dominator::traits::AsStr;
use futures_signals::signal::Mutable;

pub struct SimpleSelect<T, P, L> {
    pub(super) label: Option<L>,
    pub(super) placeholder: Option<P>,
    pub(super) value: Mutable<Option<T>>,
    pub(super) values: Vec<T>,
    pub(super) on_change: Option<Box<dyn Fn(Option<&str>)>>
}

impl <T, P, L> SimpleSelect <T, P, L> {
    pub fn new(label:Option<L>, placeholder: Option<P>, init_value: Option<T>, values: Vec<T>, on_change: impl Fn(Option<&str>) + 'static) -> Rc<Self> {
        Self::_new(label, placeholder, init_value, values, Some(on_change))
    }

    pub fn new_no_handler(label:Option<L>, placeholder: Option<P>, init_value: Option<T>, values: Vec<T>) -> Rc<Self> {
        Self::_new(label, placeholder, init_value, values, None::<fn(Option<&str>)>)
    }

    fn _new(label:Option<L>, placeholder: Option<P>, init_value: Option<T>, values: Vec<T>, on_change: Option<impl Fn(Option<&str>) + 'static>) -> Rc<Self> {
        Rc::new(Self {
            label,
            placeholder,
            value: Mutable::new(init_value),
            values,
            on_change: on_change.map(|f| Box::new(f) as Box<_>)
        })
    }
}

impl <T: Clone, P, L> SimpleSelect <T, P, L> {
    pub fn get_value(&self) -> Option<T> {
        self.value.get_cloned()
    }
}
