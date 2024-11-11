use std::default;

use egui::{Ui,Image,Vec2,include_image,RichText};
use image::EncodableLayout;
use crate::app::OpenWindows;

use super::{clickable_icon,list,inline_text,text_list_item,ListItem,UiList};

use sha2::{Digest,Sha256};

#[derive(Default)]
pub struct LearnData {
    hash_activity: String,
    hash_activity_result: String,
    salted_hash_activity: String,
    salt_value: String,
    salted_hash_activity_result: String,
}

pub fn learn_button(ui: &mut Ui,open_windows:&mut OpenWindows) {
    let hat: Image<'_> = Image::new(include_image!("../../assets/learnIcon.png"))
                    .fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });
    clickable_icon(ui,"Learn",&hat,||{
        open_windows.learning_page = !open_windows.learning_page;
    });
}

pub fn learn_window(ui: &mut Ui,learn_data:&mut LearnData,open_windows: &mut OpenWindows,){
    let window = egui::Window::new("Learn")
    .default_size(Vec2 { x: 450.0, y: 500.0 })
    .min_size(Vec2 { x: 450.0, y: 500.0 })
    .max_width(600.0)
    .scroll(true)
    .resizable(true);
    window.open(&mut open_windows.learning_page)
    .show(ui.ctx(),|ui|{
        ui.label("The following sections will each take you through security issues associated with each title.");
        ui.collapsing("Introduction to cybersecurity", |ui| {
            ui.label("Cybersecurity is the practice of protecting computer networks and devices from being accessed or modified by unauthorised organisations or individuals.");
            ui.label(RichText::new("Why is it important?").strong());
            ui.label("Cybersecurity is incredibly important in the modern world because so much of what we do relies on computers and the data stored within them. It protects our banks, our hospitals and our internet presence.");
        });
        ui.collapsing("Encryption", |ui| {
            ui.label(RichText::new("What is it?").strong());
            ui.label("Encryption is a practice through which communication between 2 or more parties can be kept secret from 3rd parties who might be able to intercept it.");
            ui.label(RichText::new("Why is it important?").strong());
            ui.label("Encryption is a fundamental part of the internet and communication as a whole because it allows us to communicate with other parties without being eavesdropped even when the messages are travelling through systems used by and monitored by other people.");
            ui.collapsing("RSA", |ui| {
                ui.label(RichText::new("What is it?").strong());
                ui.label("RSA encryption is a special form of encryption called asymmetric encryption.");
                ui.label("This means that the sender and the receiver have a different key.");
                ui.label(RichText::new("How does it work?").strong());
                ui.label("RSA encryption works by using the fact that it is very difficult to find prime factors of very large numbers.");
                ui.hyperlink_to("RSA", "https://en.wikipedia.org/wiki/RSA_(cryptosystem)");
            });
            ui.collapsing("One time pad", |ui| {
                ui.label(RichText::new("What is it?").strong());
                ui.label("One time pad encryption is a method of symmetric encryption that uses a random string of numbers to disguise a message. This \"key\" is used by both the sender and receiver to encrypt and decrypt the message.");
                ui.label(RichText::new("How does it work?").strong());
                ui.label("A one time pad uses the fact that when applying operations like XOR to a random sequence of valeus and any other sequence of values like a message the result is itself random. Kind of like mixing them.");
                ui.label("So we can use reversible operations to \"Mix\" the random values and message together to encrypt the message and then the receiver uses the same operation to \"un-Mix\" them.");
                ui.label(RichText::new("What's the big deal?").strong());
                ui.label("Unlike encryption like RSA one time pads are unbreakable if you don't have the key. There is simply no way to extract the message back from the seemingly random sequence of encrypted values without it.");
                ui.hyperlink_to("One time pad", "https://en.wikipedia.org/wiki/One-time_pad");
            });
        });
        ui.collapsing("Password security", |ui| {
            ui.collapsing("Hashed & Salted?", |ui|{
                ui.collapsing("Hashing", |ui|{
                ui.label(RichText::new("What is it?").strong());
                ui.label("Hashing is a process by which a piece of data like some text or a number is irreversibly transformed into another value.");
                ui.collapsing("Here are some examples", |ui|{
                    ui.label("You can imagine it like a machine crushing up a piece of art into molecule sized pieces and no matter what you do you won't get that art piece back but this machine is special it always crushes identical objects into an identical result and won't ever make the same result if the objects aren't exactly the same.");
                    ui.collapsing("The Mona lisa", |ui| {
                        let hashed_mona_lisa = Image::new(include_image!("../../assets/hashMonaLisa.png"));
                        ui.add(hashed_mona_lisa);
                    });
                    ui.collapsing("The number 17", |ui| {
                        let hashed_17 = Image::new(include_image!("../../assets/hash17.png"));
                        ui.add(hashed_17);
                    });
                    ui.collapsing("Or even the classic hello world", |ui| {
                        let hashed_hello_world = Image::new(include_image!("../../assets/hashHelloWorld.png"));
                        ui.add(hashed_hello_world);
                    });
                    ui.collapsing("Try it yourself", |ui| {
                        if ui.button("Hash").clicked(){
                            let mut hasher : Sha256 = sha2::digest::Digest::new();
                            
                            hasher.update(learn_data.hash_activity.clone()+"\n");
                            learn_data.hash_activity_result = hasher.finalize().as_bytes().into_iter().map(|&b| format!("{b:x}")).collect();
                        }
                        ui.label("Data");
                        ui.text_edit_singleline(&mut learn_data.hash_activity);
                        ui.label("Result");
                        ui.text_edit_singleline(&mut learn_data.hash_activity_result);
                    });
                // hashed_artwork.
                });
                // ui.label(RichText::new("It's a little like this:").strong());
                
                ui.label(RichText::new("Why's it important in cybersecurity?").strong());
                ui.label("Hashing is a fundamental part of cybersecurity because it allows people to compare one value to a secret value without knowings what the secret value is.");
                ui.label("So a company can store your password as a hash and then when you want to login they hash your password and compare it to the stored value and if they match you can login.");
                ui.label("And should they ever have a data breach the attacker won't be able to grab your password they'll only get your password which won't help them.");
                ui.label(RichText::new("But there's a problem.").strong());
                inline_text(ui, |ui| {
                    ui.label("Well what happens if someone gets a bunch of common passwords and they use the same algorithm to hash the passwords? Then they can check the hashes against the password hashes they've stolen and if they match they'll know what your password was. That's called a");
                    ui.hyperlink_to("rainbow table attack","https://en.wikipedia.org/wiki/Rainbow_table");
                    ui.label("and here's where salting comes in.");
                });
             });

                
                ui.separator();
                ui.collapsing("Salting", |ui|{
                    ui.label(RichText::new("What is it?").strong());
                    ui.label("Salting is a process by which you change a value just a little before you hash it and this completely changes the result.");
                    ui.label(RichText::new("So what's the big deal?").strong());
                    ui.label("Well if you have 2 hashes, one unsalted and the other salted you'll never be able to figure out how someone changed the salted one or if they originally very similar.");
                    ui.label(RichText::new("And how does that help us?").strong());
                    

                    ui.label("Well now the attackers we discussed in the hashing section can't use their special rainbow table because without the salt which we keep separate they'll never figure out how to compare the common passwords against our company's data.");

                    ui.collapsing("Try it yourself", |ui| {
                        if ui.button("Hash").clicked(){
                            let mut hasher : Sha256 = sha2::digest::Digest::new();
                            
                            hasher.update(learn_data.salted_hash_activity.clone()+learn_data.salt_value.as_str()+"\n");
                            learn_data.salted_hash_activity_result = hasher.finalize().as_bytes().into_iter().map(|&b| format!("{b:x}")).collect();
                        }
                        ui.label("Data");
                        ui.text_edit_singleline(&mut learn_data.salted_hash_activity);
                        ui.label("Salt");
                        ui.text_edit_singleline(&mut learn_data.salt_value);
                        ui.label("Result");
                        ui.text_edit_singleline(&mut learn_data.salted_hash_activity_result);
                    });

                    });

                    
            });
            ui.label(RichText::new("Tips").strong());
            ui.list("Password tips", |ui| {
                ui.li("Always choose unique passwords");
                ui.li("Avoid using a lazy password like \"password123\" ");
                ui.li("Consider using reputable password manager instead of manually entering passwords.");
            });
            ui.separator();
            
            
        });
        ui.collapsing("Man in the middle attacks", |ui| {
            ui.label(RichText::new("What are they?").strong());
            ui.label("Man in the middle attacks (MITM) are a form of hack that involves intercepting and modifying and / or eavesdropping on communications between 2 other parties potentially without either party realising.");
            ui.label(RichText::new("What is the point of these attacks?").strong());
            ui.label("MITM attacks can be used by hackers to steal sensitive and private information or to cause miscommunication to further their goals.");
            ui.collapsing("Examples", |ui| {
                ui.label(RichText::new("An example").strong());
                ui.label("A primary school kid Timmy wants to send a note to his crush Sarah and in naivety asks people in his class to pass it to her.");
                ui.label(RichText::new("But disaster strikes").strong());
                ui.label("The class bully takes his note and reads it to the class mortifying Timmy");
                ui.label(RichText::new("Or in alternate universe something worse happens").strong());
                ui.label("The class bully instead takes his note and switches it with an alternative that says \"Sarah has cooties\", a devestating insult for sure which causes Sarah to hate Timmy.");
                ui.separator();
                ui.label("While this example might sound juvenile it mirrors what can happen in the real world.");
                ui.label("In the real world companies might suffer financial or reputational damages if a message that is meant to be sent to another company is revealed or modified.")
            });
        });
        ui.collapsing("Social Engineering", |ui| {
            ui.label(RichText::new("What is it?").strong());
            ui.label("Social engineering is a manipulation technique that involves tricking victims into making security mistakes or divulging sensitive information often exploiting a variety of weaknesses in human decision making.");
            ui.separator();
            ui.label(RichText::new("So what can they exploit?").strong());
            ui.label("Heightened emotions: When you experience strong emotions you are more likely to take poorly calculated risks.");
            ui.label("Urgency: When you feel like you lack the time to properly consider a decision you are less likely to consider the negative consequences.");
            ui.label("Trust: To ensure that you give them sensitive details the attacker needs you to trust them.");
            ui.separator();

            ui.collapsing("Social engineering attacks", |ui| {
                ui.collapsing("Phishing", |ui| {
                    ui.label(RichText::new("What is it?").strong());
                    ui.label("Phishing is a technique that uses your trust in a legitimate organisation or people you know to trick you into revealing sensitive information.");
                    ui.label(RichText::new("An example").strong());
                    ui.label("A common phishing attack that people fall victim to is fraudulent bank email which tells them they need to login and gives them a link to do so. Unfortunately for the victim this link will actually take them to a copy of the legitimate site that will send their details to the attacker instead of logging them in.");
                    ui.label("However a clever attacker may also send their details to a legitimate page so that the person ends up on the legitimate bank website with no sign that they just had their details stolen. ");
                    ui.label("This is also a man in the middle attack like we mentioned before.");
                });

                ui.label(RichText::new("So how do you protect yourself?").strong());
                ui.label("Be wary of clicking on links or downloading files unless you are very sure they are from legimate sources");
                ui.label("If you are unsure about links or files in an email consider navigating from the official site rather than using the link in the email.");
                ui.label("Always take a moment to consider if the email or message looks too good to be true. Does it offer money or a prize for a competition that you didn't enter?");
            });
            
        });
       
    });
}