use super::Photo;

pub trait Platform {
     fn upload(&self, photo: &Photo);
}
