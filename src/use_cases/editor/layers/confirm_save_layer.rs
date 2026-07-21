use cursive::{
    Cursive, With,
    view::ViewWrapper,
    views::{CircularFocus, Dialog, TextView},
    wrap_impl,
};

pub(crate) struct ConfirmSaveLayer {
    dialog: CircularFocus<Dialog>,
}

impl ConfirmSaveLayer {
    pub fn new<FYes, FNo>(on_yes: FYes, on_no: FNo) -> Self
    where
        FYes: Fn(&mut Cursive) + Sync + Send + 'static,
        FNo: Fn(&mut Cursive) + Sync + Send + 'static,
    {
        let dialog = Dialog::around(TextView::new("Would you like to save changes?"))
            .title("Save")
            .button("Yes", on_yes)
            .button("No", on_no)
            .wrap_with(CircularFocus::new)
            .wrap_tab();
        Self { dialog: dialog }
    }
}
impl ViewWrapper for ConfirmSaveLayer {
    wrap_impl!(self.dialog: CircularFocus<Dialog>);
}
