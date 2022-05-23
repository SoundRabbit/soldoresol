mod ast;

pub use ast::Argument;
pub use ast::Command;
pub use ast::Message;
pub use ast::MessageToken;
pub use ast::Reference;
pub use message_parser::*;

peg::parser! {
    grammar message_parser() for str {
        pub rule message() -> Message
            = m_tokens:message_token()* { Message::from(m_tokens).flatten() }

        rule message_token() -> MessageToken
            = precedence! {
                c_block:curly_block() { c_block }
                --
                r"\{" { MessageToken::Text(String::from(r"{")) }
                r"\}" { MessageToken::Text(String::from(r"}")) }
                r"\\" { MessageToken::Text(String::from(r"\")) }
                r"\n" { MessageToken::Text(String::from("\n")) }
                r"\" "\n" { MessageToken::Text(String::from("")) }
                --
                t:$([_]) { MessageToken::Text(String::from(t)) }
            }

        rule curly_block() -> MessageToken
            = precedence! {
                command:command() { MessageToken::Command(command) }
                --
                reference:reference() { MessageToken::Reference(reference) }
            }

        rule command() -> Command
            = precedence! {
                r"{\" name:command_name_with_args() args:curly_block_args() text:curly_block_message() "}" { Command { name, args, text } }
                --
                r"{\" name:command_name_with_none() text:command_text_with_none()? "}"
                    { Command { name, args:vec![], text: text.unwrap_or_else(|| Message::from(vec![])) } }
            }

        rule command_name_with_args() -> Message
            = m_tokens:command_name_token_with_args()* { Message::from(m_tokens) }

        rule command_name_token_with_args() -> MessageToken
            = !"[" m_token:message_token() { m_token }

        rule command_name_with_none() -> Message
            = m_tokens:command_name_token_with_none()* { Message::from(m_tokens) }

        rule command_name_token_with_none() -> MessageToken
            = ![' '|'}'] m_token:message_token() { m_token }

        rule command_text_with_none() -> Message
            = " " msg:curly_block_message() { msg }

        rule reference() -> Reference
            = precedence! {
                "{" name:reference_name_with_args() ** "::" args:curly_block_args() option:reference_option()? "}" { Reference{ name, args, option } }
                --
                "{" name:reference_name_with_none() ** "::" option:reference_option()? "}" { Reference{ name, args: vec![], option } }
            }

        rule reference_name_with_args() -> Message
            = m_tokens:reference_name_token_with_args()* { Message::from(m_tokens) }

        rule reference_name_token_with_args() -> MessageToken
            = !("::"/"[") m_token:message_token() { m_token }

        rule reference_name_with_none() -> Message
            = m_tokens:reference_name_token_with_none()* { Message::from(m_tokens) }

        rule reference_name_token_with_none() -> MessageToken
            = !("::"/"."/"}") m_token:message_token() { m_token }

        rule reference_option() -> Message
            = "." msg:curly_block_message() { msg }

        rule curly_block_args() -> Vec<Argument>
            = "[" args:curly_block_arg() ** "," "]" { args }

        rule curly_block_arg() -> Argument
            = precedence! {
                value:curly_block_arg_value_with_option() "=" option:curly_block_arg_option()
                    { Argument{value: value, option: Some(option)} }
                --
                value:curly_block_arg_value_with_none()
                    { Argument{value: value, option: None} }
            }

        rule curly_block_arg_value_with_option() -> Message
            = m_tokens:curly_block_arg_value_with_option_token()* { Message::from(m_tokens) }

        rule curly_block_arg_value_with_option_token() -> MessageToken
            = !['='|','|']'] m_token:message_token() { m_token }

        rule curly_block_arg_value_with_none() -> Message
            = m_tokens:curly_block_arg_value_with_none_token()* { Message::from(m_tokens) }

        rule curly_block_arg_value_with_none_token() -> MessageToken
            = ![','|']'] m_token:message_token() { m_token }

        rule curly_block_arg_option() -> Message
            = m_tokens:curly_block_arg_option_token()* { Message::from(m_tokens) }

        rule curly_block_arg_option_token() -> MessageToken
            = ![','|']'] m_token:message_token() { m_token }

        rule curly_block_message() -> Message
            = m_tokens:curly_block_message_token()* { Message::from(m_tokens) }

        rule curly_block_message_token() -> MessageToken
            =  !"}" m_token:message_token() { m_token }
    }
}
