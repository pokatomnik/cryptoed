use cursive::{
    Cursive, With,
    view::ViewWrapper,
    views::{CircularFocus, Dialog, TextView},
    wrap_impl,
};

pub(crate) struct ErrorLayer {
    dialog: CircularFocus<Dialog>,
}

impl ErrorLayer {
    pub fn new<F>(error: impl Into<String>, ok: F) -> Self
    where
        F: Fn(&mut Cursive) + Sync + Send + 'static,
    {
        let dialog = Dialog::around(TextView::new(error.into()))
            .title("Oops!")
            .button("OK", ok)
            .wrap_with(CircularFocus::new)
            .wrap_tab();
        Self { dialog: dialog }
    }
}
impl ViewWrapper for ErrorLayer {
    wrap_impl!(self.dialog: CircularFocus<Dialog>);
}
