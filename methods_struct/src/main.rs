/* 
    Pour deriver display on doit le faire manuelement, 
    d'bord on inclus la lib qui contien le trait

*/
use std::fmt; // c'est ici que le trait Display reside.



/*
     certain comportement necessite de deriver des trait.
     ici on souhaite ajouter le debub flag pour ce struc :? et :#?
     tout les trait ne sont pas derivable par default.
     ici Debug est derivable pas Display
*/
#[derive(Debug)]
struct Object{
    width: u32,
    height: u32
}


/*
    ___________
    | METHODS |
    -----------
*/
impl Object {


    //methode "normale"
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn show (&self) {
        println!("{}*{} a pour aire : {} ", self.height, self.width, self.area());
    }   
}
/*
    _____________________
    | Related Functions |
    ---------------------
*/
impl Object {
        //related fonction on les appelle avec ::. On les appelle avec des mot clef spécifique en Rust. autour de 32 actuellement
        fn new (width:u32, height:u32) -> Object {
            Object{
                //Si les params et les proprietées de la struct on le même nom on peut simplement mettre le nom du param le binding se fait automatiquement en Rust.
                width,
                height
            }
        }
}

/*
    ____________________
    | Trait Derivation |
    --------------------
*/
impl fmt::Display for Object {
    /* 
        le fmt formatter est essentiellement utiliser pour ecrire dans la console ou dans des fichiers.
    */
    fn fmt(&self, f :&mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) et l'aire : {}", self.width, self.height , self.area())
    }

}



fn main() {
    let o = Object {
        width: 35,
        height: 55
    };

    let obj = Object::new(20, 40);

    o.show();
    obj.show();

    println!("{:?}", o);
    println!("{:#?}", obj);

    println!("{}", o);
    println!("{}", obj);

}
