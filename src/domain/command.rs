use domain::wheels_command::WheelsCommand;
use protos::command::Global_Commands;
use protos::command::Robot_Command;
use protobuf::RepeatedField;
use rand::{thread_rng, Rng};
use domain::constants::{MIN_COMMAND_SIZE, MAX_COMMAND_SIZE};

#[derive(Clone, Debug)]
pub struct Command {
    pub commands: Vec<WheelsCommand>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            commands: Vec::new()
        }
    }

    pub fn new_with(commands: Vec<WheelsCommand>) -> Self {
        Self {
            commands
        }
    }

    pub fn new_random() -> Self {
        Self {
            commands: (0..thread_rng().gen_range(MIN_COMMAND_SIZE, MAX_COMMAND_SIZE))
                .map(|_| {
                    WheelsCommand::new_random()
                })
                .collect()
        }
    }

    pub fn is_zero(&self) -> bool {
        self.commands.len() == 0
    }
}

impl From<Command> for Global_Commands {
    fn from(command: Command) -> Self {
        let mut _self = Global_Commands::new();

        let robots_command = command.commands
            .iter()
            .map(|x| Robot_Command::from(x.clone()))
            .collect();

        _self.set_robot_commands(RepeatedField::from_vec(robots_command));

        _self
    }
}
