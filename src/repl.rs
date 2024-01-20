use crate::MetaCommands;

pub fn handle_input(buffer: &String) -> MetaCommands {
    if buffer.contains(".exit") {
        return MetaCommands::META_COMMAND_EXIT
    }

    return MetaCommands::META_COMMAND_CONTINUE
}