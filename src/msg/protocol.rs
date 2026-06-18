#[derive(Clone,Debug)]
pub struct Protocol;

// vogliamo che sia serializzabile a stringa quindi dobbiamo implementare trait AsRef
// str perchè stinga statica allocata sul binario stesso quando viene compilato

impl AsRef<str> for Protocol{
    fn as_ref(&self) -> &str{
        
    }
}