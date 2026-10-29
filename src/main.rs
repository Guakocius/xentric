use eframe::{Storage, egui};
use std::io;
use xmpp::{ClientBuilder, jid::BareJid};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Xentric",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<XentricApp>::default())
        }),
    )
}

struct XentricApp {
    user: User,
}

impl Default for XentricApp {
    fn default() -> Self {
        Self {
            user: User::default(),
        }
    }
}

struct Channel {
    name: String,
}

struct User {
    name: String,
    password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: read_user_jid().to_owned(),
            password: read_user_password().to_owned(),
        }
    }
}

struct ChannelList {
    channels: Vec<Channel>,
    //storage: Storage,
}

fn read_user_jid() -> String {
    println!("Please enter your username in the format of <node@domain>:");
    let mut jid = String::new();
    io::stdin().read_line(&mut jid).expect("failed to readline");
    jid.trim().to_string()
}

fn read_user_password() -> String {
    println!("Please enter your password:");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("failed to readline");
    password.trim().to_string()
}

fn build_client(jid: &str, password: &str) -> anyhow::Result<()> {
    let jid = BareJid::new(jid)?;
    let client_builder = ClientBuilder::new(jid, password);
    Ok(())
}

impl eframe::App for XentricApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Xentric");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Channel list");
                ui.set_max_width(50.0);
            });
            ui.horizontal(|ui| {
                let name = &self.user.name;
                ui.label(format!("Your name: {name}"));
            });
        });
    }
}
