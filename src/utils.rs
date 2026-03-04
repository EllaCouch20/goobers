use prism::Context;
use crate::theme::Theme;

pub trait Callback: FnMut(&mut Context, &Theme) + 'static {
    fn clone_box(&self) -> Box<dyn Callback>;
}

impl PartialEq for dyn Callback{fn eq(&self, _: &Self) -> bool {true}}

impl<F> Callback for F where F: FnMut(&mut Context, &Theme) + Clone + 'static {
    fn clone_box(&self) -> Box<dyn Callback> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Callback> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn Callback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clonable Closure")
    }
}