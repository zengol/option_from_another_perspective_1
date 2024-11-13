#[derive(Debug)]

enum Media {
    Book{title: String , autor : String},
    Movie{title:String , director: String},
    AudioBook{title: String},
    Serie{title: String},
   

}

// impl utilizando Pattern Matching with Enum. 

impl Media {
    fn description (&self) -> String {
          match self {
            Media::Book {title, autor} => {
                format!("Book: {} {}",title, autor)
            },
            Media::Movie {title,director} => {
                format!("Movie: {} {}", title, director)
            },
            Media::AudioBook {title} => {
                format!("AudioBook {} ", title)
            }
            Media::Serie {title} => {
                format!("Audiobook: {} ", title)
            }
        }
    } 

   
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    //función que me sirve para (1) en main.
    //el uso de Self es generalmente preferido en la comunidad de Rust por sus ventajas en términos de mantenibilidad, claridad y consistencia 
    //con las prácticas comunes del lenguaje. 
    fn new() -> Self {
        Catalog {items: vec![]}
    }
    
    //la fn tiene una ref a self muteable porque por cada llamado va a cambiar el valor de items.self que un vector como esta definido en la struct.
    //El &mut self indica que el método necesita una referencia mutable a la instancia de Catalog para poder 
    //modificar su contenido (en este caso, añadir un elemento al vector items).
    //En la mayoría de los casos, cuando se define un método dentro de un bloque impl, el primer parámetro debe ser self, &self, o &mut self
    //Cuando defines un método dentro de un bloque impl, el primer parámetro (que representa la instancia del objeto) debe ser self, &self, o &mut self.
    // En este caso, necesitas una referencia mutable a self.
    //función que me sirve para (2) en main.
    fn add(&mut self, media: Media){
        self.items.push(media);
    }

    /* 
    Préstamo inmutable (Immutable borrowing):
        Al devolver &Media, la función presta una referencia inmutable al elemento Media dentro del vector items,
         en lugar de transferir la propiedad o crear una copia.
        

    En resumen, usar &Media como tipo de retorno permite un acceso seguro y eficiente a los elementos del Catalog,
    manteniendo la integridad de la estructura de datos y adhiriéndose a los principios de ownership y borrowing de Rust. */

    fn get_by_index(&self, index: usize) -> &Media {
        &self.items[index]
    }



}
 fn main() {
    //creamos un enum y asignamos un valor en AudioBook.
    let audiobook = Media::AudioBook { 
        title: String::from("An audiobook")
     };
     let book = Media::Book {
        title: String::from("Good Book"),
        autor: String::from("Good Autor"),
     };
     let movie = Media::Movie {
        title: String::from("Bad movie"),
        director: String::from("Bad director")
     };
     let serie= Media::Serie { title: String::from("Good serie") };
     /* 

     println!("{}", audiobook.description());
     println!("{}", book.description());
     println!("{}", movie.description());
     println!("{}", serie.description());
 */
     // (1) inicializa  la impl
     let mut catalog = Catalog::new();

     // (2) aqui llamo a la fn add, lo que le envie por argumento la fn add lo va a agregar al vector self.items.
     catalog.add(audiobook);
     catalog.add(book);
     catalog.add(movie);
     catalog.add(serie);

     let item = catalog.get_by_index(0);
     println!("{:#?}", item);

 }
