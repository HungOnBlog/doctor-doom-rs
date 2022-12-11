use doctor_doom_rs::doom;
fn main() {
    let doom_options = doom::DoomOptions::new_and_assign_from_env();
    println!("{:?}", doom_options);
}
