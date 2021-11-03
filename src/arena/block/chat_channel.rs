uses! {
    super::ChatMessage;
    super::util::Pack;
}

block! {
    [pub ChatChannel]
    messages: Vec<U128Id> = vec![];
}
