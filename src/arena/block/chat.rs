uses! {
    super::ChatChannel;
    super::util::Pack;
}

block! {
    [pub Chat]
    channels: Vec<Rc<RefCell<ChatChannel>>> = vec![];
}
