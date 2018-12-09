use keyboard::*;
use core::fmt::Debug;
use core::str;
mod pshell;
pub use self::pshell::PShell;

pub const MAX_CMD_LEN: u8 = 100 as u8;

const EMPTY_CMD: [u8; MAX_CMD_LEN as usize] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                                        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                                        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                                        0,0,0,0,0,0,0,0,0];

pub const EXIT_FLAG: u8 = 1;


pub struct ShellManager<'a, T: Shell> {
    current_command: [u8; MAX_CMD_LEN as usize],
    current_cmd_len: u8,
    shell: &'a mut T,
}

impl<'a, T: Shell> ShellManager<'a, T> {
    pub fn new(shell: &'a mut T) -> ShellManager<'a, T> {
        let mut new_shell_manager = ShellManager {
            current_command: EMPTY_CMD,
            current_cmd_len: 0 as u8,
            shell: shell,
        };
        new_shell_manager.new_shell_line(T::OK);
        new_shell_manager
    }

    pub fn update(&mut self, key_in: KeyHandlerOut) -> u8 {
        let mut ret_flags: u8 = 0;
        if key_in.clone().get_flag(KEY_ENTER){
            println!("");
            let command_result = self.shell.exec_command(&str::from_utf8(&self.current_command[0..(self.current_cmd_len as usize)]).expect("String conversion error"));
            if command_result == T::EXIT_OS {
                ret_flags |= EXIT_FLAG;
            }else{
                self.new_shell_line(command_result);
                self.current_command = EMPTY_CMD;
                self.current_cmd_len = 0;
            }
        }else{
            let stdin = key_in.get_stdin();
            if stdin == '\x08' {
                if self.current_cmd_len > 0{
                    self.current_cmd_len -= 1;
                    self.current_command[self.current_cmd_len as usize] = 0;
                    print!("{}", stdin);
                }
            }else if self.current_cmd_len + 1 < MAX_CMD_LEN {
                self.current_command[self.current_cmd_len as usize] = stdin as u8;
                print!("{}", stdin);
                self.current_cmd_len += 1;
            }else{
                self.current_cmd_len = 0;
            }
        }
        ret_flags
    }

    fn new_shell_line(&mut self, result: usize) {
        self.shell.print_shell_start(result);
    }
}

pub trait Shell {
    const EXIT_OS: usize = 0;
    const OK: usize = 1;
    fn exec_command(&mut self, command: &str) -> usize;
    fn print_shell_start(&mut self, last_command_out: usize) {
        if last_command_out != Self::OK {
            println!("------------------------------\nReturned code {}", last_command_out);
        }
        print!(" > ");
    }
}
