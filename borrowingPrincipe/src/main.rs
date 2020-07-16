fn main() {
let s = String::from("String");
let y = &s;
println!("{}", s);


//_________________
exemple_moving();
exemple_copying();
}

//exemple de moving on prend le controle et on ne le rend jamais. la variable est detruite hors contexte ensuite.
fn exemple_moving(){

    let mut vec = Vec ::new();

    for i in 1..100{
        vec.push(i)
    }
    
    moving(vec);
    // on ne pourais pas reutiliser la variable vec println!("{}", vec[0])
    println!("finis");


}

fn moving (v:Vec<i32>){
    println!("addition de valeur de v : {}", v[10] + v[70]);
}

//_______________________________

//exemple de copying les varaible a et b existe dans les deux scope en meme temps, pas genant nous sommes dans la stack avec dce type de donées scalaire.
fn exemple_copying(){

   let a = 32;
   let b = 42;

    cop(a,b);
  
    
    println!("on a utiliser les varaible a : {} et b : {} ",a ,b );
}

fn cop (a:i32, b:i32){
   
    println!("addtion params a et b : {}", a + b);
}

//_______________________________

//exemple de copying