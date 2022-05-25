use super::{
    Argument, BlockMut, Command, CommandResult, GameSystemClass, Message, MessageToken, Property,
    Reference,
};
use std::collections::HashMap;

pub fn map_message(
    props: &Vec<BlockMut<Property>>,
    refs: &mut dyn FnMut(&String) -> Message,
    var_nums: &mut HashMap<String, Vec<usize>>,
    descriptions: &mut Vec<(String, String)>,
    message: Message,
) -> Message {
    message
        .map(|token| map_token(props, refs, var_nums, descriptions, token))
        .flatten()
}

fn map_token(
    props: &Vec<BlockMut<Property>>,
    refs: &mut dyn FnMut(&String) -> Message,
    var_nums: &mut HashMap<String, Vec<usize>>,
    descriptions: &mut Vec<(String, String)>,
    token: MessageToken,
) -> Message {
    match token {
        MessageToken::Text(text) => Message::from(vec![MessageToken::Text(text)]),
        MessageToken::Reference(reference) => {
            map_reference(props, refs, var_nums, descriptions, reference)
        }
        MessageToken::Command(cmd) => map_command(props, refs, var_nums, descriptions, cmd),
    }
}

fn map_command(
    props: &Vec<BlockMut<Property>>,
    refs: &mut dyn FnMut(&String) -> Message,
    var_nums: &mut HashMap<String, Vec<usize>>,
    descriptions: &mut Vec<(String, String)>,
    cmd: Command,
) -> Message {
    let name = map_message(props, refs, var_nums, descriptions, cmd.name);
    let args: Vec<_> = cmd
        .args
        .into_iter()
        .map(|arg| map_argument(props, refs, var_nums, descriptions, arg))
        .collect();

    if name.to_string() == "capture" {
        let mut cap_names = set! {};

        for mut arg in args {
            if arg.option.is_none() && arg.value.len() == 1 {
                if let Some(MessageToken::Command(cap)) = arg.value.pop() {
                    for cap_name in cap.args {
                        let cap_name = cap_name.to_string();
                        descriptions.push((cap.name.to_string(), cap.text.to_string()));
                        let num = descriptions.len();
                        if let Some(vars) = var_nums.get_mut(&cap_name) {
                            vars.push(num);
                        } else {
                            var_nums.insert(cap_name.clone(), vec![num]);
                        }
                        cap_names.insert(cap_name);
                    }
                }
            }
        }

        let text = map_message(props, refs, var_nums, descriptions, cmd.text);

        for cap_name in cap_names {
            if let Some(vars) = var_nums.get_mut(&cap_name) {
                vars.pop();
            }
        }

        text
    } else if name.to_string() == "ref" {
        let cap_name = map_message(props, refs, var_nums, descriptions, cmd.text).to_string();
        let text = if let Some(num) = var_nums.get(&cap_name).and_then(|x| x.last()) {
            Message::from(vec![MessageToken::Text(num.to_string())])
        } else {
            Message::from(vec![MessageToken::Text(cap_name)])
        };
        Message::from(vec![MessageToken::Command(Command { name, args, text })])
    } else {
        let text = map_message(props, refs, var_nums, descriptions, cmd.text);
        Message::from(vec![MessageToken::Command(Command { name, args, text })])
    }
}

fn map_reference(
    props: &Vec<BlockMut<Property>>,
    refs: &mut dyn FnMut(&String) -> Message,
    var_nums: &mut HashMap<String, Vec<usize>>,
    descriptions: &mut Vec<(String, String)>,
    reference: Reference,
) -> Message {
    let text = reference.to_ref_text();
    let Reference { name, args, option } = reference;

    let name = name
        .into_iter()
        .map(|a_name| map_message(props, refs, var_nums, descriptions, a_name).to_string())
        .collect::<Vec<_>>();

    let args = args
        .into_iter()
        .map(|arg| map_argument(props, refs, var_nums, descriptions, arg))
        .map(|arg| {
            (
                arg.value.to_string(),
                arg.option.map(|option| option.to_string()),
            )
        })
        .collect::<Vec<_>>();

    let option =
        option.map(|option| map_message(props, refs, var_nums, descriptions, option).to_string());

    let target = if let Some(value) = props.iter().find_map(|prop| {
        prop.map(|prop| prop.ref_value(name.iter().collect(), args.iter().collect()))
            .unwrap_or_default()
    }) {
        Message::from_str(&value.to_string_with_option(option.as_ref()))
    } else {
        refs(&text)
    };

    let message = map_message(props, refs, var_nums, descriptions, target);

    message
}

fn map_argument(
    props: &Vec<BlockMut<Property>>,
    refs: &mut dyn FnMut(&String) -> Message,
    var_nums: &mut HashMap<String, Vec<usize>>,
    descriptions: &mut Vec<(String, String)>,
    arg: Argument,
) -> Argument {
    if let Some(option) = arg.option {
        Argument {
            value: map_message(props, refs, var_nums, descriptions, arg.value),
            option: Some(map_message(props, refs, var_nums, descriptions, option)),
        }
    } else {
        Argument {
            value: map_message(props, refs, var_nums, descriptions, arg.value),
            option: None,
        }
    }
}
