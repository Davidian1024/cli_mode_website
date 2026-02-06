use gloo_console::log;
use serde_json::json;
use web_sys::{
    HtmlInputElement,
    wasm_bindgen::{JsCast, UnwrapThrowExt},
};
use yew::prelude::*;

const SHELL_NAME: &str = "fctcsh";
const COMMAND_PROMPT: &str = "]";

#[component]
fn App() -> Html {
    log!("fn App() start");
    let pwd = use_state(|| "".to_string());
    let file_system = use_state(|| {
        json!(
            {
                "home":
                {
                    "alice":
                    {
                        "notes":
                        [
                            "9/12\nCall Bob about the modem issue.\nRemember to run backup tonight.\n"
                        ],
                        "todo":
                        [
                            "TODO:\n- finish shell script\n- run daily backup\n- update notes file\n"
                        ]
                    },
                    "bob":
                    {
                        "notes":
                        [
                            "Quick reminder:\nCheck .profile changes\nCompile hello.c\nUpdate todo list\n"
                        ],
                        "todo":
                        [
                            "Tasks:\n- fix compile errors in hello.c\n- clean up /tmp\n- check disk usage\n"
                        ]
                    },
                    "carol":
                    {
                        "Notes.txt":
                        [
                            "Notes:\nPrinter jammed again.\nTry lp after lunch.\nDo not forget meeting at 3.\n"
                        ],
                        "todo":
                        [
                            "Daily:\n- read incoming mail\n- call Bob about printer\n- review project plan\n"
                        ]
                    },
                },
                "usr":
                {
                    "bin":
                    {
                        "pwd":
                        [
                            "f0VMRgSbsn2dTwyJvMbUXOZCA/D96wBQD4yx8ByleVDZ7fu51kbGPSHwvgNV575VuFsKKaNY5/QOaAgIkQKvA7Bg/93K+tFFE2MQotyau7MlwfmTcFul6l8SV/BnAHV4gutRHft9Lw135Mx1ghFI01iK6J+3uYzOLqEH6ghnIrAKYzSYW6f66yH3TwWA/EPfolZ2iYV0lWr5cPw7POe9A1aS3c+aKnv3e4MrGd1ZR3X9Ga6KCJPUH7s+cHR6RgJ/8XQzGcPvWe0jCjreO3r50vNVcN9bXK2CB8WhVYWSd+OtBZ3+ZDiRwLwI1aHxT7GS0iCmsMkiCmcfxYFGWNTWKvmISEjDgHk6x52K8oLUT4v0PhZ1nNelk/gPA72OSH1lxCUuEcX4YLiVzLSU7kVbgerN93FfeYEh9qYrw2o4W6fF0xjrs1v9Y"
                        ],
                        "cd":
                        [
                            "f0VMRgoQYN4atnJuuTv+tUEkVSTOFwfWtnSoYH022W6n9av+KdxE9PAgiKOs6Jk2NI3VUrf3u3Rmg+HL4wArYlsSKFGe6Vbkg4/gz8SSdF07+S6PUu3PSScdrobz+NHJzQUmVL/E8yUsUTERduUShtKtwZztkrh3f/K9Mu4AwbTlcv94VIkAJ8/it8B+uEAdhNL4vK35j0yPO5tjmoqKrW8PJIZ4WPTggtw1pGGrCknf5s3zKfcDeGTt/FHwPFfQ84wKsPBIgtEnOfNfsLKWTl45GPjkJtzYVrYB8ygFWvmy1RF9K9BRZ/oOMLeiRYL6koKsj/ILDLZ/yRXEPCUj3K+/TfVMICDupZraRpeynf1agLJn6ci9/MIiBGa4eyCG/pwoicj3aI/Wsk94v6XpCHHEcLbZdSZc2VUd54IgZ/SsaZI2gwFPGzLL17kKouF+TPx1wX6d5PL6xMFcAps5WnP1BKPsa/+nYGUNHMHuqq9ybUMj63GClZOvCEm0relp90STYyEy2/mAM5qcRl0ZpBT2oXBmqAf5/p+nugVqnw/od7AWIGUtqucdxIod3jwjr51wGPG19p36QDeWZ0yNP0LlvFljtxRcqvGqnUoAw3Y5GE8zwJr7XRIpX9g"
                        ],
                        "ls":
                        [
                            "f0VMRg7fu51kbGPSHwvgNV575VuFsKKaNY5/QOaAgIkQKvA7Bg/93K+tFFE2MQotyau7MlwfmTcFul6l8SV/BnAHV4gutRHft9Lw135Mx1ghFI01iK6J+3uYzOLqEH6ghnIrAKYzSYW6f66yH3TwWA/EPfolZ2iYV0lWr5cPw7POe9A1aS3c+aKnv3e4MrGd1ZR3X9Ga6KCJPUH7s+cHR6RgJ/8XQzGcPvWe0jCjreO3r50vNVcN9bXK2CB8WhVYWSd+OtBZ3+ZDiRwLwI1aHxT7GS0iCmsMkiCmcfxYFGWNTWKvmISEjDgHk6x52K8oLUT4v0PhZ1nNelk/gPA72OSH1lxCUuEcX4YLiVzLSU7kVbgerN93FfeYEh9qYrw2o4W6fF0xjrs1v9YrVXGeG4GHh8FGvut77omQvQ3207Xakc0qUL9"
                        ],
                        "cat":
                        [
                            "f0VMRg5ub3RlLkFCSS10YWcALm5vdGUucGFja2FnZQAuaW5pdF9hcnJheQAuZmluaV9hcnJheQAuZGF0YS5yZWwucm8ALmR5bmFtaWMALmdvdAAuZGF0YQAuYnNzAC5nbnUuYnVpbGQuYXR0cmlidXRlcwAuZ251X2RlYnVnbGluawAuZ251X2RlYnVnZGF0YQAAAAAAAAAAA"
                        ],
                    },
                },
                "etc":
                {
                    "passwd":
                    [
                        "This incident will be reported."
                    ],
                    "fstab":
                    [
                        "# /etc/fstab\n#\n# Device          Mount Point      FSType   Mount Flags   Dump  Pass\n#\n/dev/sd0a        /               ufs      rw            1     1\n/dev/sd0b        /usr            ufs      ro            1     2\n/dev/sd0c        /var            ufs      rw            1     2\n/dev/sd0d        /tmp            ufs      rw            0     0\n/dev/sd1a        /home           ufs      rw            1     2\n/dev/sd2a        /local          ufs      rw            1     2\n/dev/fd0         /floppy         msdos    ro,noauto     0     0\n/dev/cd0         /cdrom          iso9660  ro,noauto     0     0\n/dev/tty01       /dev/console    devfs    rw            0     0"
                    ],
                    "hosts":
                    [
                        "# /etc/hosts\n# IP address    Hostname    Alias\n127.0.0.1       localhost   loopback\n192.168.0.1     oldrouter  router\n192.168.0.2     workstation1 ws1\n192.168.0.3     workstation2 ws2\n192.168.0.10    server1     srv1\n192.168.0.11    server2     srv2\n10.0.0.1        gateway     gw\n10.0.0.100      printer     prn1"
                    ],
                },
                "dev":
                {
                    "null":
                    [
                        "c", "1", "3",
                    ],
                    "random":
                    [
                        "c", "1", "8",
                    ],
                    "tty1":
                    [
                        "c", "4", "1",
                    ],
                    "sda":
                    [
                        "b", "8", "0",
                    ],
                },
            }
        )
    });

    let history: UseStateHandle<Vec<String>> = use_state(|| vec![
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "FCTC OS v2026 ready".to_string(),
        "Issue the 'help' command to get a list of commands.".to_string(),
    ]);
    let command = use_state(|| String::new());

    let oninput = {
        let the_oninput_command = command.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            the_oninput_command.set(target.value());
        })
    };

    let cloned_command_value = command.clone();
    let cloned_history_value = history.clone();

    let cloned_pwd_value = pwd.clone();
    let onkeydown = {
        let dereference_cloned_history = (*history).clone();
        Callback::from(move |keyboard_event: KeyboardEvent| {
            let command_str = cloned_command_value.get(..).unwrap_throw();
            if keyboard_event.key() == "Enter" {
                let mut target_directory = "".to_string();
                let mut change_directory = false;
                let mut working_history =
                    Vec::from(dereference_cloned_history.get(..).unwrap_throw());

                scroll_insert(
                    &mut working_history,
                    &vec![format!(
                        "{}{}{}",
                        if *cloned_pwd_value == "" {
                            "/".to_string()
                        } else {
                            (*cloned_pwd_value).clone()
                        },
                        COMMAND_PROMPT,
                        command_str
                    )],
                );

                let command_tokens: Vec<&str> = command_str.split_whitespace().collect();

                if command_tokens.len() > 0 {
                    match command_tokens[0] {
                        "help" => {
                            scroll_insert(&mut working_history, &vec![
                                "Valid Commands:".to_string(),
                                "- pwd: return present working directory.".to_string(),
                                "- cd: change directory.".to_string(),
                                "- ls: list contents of present working directory.".to_string(),
                                "- cat: return contents of a file.".to_string(),
                                ]);
                        }
                        "pwd" => {
                            scroll_insert(
                                &mut working_history,
                                &vec![if *cloned_pwd_value == "" {
                                    format!("pwd: \"{}\"", "/")
                                } else {
                                    format!("pwd: \"{}\"", *cloned_pwd_value)
                                }],
                            );
                        }
                        "cd" => {
                            if command_tokens.len() == 2 {
                                if command_tokens[1] == ".." {
                                    scroll_insert(&mut working_history, &vec!["todo: implement '..'".to_string()]);
                                } else {
                                    let path_tokens: Vec<&str> = command_tokens[1].split('/').collect();
                                    target_directory = (*cloned_pwd_value).clone();
                                    for (index, path_token) in path_tokens.iter().enumerate() {
                                        match path_token {
                                            &"" => {
                                                if index == 0 {
                                                    target_directory = "".to_string();
                                                }
                                            }
                                            _ => {
                                                target_directory = target_directory + "/" + path_token;
                                            }
                                        }
                                    }
                                    if let Some(cd_pointer) =
                                        (*file_system).clone().pointer(&target_directory)
                                    {
                                        match cd_pointer {
                                            serde_json::Value::Object(_o) => {
                                                change_directory = true;
                                            }
                                            _ => {
                                                scroll_insert(
                                                    &mut working_history,
                                                    &vec![format!(
                                                        "-{}: {}: not a directory: {}",
                                                        SHELL_NAME, command_tokens[0], target_directory
                                                    )],
                                                );
                                            }
                                        }
                                    } else {
                                        scroll_insert(&mut working_history, &vec![format!(
                                            "target_directory: {}; invalid",
                                            target_directory
                                        )]);
                                    }
                                }
                            } else {
                                scroll_insert(&mut working_history, &vec![format!(
                                    "-{}: {}: incorrect number of arguments",
                                    SHELL_NAME, command_tokens[0]
                                )]);
                            }
                        }
                        "ls" => {
                            if let Some(ls_pointer) =
                                (*file_system).clone().pointer((*cloned_pwd_value).as_str())
                            {
                                match ls_pointer {
                                    serde_json::Value::Null => {
                                        scroll_insert(&mut working_history, &vec![format!("NULL")]);
                                    }
                                    serde_json::Value::Bool(b) => {
                                        scroll_insert(&mut working_history, &vec![format!("Bool: {}", b)]);
                                    }
                                    serde_json::Value::Number(number) => {
                                        scroll_insert(&mut working_history, &vec![format!("Number: {}", number)]);
                                    }
                                    serde_json::Value::String(s) => {
                                        scroll_insert(&mut working_history, &vec![format!("String: {}", s)]);
                                    }
                                    serde_json::Value::Array(values) => {
                                        scroll_insert(&mut working_history, &vec![format!("Array: {:#?}", values)]);
                                    }
                                    serde_json::Value::Object(map) => {
                                        for (k, _v) in map {
                                            scroll_insert(&mut working_history, &vec![format!("{}", k)]);
                                        }
                                    }
                                }
                            } else {
                                scroll_insert(&mut working_history, &vec![format!(
                                    "-{}: {}: present working directory is invalid",
                                    SHELL_NAME, command_tokens[0]
                                )]);
                            }
                        }
                        "cat" => {
                            if command_tokens.len() == 2 {
                                let cat_target_path = format!("{}/{}", *cloned_pwd_value, command_tokens[1]);
                                let foo = (*file_system).clone();
                                if let Some(bar) = foo.pointer(&cat_target_path) {
                                    match bar {
                                        serde_json::Value::Array(values) => {
                                            for each_value in values {
                                                if let Some(baz) = each_value.as_str() {
                                                    let mut qux = vec![];
                                                    for each in baz.split("\n") {
                                                        qux.push(each.to_string());
                                                    }
                                                    scroll_insert(&mut working_history, &qux);
                                                }
                                            }
                                        },
                                        _ => {},
                                    }
                                }

                            } else {
                                scroll_insert(&mut working_history, &vec![format!(
                                    "-{}: {}: incorrect number of arguments",
                                    SHELL_NAME, command_tokens[0]
                                )]);
                            }
                        }
                        _ => {
                            scroll_insert(&mut working_history, &vec![format!(
                                "-{}: {}: command not found",
                                SHELL_NAME, command_tokens[0]
                            )]);
                        }
                    }
                }

                cloned_history_value.set(working_history);
                cloned_command_value.set("".to_string());
                if change_directory {
                    cloned_pwd_value.set(target_directory.clone());
                }
            };
        })
    };

    log!("fn App() returning...");
    html! {
        <>
            <div style="width: 100%; background-image: url('static/images/library.png'); background-size: 100% 100%; padding: 3rem; width: 109rem; height: 65rem; ">
                // monitor div
                <div style="margin: 0 auto; background-image: url('static/images/monitor.png'); background-size: 100% 100%; padding-top: 7rem; padding-bottom: 6rem; padding-left:10rem; padding-right:11rem; width: 80rem; height: 40rem; border-radius: 1rem; box-shadow: 4px 4px 10px rgba(0,0,0,0.5);">

                    // terminal area
                    <div style="padding: 0.1rem; background-color: black; height: 100%;">

                        // terminal history lines
                        { for history.iter().map(|line| html!{<div class="terminal" style="border: solid 0.1rem black; height: 2.2rem; padding: 0.1rem; font-family: VT323, monospace; font-size: 2rem">{line}</div>}) }

                        // terminal prompt line
                        <div class="terminal" style="display: flex; flex-direction: row; align-items: center; white-space: nowrap; height: 2.2rem; font-family: VT323, monospace; font-size: 2rem">
                            { if (*pwd).clone() == "" { "/".to_string() } else { (*pwd).clone() } }
                            {COMMAND_PROMPT}<input class="terminal" style="width: 100%; border: none; outline: none; font-size: 2rem; font-family: VT323, monospace;" value={(*command).clone()} {oninput} {onkeydown} />
                        </div>

                    </div>

                </div>
            </div>
        </>
    }
}

fn scroll_insert(history: &mut Vec<std::string::String>, new_text: &Vec<String>) {
    for line in new_text {
        history.remove(0);
        let mut truncated_line: String = "".to_string();
        for (index, char) in line.chars().enumerate() {
            if index > 96 { break }
            truncated_line.push(char);
        }
        history.push(truncated_line.to_string());
    }
}

fn main() {
    log!("Initializing cli_mode_website...");
    yew::Renderer::<App>::new().render();
}
