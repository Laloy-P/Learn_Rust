fn main() {
    
    // exemple simplifié de Ownership/borrowing : 
    let s = String::from("String");
    let y = &s;
    println!("{}", s);

    println!("________MOVING________");
    exemple_moving();
    println!("_______________________");
    println!("________COPYING________");
    exemple_copying();
    println!("_________________________");
    println!("________BORROWING________");
    exemple_borrowing();
}

/*  ____________________
    |exemple de moving.|
    --------------------
    moving : on prend la proprietes (Ownership) de la variable et on ne le rend jamais.
    la variable est detruite hors contexte ensuite.

    remarque : il a lieu avec des donnees composees.
*/
fn exemple_moving(){

    let mut vec = Vec ::new();

    for i in 1..100{
        vec.push(i)
    }
    
    moving(vec);
    // on ne pourais pas reutiliser la variable vec println!("{}", vec[0])
    println!("finis");


}

/*
    lors d'un mov c'est la fonction qui reccupere la propriete de la donnee elle ne la rend jamais
*/
fn moving (v:Vec<i32>){
    println!("addition de valeur de v : {}", v[10] + v[70]);
}



/*  
    _____________________
    |exemple de copying |
    ---------------------
    copying : les varaibles a et b existe dans les deux scopes en meme temps, c'est stocke dans la stack. 
    
    remarque : il a lieu lorsque l'on est avec des donnee scalaire.

*/
fn exemple_copying(){

   let a = 32;
   let b = 42;

    cop(a,b);
  
    
    println!("on a utiliser les variables a : {} et b : {} ",a ,b );
}

fn cop (a:i32, b:i32){
   
    println!("addtion params a et b : {}", a + b);
}

/*  
    _______________________
    |exemple de borrowing |
    -----------------------
    Avoir plusieurs réferences sur la même valeur. Les références sont aussi des objets. Les références ressemble aux pointeurs. 
    Les reference mutables sont move les reference imutable sont copy. Quand une refrence est hors scope drop le borrowing s'arette
*/
fn re (v: Vec<i32>)->Vec<i32>{
    println!("{}",v[120] + v[111]);
    v
}

fn borrow_1(v: &Vec<i32>){
    println!("{}",(*v)[10] + (*v)[12]);
}
fn borrow_2(v: &Vec<i32>){
    println!("{}",v[10] + v[11]);
}



fn exemple_borrowing(){
    let mut v = Vec::new();


    for i in 1..1000{
        v.push(i)
    }

    v = re(v);
    println!("Possède toujours v: {} {}", v[0], v[1]);
    
    borrow_1(&v);
    println!("Possède toujours v: {} {}", v[0], v[1]);
    
    borrow_1(&v);
    println!("Possède toujours v: {} {}", v[0], v[1]);
}



