use clap::Parser;
use dialoguer::Confirm;
use std::fs;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};

use crate::script_loader::read_toml;

pub mod script_loader;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.name);

    read_toml();
    // install_brew();
}

fn install_brew() {
    println!("Bienvenue dans le script d'installation de Homebrew.");

    // Confirmer si l'utilisateur veut continuer
    let confirmation = Confirm::new()
        .with_prompt("Voulez-vous installer Homebrew ?")
        .default(false)
        .interact()
        .unwrap();

    if !confirmation {
        println!("Installation annulée.");
        return;
    }

    // Exécuter le script d'installation de Homebrew
    println!("Téléchargement et installation de Homebrew...");
    // Crée un dossier "scripts" s'il n'existe pas déjà
    fs::create_dir_all("scripts").expect("Impossible de créer le dossier scripts");

    let output = Command::new("curl")
        .arg("-fsSL")
        .arg("-o")
        .arg("scripts/install.sh") // Enregistre le script téléchargé dans le dossier "scripts"
        .arg("https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match output {
        Ok(mut child) => {
            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();

            let stdout_thread = std::thread::spawn(move || {
                let stdout_reader = io::BufReader::new(stdout);
                for line in stdout_reader.lines() {
                    println!("BREW SCRIPT: {}", line.unwrap());
                }
            });

            let stderr_thread = std::thread::spawn(move || {
                let stderr_reader = io::BufReader::new(stderr);
                for line in stderr_reader.lines() {
                    println!("BREW SCRIPT ERROR: {}", line.unwrap());
                }
            });

            let _ = stdout_thread.join();
            let _ = stderr_thread.join();

            let status = child
                .wait()
                .expect("Échec du téléchargement du script d'installation de Homebrew");
            if status.success() {
                println!("Script téléchargé avec succès.");
                // Exécute le script via bash
                let output = Command::new("bash")
                    .arg("scripts/install.sh")
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn();

                match output {
                    Ok(mut child) => {
                        let stdout = child.stdout.take().unwrap();
                        let stderr = child.stderr.take().unwrap();

                        let stdout_thread = std::thread::spawn(move || {
                            let stdout_reader = io::BufReader::new(stdout);
                            for line in stdout_reader.lines() {
                                println!("BREW: {}", line.unwrap());
                            }
                        });

                        let stderr_thread = std::thread::spawn(move || {
                            let stderr_reader = io::BufReader::new(stderr);
                            for line in stderr_reader.lines() {
                                println!("BREW ERROR: {}", line.unwrap());
                            }
                        });

                        let _ = stdout_thread.join();
                        let _ = stderr_thread.join();

                        let status = child
                            .wait()
                            .expect("Échec de l'exécution du script d'installation de Homebrew");
                        if status.success() {
                            println!("Homebrew a été installé avec succès.");
                        } else {
                            println!("Erreur lors de l'installation de Homebrew.");
                        }
                    }
                    Err(e) => {
                        println!("Erreur lors de l'exécution du processus : {}", e);
                    }
                }
            } else {
                return println!("Erreur lors du téléchargement du script.");
            }
        }
        Err(e) => {
            println!("Erreur lors du téléchargement du script : {}", e);
        }
    }
}
