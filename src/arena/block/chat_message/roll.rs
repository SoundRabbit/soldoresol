use super::{Argument, Command, CommandResult, GameSystemClass, Message, MessageToken, Reference};

pub fn roll_message(
    game_system_class: &GameSystemClass,
    command_results: &mut Vec<CommandResult>,
    message: &Message,
) {
    for token in message.iter() {
        roll_token(game_system_class, command_results, token);
    }
}

fn roll_token(
    game_system_class: &GameSystemClass,
    command_results: &mut Vec<CommandResult>,
    token: &MessageToken,
) {
    match token {
        MessageToken::Text(text) => roll_text(game_system_class, command_results, text),
        MessageToken::Reference(reference) => {
            roll_reference(game_system_class, command_results, reference)
        }
        MessageToken::Command(command) => roll_command(game_system_class, command_results, command),
    }
}

fn roll_text(
    game_system_class: &GameSystemClass,
    command_results: &mut Vec<CommandResult>,
    text: &String,
) {
    if let Some(cmd_result) = game_system_class.eval(&text) {
        command_results.push(cmd_result);
    }
}

fn roll_command(
    game_system_class: &GameSystemClass,
    command_results: &mut Vec<CommandResult>,
    cmd: &Command,
) {
    roll_message(game_system_class, command_results, &cmd.text);
}

fn roll_reference(
    _game_system_class: &GameSystemClass,
    _command_results: &mut Vec<CommandResult>,
    _reference: &Reference,
) {
}

fn roll_argument(
    _game_system_class: &GameSystemClass,
    _command_results: &mut Vec<CommandResult>,
    _argument: &Argument,
) {
}
