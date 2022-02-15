use super::{Message, MessageCommand, MessageToken};
use std::collections::HashMap;

pub fn map_message(
    refs: &mut dyn FnMut(&String) -> Message,
    var_nums: &mut HashMap<String, Vec<usize>>,
    descriptions: &mut Vec<(String, String)>,
    message: Message,
) -> Message {
    message
        .map(|token| map_token(refs, var_nums, descriptions, token))
        .compress()
}

fn map_token(
    refs: &mut dyn FnMut(&String) -> Message,
    var_nums: &mut HashMap<String, Vec<usize>>,
    descriptions: &mut Vec<(String, String)>,
    token: MessageToken,
) -> Message {
    match token {
        MessageToken::Text(text) => Message::from(vec![MessageToken::Text(text)]),
        MessageToken::Refer(refer) => {
            let refer = map_message(refs, var_nums, descriptions, refer);
            let refer = refs(&refer.to_string());
            let message = map_message(refs, var_nums, descriptions, refer);
            message
        }
        MessageToken::CommandBlock(cmd, text) => {
            let cmd_name = map_message(refs, var_nums, descriptions, cmd.name);
            let cmd_args: Vec<_> = cmd
                .args
                .into_iter()
                .map(|x| map_message(refs, var_nums, descriptions, x))
                .collect();

            if cmd_name.to_string() == "capture" {
                let mut cap_names = vec![];

                for args in cmd_args {
                    let args: Vec<_> = args.into();
                    for arg in args {
                        if let MessageToken::CommandBlock(cap, desc) = arg {
                            for cap_name in cap.args {
                                let cap_name = cap_name.to_string();
                                descriptions.push((cap.name.to_string(), desc.to_string()));
                                let num = descriptions.len();
                                if let Some(vars) = var_nums.get_mut(&cap_name) {
                                    vars.push(num);
                                } else {
                                    var_nums.insert(cap_name.clone(), vec![num]);
                                }
                                cap_names.push(cap_name);
                            }
                        }
                    }
                }

                let text = map_message(refs, var_nums, descriptions, text);

                for cap_name in cap_names {
                    if let Some(vars) = var_nums.get_mut(&cap_name) {
                        vars.pop();
                    }
                }

                text
            } else if cmd_name.to_string() == "ref" {
                let cap_name = map_message(refs, var_nums, descriptions, text).to_string();
                let text = if let Some(num) = var_nums.get(&cap_name).and_then(|x| x.last()) {
                    Message::from(vec![MessageToken::Text(num.to_string())])
                } else {
                    Message::from(vec![MessageToken::Text(cap_name)])
                };
                Message::from(vec![MessageToken::CommandBlock(
                    MessageCommand {
                        name: cmd_name,
                        args: cmd_args,
                    },
                    text,
                )])
            } else {
                let text = map_message(refs, var_nums, descriptions, text);
                Message::from(vec![MessageToken::CommandBlock(
                    MessageCommand {
                        name: cmd_name,
                        args: cmd_args,
                    },
                    text,
                )])
            }
        }
    }
}
