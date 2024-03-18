use ansi_term::Colour;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("{}", Colour::Blue.paint("command -> "));
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Giriş başarısız.");
        
        let choices: Vec<&str> = user_input.trim().split_whitespace().collect();
        
        match choices.get(0) {
            Some(&cmd) => {
                match cmd {
                    "help" => help(),
                    "ls" => {
                        if choices.len() > 1 && choices[1] == "-l" {
                            ls(true, None);
                        } else if choices.len() > 1 {
                            println!("{}", Colour::Red.paint("ls komutu dosya veya dizin adı alamaz."));
                        } else {
                            ls(false, None);
                        }
                    },
                    "cat" => {
                        let files: Vec<&str> = choices.iter().skip(1).map(|&x| x).collect();
                        cat(files);
                    },
                    "clear" => clear(),
                    _ => {
                        println!("{}", Colour::Red.paint(format!("'{}' komutu bulunamadı, lütfen 'help' komutunu kullanın.", cmd)));
                    }
                }
            },
            None => println!("{}", Colour::Red.paint("Geçerli bir komut girilmedi.")),
        }
    }

}

fn help() {
    println!("Kullanılabilir komutlar:");
     println!("ls - Belirtilen dizindeki dosya ve dizinleri listeler.");
     println!("ls -l - Dosya izinleri ile birlikte dosya ve dizinleri listeler (uzun form).");

}

fn clear() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .output()
                .expect("Komut çalıştırılamadı.")
    } else {
        Command::new("clear")
                .output()
                .expect("Komut çalıştırılamadı.")
    };

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn ls(long_format: bool, path: Option<&str>) {
    let mut command = Command::new("ls");
    
    if long_format {
        command.arg("-l");
    }

    if let Some(p) = path {
        command.arg(p);
    }

    let output = command.output().expect("Process başlatılamadı.");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        eprintln!("{}", Colour::Red.paint("ls komutu çalıştırılamadı."));
    }
}


fn cat(files: Vec<&str>) {
    for file in files {
        if file.contains("/") {
            eprintln!("Geçerli bir dosya adı değil: {}", file);
            continue;
        }

        let output = std::process::Command::new("cat")
            .arg(file)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    std::io::stdout().write_all(&output.stdout).unwrap();
                } else {
                    eprintln!("{} dosyası okunamadı. Hata: {}", file, String::from_utf8_lossy(&output.stderr));
                }
            },
            Err(err) => {
                eprintln!("{} dosyası okunamadı. Hata: {}", file, err);
            }
        }
    }
}