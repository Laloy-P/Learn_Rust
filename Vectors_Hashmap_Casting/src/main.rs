use std::collections::HashMap;//import des hashmap
use std::fs::File;


fn main() {


/*
    ___________
    | Vectors |
    -----------
Vectors can grow and shrink are represented by 3 different pieces of data : 
pointer to the data the lenght-size and his capacity
quand sa taille depasse sa capaciter alors le vecteur est réalouer (pottentiellement couteux en ressources ?)
ils ne peuvent etre constituer qu'un seul type de donnée.println!neanmoins un vecteur peut stocker des enum 
ou des struct pour stocker des donnée un peu plus complexe ou different tyopte de donnée a l'interieur du vecteur
*/

let mut v = Vec::new();


let mut v2: Vec<i32>  = Vec::new();// permet de typer le veteur.

v2.push(10);

v.push(5);
v.push(6);
v.push(7);
v.push(8);

    for i in &v {
        println!("{}",i );
    }

println! ("{:#?} {} {}", &v, v.capacity(), v.len());
    v.push(10);
println! ("{:#?} {} {}", &v, v.capacity(), v.len());



v.pop();//sort la derniere valeur presente dans un vecteur renvoie un OPtion Value some none



/*
    ___________
    | Hashmap |
    -----------
Ils ne font pas parti de la librairie standard il y a donc un besoins de les importers.

lors de l'insertion les type doivent etre coherent on ajoute toujour les meme couple de donnée

*/

let mut hm = HashMap::new();

hm.insert("random", 12);
hm.insert("toto", 24);


/*
    __________________
    | if let binding |
    ------------------

    sucre syntaxique qui permet d'etre moins lourd qu'un match.
    ce n'est pas exhaustif comme un mach , il n'y a pas l'option default qui permet de couvrir les autres cas

*/
let s = Some('c');// le Some est important pour un if let dans cegt exemple si on l'enleve la on doir l'enlever dans le test l80
let s2 = Some('D');

//avec un match :
match s2 {
    Some(i) => println! ("{}", i),
    _=> println! ("rien")
}
 //avec un if let :println!


 if let Some(i) = s {
    println! ("{}", i)
 } else {
     {}
 }


/*
 __________________
 | While let |
 ------------------

 moins verbeux qu'une boucle for avec un match a l'interieur mais n'a pas lexauhstivite d'un match c'est sucre syntaxique
 */




/*

 le cast doit etre declaratif il n'y a pas de convertion implicite en Rust

*/

let f = 24.4321_f32;
let i = f as u8;
let c = i as char;

println!("{} __{}__ {}", f,i,c);

println!("{}", 255 as char);//attention a la limte de taille des type ici 256 ne passe pas

/*
error handling example : 

*/
let f = File::open("text.txt");

let f =  match f {
    Ok (file)=>file,
    Err (error) => {
        panic!("il y a eu un probleme  :::: {:?}", error)
    },
};


}
/*

 RESULT enum permet de gere les erreur en rust result permet de debugger la raison du faill


*/

enum Result<T, E>{
OK(T),
ERR(E),
}









